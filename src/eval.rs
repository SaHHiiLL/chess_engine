use std::str::FromStr;

use crate::{
    engine::GamePhases, BoardMaterial, MaterialSumExt, KING_MIDDLE_BLACK, KING_MIDDLE_WHITE,
};
use chess::{Board, ChessMove, Color, MoveGen, Piece, Square};

use crate::{
    BISHOP_VALUE_PER_SQUARE_BLACK, BISHOP_VALUE_PER_SQUARE_WHITE, KNIGHT_VALUE_PER_SQUARE_BLACK,
    KNIGHT_VALUE_PER_SQUARE_WHITE, PAWN_VALUE_PER_SQUARE_BLACK, PAWN_VALUE_PER_SQUARE_WHITE,
    QUEEN_VALUE_PER_SQUARE_BLACK, QUEEN_VALUE_PER_SQUARE_WHITE, ROOK_VALUE_PER_SQUARE_BLACK,
    ROOK_VALUE_PER_SQUARE_WHITE,
};

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum EvalFlags {
    PieceCount,
    Mobility,
    KingSafety,
    PieceSquare,
    FavourBishopPairs,
}

pub struct Evaluation<'a> {
    engine_side: &'a Board,
    flags: &'a [EvalFlags],
    phases: GamePhases,
}

impl<'a> Evaluation<'a> {
    pub fn new(engine_side: &'a Board) -> Self {
        Self {
            engine_side,
            flags: &[
                EvalFlags::KingSafety,
                EvalFlags::Mobility,
                EvalFlags::PieceCount,
                EvalFlags::PieceSquare,
                EvalFlags::FavourBishopPairs,
            ],
            phases: Default::default(),
        }
    }

    pub fn is_piece_on_original_pos(&self, piece: &Piece, square: &Square, color: &Color) -> bool {
        let default_squares: Vec<Square> = match piece {
            Piece::Knight => match color {
                Color::White => vec![
                    Square::from_str("b1").expect("IS A CORRECT SQUARE"),
                    Square::from_str("g1").expect("IS A CORRECT SQUARE"),
                ],
                Color::Black => vec![
                    Square::from_str("b8").expect("IS A CORRECT SQUARE"),
                    Square::from_str("g8").expect("IS A CORRECT SQUARE"),
                ],
            },
            Piece::Bishop => match color {
                Color::White => vec![
                    Square::from_str("c1").expect("IS A CORRECT SQUARE"),
                    Square::from_str("f1").expect("IS A CORRECT SQUARE"),
                ],
                Color::Black => vec![
                    Square::from_str("c8").expect("IS A CORRECT SQUARE"),
                    Square::from_str("f8").expect("IS A CORRECT SQUARE"),
                ],
            },

            Piece::Rook => match color {
                Color::White => vec![
                    Square::from_str("a1").expect("IS A CORRECT SQUARE"),
                    Square::from_str("h1").expect("IS A CORRECT SQUARE"),
                ],
                Color::Black => vec![
                    Square::from_str("a8").expect("IS A CORRECT SQUARE"),
                    Square::from_str("h8").expect("IS A CORRECT SQUARE"),
                ],
            },
            Piece::King => match color {
                Color::White => vec![Square::from_str("e1").expect("IS A CORRECT SQUARE")],
                Color::Black => vec![Square::from_str("e8").expect("IS A CORRECT SQUARE")],
            },
            _ => return false,
        };

        for sq in default_squares.iter() {
            if sq.eq(square) {
                return true;
            }
        }
        false
    }

    /// adds a bonus for having a bishop pair of the engine side
    fn favour_bishop_pair(&self, board: &Board) -> isize {
        let b_p = board.pieces(Piece::Bishop);
        if b_p.0 == 0 {
            return 0;
        }
        let mut res = 0;
        let color_bp = match board.side_to_move() {
            Color::White => board.color_combined(Color::White) & b_p,
            Color::Black => board.color_combined(Color::Black) & b_p,
        };

        if (board.side_to_move() == self.engine_side.side_to_move()) && color_bp.count() == 2 {
            res += 10;
        } else {
            res -= 10;
        }
        res
    }

    pub fn eval_board(&self, board: &Board, board_history: &[u64]) -> isize {
        // if the position has been reached before at least 3 times it will be draw by three-fold
        // repetition
        let repeat_board = board_history
            .iter()
            .filter(|d| **d == board.get_hash())
            .count();

        if repeat_board > 2 {
            println!("REPEAT BOARD -> {repeat_board}");
            return 0;
        }

        let mut value_based_on_pos: isize = 0;
        for x in 0..64 {
            let square = unsafe { Square::new(x) };
            let piece = board.piece_on(square);
            let color = board.color_on(square);

            if let (Some(piece), Some(color)) = (piece, color) {
                let piece_value = match piece {
                    chess::Piece::Pawn => match color {
                        Color::White => PAWN_VALUE_PER_SQUARE_WHITE[x as usize],
                        Color::Black => PAWN_VALUE_PER_SQUARE_BLACK[x as usize],
                    },
                    chess::Piece::Knight => match color {
                        Color::White => KNIGHT_VALUE_PER_SQUARE_WHITE[x as usize],
                        Color::Black => KNIGHT_VALUE_PER_SQUARE_BLACK[x as usize],
                    },
                    chess::Piece::Bishop => match color {
                        Color::White => BISHOP_VALUE_PER_SQUARE_WHITE[x as usize],
                        Color::Black => BISHOP_VALUE_PER_SQUARE_BLACK[x as usize],
                    },
                    // chess::Piece::King => match color {
                    //     Color::White => KING_MIDDLE_WHITE[x as usize],
                    //     Color::Black => KING_MIDDLE_BLACK[x as usize],
                    // },
                    chess::Piece::Rook => match color {
                        Color::White => ROOK_VALUE_PER_SQUARE_WHITE[x as usize],
                        Color::Black => ROOK_VALUE_PER_SQUARE_BLACK[x as usize],
                    },
                    // chess::Piece::Queen => match color {
                    //     Color::White => QUEEN_VALUE_PER_SQUARE_WHITE[x as usize],
                    //     Color::Black => QUEEN_VALUE_PER_SQUARE_BLACK[x as usize],
                    // },
                    _ => 0,
                };

                value_based_on_pos += piece_value;

                if self.is_piece_on_original_pos(&piece, &square, &color) {
                    // decrease the evals - to encourage to move pieces forward
                    value_based_on_pos = value_based_on_pos.saturating_sub(5);
                }
            }
        }

        let mat_val = match board.status() {
            chess::BoardStatus::Ongoing => {
                let board_sum = board.material_sum_bitboard();
                let eval = match board.side_to_move() {
                    Color::White => board_sum.white as isize - board_sum.black as isize,
                    Color::Black => board_sum.black as isize - board_sum.white as isize,
                };

                if board.side_to_move() != self.engine_side.side_to_move() {
                    -eval
                } else {
                    eval
                }
            }
            chess::BoardStatus::Stalemate => 0,
            chess::BoardStatus::Checkmate => {
                if board.side_to_move() == self.engine_side.side_to_move() {
                    -isize::MAX
                } else {
                    isize::MAX
                }
            }
        };
        mat_val
            .saturating_add(value_based_on_pos)
            .saturating_add(self.favour_bishop_pair(board))
    }

    pub fn eval_mobility(&self, moves: &[ChessMove]) -> isize {
        moves.len().saturating_mul(2).try_into().unwrap()
    }
}
