use std::{cell::RefCell, rc::Rc, str::FromStr};

use crate::{
    game_state::GameState, BoardMaterial, MaterialSumExt, PieceFromColor, KING_MIDDLE_BLACK,
    KING_MIDDLE_WHITE,
};
use chess::{BitBoard, Board, ChessMove, Color, MoveGen, Piece, Square};

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
    game_state: &'a Rc<RefCell<GameState>>,
}

//TODO: make a game result enum for checkmate that has move count for checkmate

impl<'a> Evaluation<'a> {
    pub fn new(engine_side: &'a Board, game_state: &'a Rc<RefCell<GameState>>) -> Self {
        Self {
            engine_side,
            game_state,
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

    fn pass_pawn(&self, board: &Board, square: Square, color: Color) -> isize {
        let pawn_mask = match color {
            Color::White => self.pass_pawn_bitmask_white(square),
            Color::Black => self.pass_pawn_bitmask_black(square),
        };

        if (pawn_mask & board.combined()).0 == 0 {
            let rank_bonus = match color {
                Color::White => &[0, 10, 30, 40, 50, 60, 90, 0],
                Color::Black => &[0, 90, 60, 50, 40, 30, 10, 0],
            };
            15 + rank_bonus[square.get_rank().to_index()]
        } else {
            0
        }
    }

    /// returns a bitboard with only just the single file being turned on
    fn file_bitboard(&self, file_idx: usize) -> BitBoard {
        BitBoard(72340172838076673 << file_idx)
    }

    /// creates a bitmask to check if a pawn can be considered as passed pawned or not
    fn pass_pawn_bitmask_black(&self, square: Square) -> BitBoard {
        let a_file: u64 = 72340172838076673;
        let file_idx = square.get_file().to_index() as u64;

        let pawn_file = a_file << file_idx;
        let left_file_idx: u64 = a_file << (file_idx.saturating_sub(1)).max(0);
        let right_file_idx: u64 = a_file << (file_idx + 1).min(7);
        let pawn_file = pawn_file | left_file_idx | right_file_idx;

        let shift_rank = pawn_file << 8 * (square.get_rank().to_index() as u64 + 1);
        BitBoard(shift_rank)
    }

    fn pass_pawn_bitmask_white(&self, square: Square) -> BitBoard {
        let a_file: u64 = 72340172838076673;
        let file_idx = square.get_file().to_index() as u64;

        let pawn_file = a_file << file_idx;
        let left_file_idx: u64 = a_file << (file_idx.saturating_sub(1)).max(0);
        let right_file_idx: u64 = a_file << (file_idx + 1).min(7);
        let pawn_file = pawn_file | left_file_idx | right_file_idx;

        let shift_rank = pawn_file >> 8 * (square.get_rank().to_index() as u64 + 1);
        BitBoard(shift_rank)
    }

    /// adds a bonus for having a bishop pair of the engine side
    fn favour_bishop_pair(&self, board: &Board) -> isize {
        let b_p = board.pieces(Piece::Bishop);
        if b_p.0 == 0 {
            return 0;
        }
        let color_bp = match board.side_to_move() {
            Color::White => board.color_combined(Color::White) & b_p,
            Color::Black => board.color_combined(Color::Black) & b_p,
        };

        if (board.side_to_move() == self.engine_side.side_to_move()) && color_bp.count() == 2 {
            10
        } else {
            -10
        }
    }

    pub fn eval_board(&mut self, board: &Board, board_history: &[u64]) -> isize {
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
                    chess::Piece::Pawn => {
                        let f = self.pass_pawn(board, square, color);
                        value_based_on_pos += f;
                        match color {
                            Color::White => PAWN_VALUE_PER_SQUARE_WHITE[x as usize],
                            Color::Black => PAWN_VALUE_PER_SQUARE_BLACK[x as usize],
                        }
                    }
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
                    chess::Piece::Queen => match color {
                        Color::White => QUEEN_VALUE_PER_SQUARE_WHITE[x as usize],
                        Color::Black => QUEEN_VALUE_PER_SQUARE_BLACK[x as usize],
                    },
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

                // game state is updated
                let mut game_state = self.game_state.as_ref().borrow_mut();
                game_state.update_game_phase(board_sum, board);

                let mut eval = eval
                    .saturating_sub(self.favour_bishop_pair(board))
                    .saturating_add(value_based_on_pos);

                if game_state.game_phases().is_end() {
                    // check if king is in check -- give incentive
                    if board.checkers() != &BitBoard(0)
                        && self.engine_side.side_to_move() != board.side_to_move()
                    {
                        eval = eval.saturating_add(20);
                    }

                    eval = eval.saturating_add(self.push_enemy_king_to_edge(board));
                }

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
    }

    pub fn eval_mobility(&self, moves: &[ChessMove]) -> isize {
        moves.len().saturating_mul(2).try_into().unwrap()
    }

    /// --- END GAME SPECIFIC --- ///

    fn rook_on_same_rank(&self, board: &Board) -> isize {
        todo!()
    }

    /// return a positive value if king is in edge of the board or returns a negative value if not
    fn push_enemy_king_to_edge(&self, board: &Board) -> isize {
        let edge_bitboard = BitBoard(0xff818181818181ff);
        let enemy_color = match self.engine_side.side_to_move() {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        let enemy_king = board.pieces_color(Piece::King, enemy_color);
        assert_eq!(enemy_king.0.count_ones(), 1);
        let enemy_king_square = enemy_king.to_square();
        KING_EDGE[enemy_king_square.to_index()]
    }
}

pub const KING_EDGE: &[isize; 64] = &[
    16, 16, 16, 16, 16, 16, 16, 16, 16, 14, 14, 14, 14, 14, 14, 16, 16, 14, -10, -10, -10, -10, 14,
    16, 16, 14, -10, -20, -20, -10, 14, 16, 16, 14, -10, -20, -20, -20, 14, 16, 16, 14, -10, -10,
    -10, -10, 14, 16, 16, 14, 14, 14, 14, 14, 14, 16, 16, 14, 14, 14, 14, 14, 14, 16,
];
