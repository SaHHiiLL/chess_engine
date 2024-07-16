use std::{cmp::Ordering, collections::HashMap, ptr::eq, str::FromStr};

use chess::{Board, ChessMove, Color, MoveGen, Piece, Square};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

lazy_static::lazy_static! {
    static ref PIECE_VALUE_MAP: HashMap<chess::Piece, u32> = {
        let mut map = HashMap::new();
        map.insert(chess::Piece::King, 20_000);
        map.insert(chess::Piece::Queen, 900);
        map.insert(chess::Piece::Rook, 500);
        map.insert(chess::Piece::Knight, 300);
        map.insert(chess::Piece::Bishop, 330);
        map.insert(chess::Piece::Pawn, 100);
        map
    };
    static ref INITIAL_VALUE: u16 = 23_900;
    static ref CHECKMATE_VALUE: isize = 23_900 * 2;
    static ref KNIGHT_VALUE_PER_SQUARE_WHITE: Vec<isize> = vec![
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -20,-30,-30,-40,-40,-30,-30,-20,
        -10,-20,-20,-20,-20,-20,-20,-10,
         20, 20,  0,  0,  0,  0, 20, 20,
         20, 30, 10,  0,  0, 10, 30, 20
    ];
    static ref KNIGHT_VALUE_PER_SQUARE_BLACK: Vec<isize> = KNIGHT_VALUE_PER_SQUARE_WHITE
        .iter()
        .copied()
        .rev()
        .collect();
    static ref PAWN_VALUD_PER_SQUARE_WHITE: Vec<isize> = vec![
         0,  0,  0,  0,  0,  0,  0,  0,
        50, 50, 50, 50, 50, 50, 50, 50,
        10, 10, 20, 30, 30, 20, 10, 10,
         5,  5, 10, 25, 25, 10,  5,  5,
         0,  0,  0, 20, 20,  0,  0,  0,
         5, -5,-10,  0,  0,-10, -5,  5,
         5, 10, 10,-20,-20, 10, 10,  5,
         0,  0,  0,  0,  0,  0,  0,  0
    ];
    static ref PAWN_VALUD_PER_SQUARE_BLACK: Vec<isize> =
        PAWN_VALUD_PER_SQUARE_WHITE.iter().copied().rev().collect();

    static ref BISHOP_VALUE_PER_SQUARE_WHITE: Vec<isize> = vec![
        -20,-10,-10,-10,-10,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5, 10, 10,  5,  0,-10,
        -10,  5,  5, 10, 10,  5,  5,-10,
        -10,  0, 10, 10, 10, 10,  0,-10,
        -10, 10, 10, 10, 10, 10, 10,-10,
        -10,  20,  0,  0,  0,  0,  20,-10,
        -20,-10,-10,-10,-10,-10,-10,-20,
    ];
    static ref BISHOP_VALUE_PER_SQUARE_BLACK: Vec<isize> = BISHOP_VALUE_PER_SQUARE_WHITE.iter().copied().rev().collect();


    static ref KING_VALUE_PER_SQUARE_MIDDLE_GAME_WHITE: Vec<isize> = vec![
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -20,-30,-30,-40,-40,-30,-30,-20,
        -10,-20,-20,-20,-20,-20,-20,-10,
         20, 20,  0,  0,  0,  0, 20, 20,
         20, 30, 10,  0,  0, 10, 30, 20
    ];

    static ref KING_VALUE_PER_SQUARE_MIDDLE_GAME_BLACK: Vec<isize> = KING_VALUE_PER_SQUARE_MIDDLE_GAME_WHITE
        .iter().copied().rev().collect();

    static ref QUEEN_VALUE_PER_SQUARE_WHITE: Vec<isize> = vec![
        -20,-10,-10, -5, -5,-10,-10,-20,
-10,  0,  0,  0,  0,  0,  0,-10,
-10,  0,  5,  5,  5,  5,  0,-10,
 -5,  0,  5,  5,  5,  5,  0, -5,
  0,  0,  5,  5,  5,  5,  0, -5,
-10,  5,  5,  5,  5,  5,  0,-10,
-10,  0,  5,  0,  0,  0,  0,-10,
-20,-10,-10, -5, -5,-10,-10,-20
    ];

    static ref QUEEN_VALUE_PER_SQUARE_BLACK: Vec<isize> = QUEEN_VALUE_PER_SQUARE_WHITE.iter().copied().rev().collect();

    static ref ROOK_VALUE_PER_SQUARE_WHITE: Vec<isize> = vec![
          0,  0,  0,  0,  0,  0,  0,  0,
  5, 10, 10, 10, 10, 10, 10,  5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
  0,  0,  0,  5,  5,  0,  0,  0
    ];

    static ref ROOK_VALUE_PER_SQUARE_BLACK: Vec<isize> = ROOK_VALUE_PER_SQUARE_WHITE.iter().copied().rev().collect();
}

struct BoardMaterial {
    white: u32,
    black: u32,
}

enum GamePhases {
    Opening,
    MiddleGame,
    EndGame,
}

impl GamePhases {
    fn from_material_count(white_material: usize, black_material: usize) -> Self {
        todo!()
    }
}

trait IsCaptureMoveExt {
    fn is_capture_move(&self, chess_move: ChessMove) -> bool;
}

impl IsCaptureMoveExt for chess::Board {
    fn is_capture_move(&self, chess_move: ChessMove) -> bool {
        self.piece_on(chess_move.get_dest()).is_some()
    }
}

trait MaterialSumExt {
    fn material_sum(&self) -> BoardMaterial;
}

impl MaterialSumExt for chess::Board {
    fn material_sum(&self) -> BoardMaterial {
        let mut mat = BoardMaterial { white: 0, black: 0 };
        for sq in 0..64 {
            // SAFETY: squares are only created from 0 to 64 (not including 64)
            let sq = unsafe { Square::new(sq) };
            let piece_type = self.piece_on(sq);
            let color = self.color_on(sq);

            if let (Some(piece_type), Some(color)) = (piece_type, color) {
                let p = PIECE_VALUE_MAP.get(&piece_type).expect("You idiot");
                match color {
                    chess::Color::White => mat.white += *p,
                    chess::Color::Black => mat.black += *p,
                };
            }
        }
        mat
    }
}

pub struct Engine {
    board: Board,
    best_move: Option<ChessMove>,
    side_playing: chess::Color,
    board_history: Vec<u64>,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Engine {
    type Err = chess::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board = Board::from_str(s)?;
        Ok(Self {
            board,
            best_move: None,
            side_playing: board.side_to_move(),
            board_history: vec![],
        })
    }
}

impl Engine {
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn new() -> Self {
        Self {
            board: Board::default(),
            best_move: None,
            side_playing: chess::Color::White,
            board_history: vec![],
        }
    }

    fn gen_legal_moves(&self, board: &Board) -> Vec<ChessMove> {
        MoveGen::new_legal(board).collect()
    }

    /// Sorts moves based on if the move captures a piece or does a promotion
    /// if a move is a capture or promotion it will be send higher in the list
    /// this will help the `alpha-beta` pruning
    fn sort_moves_in_place(&self, board: &Board, moves: &mut [ChessMove]) {
        moves.sort_by(|d: &ChessMove, other: &ChessMove| {
            let square: Square = d.get_dest();
            let piece = board.piece_on(square);

            let square_other: Square = other.get_dest();
            let piece_other = board.piece_on(square_other);

            if piece.is_some() && piece_other.is_some() {
                return Ordering::Equal;
            }

            if piece.is_some() {
                return Ordering::Less;
            }

            if piece_other.is_some() {
                return Ordering::Greater;
            }

            if d.get_promotion().is_some() {
                return Ordering::Less;
            }
            if other.get_promotion().is_some() {
                return Ordering::Greater;
            }

            Ordering::Equal
        });
    }

    pub fn get_best_mov(&self) -> Option<ChessMove> {
        self.best_move
    }

    pub fn play_best_move(&mut self) {
        if let Some(mov) = self.best_move {
            self.board = self.board.make_move_new(mov);
            self.board_history.clear();
        };
    }

    pub fn play_moves(&mut self, moves: Vec<ChessMove>) {
        for m in moves.iter() {
            let board = self.board.make_move_new(*m);
            self.board = board;
            self.board_history.push(board.get_hash());
        }
        self.side_playing = self.board.side_to_move();
    }

    pub fn search_slow(&mut self, depth: usize) -> isize {
        let legal_moves = self.gen_legal_moves(&self.board);
        let mut best_eval = -isize::MAX;

        for m in legal_moves.iter() {
            // make the move
            let next_board = self.board.make_move_new(*m);
            let next_eval = self.search_minimax(depth, &next_board, false);

            if next_eval > best_eval || self.best_move.is_none() {
                best_eval = next_eval;
                let _ = self.best_move.insert(*m);
            }
        }
        best_eval
    }

    pub fn search(&mut self, depth: usize) -> isize {
        let legal_moves = self.gen_legal_moves(&self.board);
        let mut best_eval = -isize::MAX;

        for m in legal_moves.iter() {
            // make the move
            let next_board = self.board.make_move_new(*m);
            let next_eval =
                self.search_alpha_beta(depth, &next_board, -isize::MAX, isize::MAX, false);

            if next_eval > best_eval || self.best_move.is_none() {
                best_eval = next_eval;
                let _ = self.best_move.insert(*m);
            }
        }
        best_eval
    }

    fn search_alpha_beta(
        &mut self,
        depth: usize,
        board: &Board,
        mut alpha: isize,
        mut beta: isize,
        is_maximizing: bool,
    ) -> isize {
        if depth == 0 {
            return self.eval_board(board);
        }

        let mut best_eval = if is_maximizing {
            -isize::MAX
        } else {
            isize::MAX
        };
        // Move Ordering based on -- if a piece can be captured from the move it can be a good move
        // thus should be looked before
        let mut moves = self.gen_legal_moves(board);
        self.sort_moves_in_place(board, &mut moves);
        let moves = moves;
        if moves.is_empty() {
            return self.eval_board(board);
        }

        for m in moves.iter() {
            // make the move
            let next_board = board.make_move_new(*m);
            let eval = self.search_alpha_beta(depth - 1, &next_board, alpha, beta, !is_maximizing);

            if is_maximizing {
                best_eval = best_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha {
                    break;
                }
            } else {
                best_eval = best_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break;
                }
            }
        }
        best_eval
    }

    fn search_minimax(&mut self, depth: usize, board: &Board, is_maximizing: bool) -> isize {
        if depth == 0 {
            if board.checkers().0 != 0 {
                return self.search_minimax(depth + 1, board, !is_maximizing);
            } else {
                return self.eval(board);
            }
        }

        let mut best_eval = if is_maximizing {
            -isize::MAX
        } else {
            isize::MAX
        };

        let moves = self.gen_legal_moves(board);

        if moves.is_empty() {
            return self.eval_board(board);
        }

        for m in moves.iter() {
            // make the move
            let next_board = board.make_move_new(*m);
            let next_eval = self.search_minimax(depth - 1, &next_board, !is_maximizing);

            if is_maximizing {
                best_eval = best_eval.max(next_eval);
            } else {
                best_eval = best_eval.min(next_eval);
            }
        }
        best_eval
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
            // Piece::Queen => match color {
            //     Color::White => vec![Square::from_str("d1").expect("IS A CORRECT SQUARE")],
            //     Color::Black => vec![Square::from_str("d8").expect("IS A CORRECT SQUARE")],
            // },
            _ => return false,
        };

        for sq in default_squares.iter() {
            if sq.eq(square) {
                return true;
            }
        }
        false
    }

    fn rocks_on_same_rank_or_file(&self, board: &Board) -> isize {
        todo!()
    }

    fn eval_mobility(&self, moves: &[ChessMove]) -> isize {
        moves.len().saturating_mul(2).try_into().unwrap()
    }

    pub fn eval(&self, board: &Board) -> isize {
        let moves = self.gen_legal_moves(board);
        self.eval_board(board)
            .saturating_add(self.eval_mobility(&moves))
    }

    pub fn eval_board(&self, board: &Board) -> isize {
        // if the position has been reached before at least 3 times it will be draw by three-fold
        // repetition
        let repeat_board = self
            .board_history
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
                        Color::White => PAWN_VALUD_PER_SQUARE_WHITE[x as usize],
                        Color::Black => PAWN_VALUD_PER_SQUARE_BLACK[x as usize],
                    },
                    chess::Piece::Knight => match color {
                        Color::White => KNIGHT_VALUE_PER_SQUARE_WHITE[x as usize],
                        Color::Black => KNIGHT_VALUE_PER_SQUARE_BLACK[x as usize],
                    },
                    chess::Piece::Bishop => match color {
                        Color::White => BISHOP_VALUE_PER_SQUARE_WHITE[x as usize],
                        Color::Black => BISHOP_VALUE_PER_SQUARE_BLACK[x as usize],
                    },
                    chess::Piece::King => match color {
                        Color::White => KING_VALUE_PER_SQUARE_MIDDLE_GAME_WHITE[x as usize],
                        Color::Black => KING_VALUE_PER_SQUARE_MIDDLE_GAME_BLACK[x as usize],
                    },
                    chess::Piece::Rook => match color {
                        Color::White => ROOK_VALUE_PER_SQUARE_WHITE[x as usize],
                        Color::Black => ROOK_VALUE_PER_SQUARE_BLACK[x as usize],
                    },
                    chess::Piece::Queen => match color {
                        Color::White => QUEEN_VALUE_PER_SQUARE_WHITE[x as usize],
                        Color::Black => QUEEN_VALUE_PER_SQUARE_BLACK[x as usize],
                    },
                };

                value_based_on_pos += piece_value;

                if self.is_piece_on_original_pos(&piece, &square, &color) {
                    // decrease the evals - to encourage it to move pieces forward
                    value_based_on_pos = value_based_on_pos.saturating_sub(5);
                }
            }
        }

        let mat_val = match board.status() {
            chess::BoardStatus::Ongoing => {
                let board_sum = board.material_sum();
                let eval = match board.side_to_move() {
                    Color::White => board_sum.white as isize - board_sum.black as isize,
                    Color::Black => board_sum.black as isize - board_sum.white as isize,
                };

                if board.side_to_move() != self.board.side_to_move() {
                    -eval
                } else {
                    eval
                }
            }
            chess::BoardStatus::Stalemate => 0,
            chess::BoardStatus::Checkmate => {
                if board.side_to_move() == self.board.side_to_move() {
                    -isize::MAX
                } else {
                    isize::MAX
                }
            }
        };
        mat_val.saturating_add(value_based_on_pos)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::Engine;

    #[test]
    fn best_move_checkmate() {
        let mut engine =
            Engine::from_str("r1b1kb2/pppp1p1p/2n1p2n/8/3q2r1/8/PPPPKPP1/RNBQ1BNR b q - 0 11")
                .expect("IDIOT");
        let eval = engine.search(1);
        assert_eq!(eval, isize::MAX);
        assert_eq!(engine.best_move.unwrap().to_string(), "d4e4");
    }

    #[test]
    fn test_best_move_capture_queen() {
        let mut engine =
            Engine::from_str("rn2k1nr/ppp2ppp/8/3pp3/8/P1P3qb/1PQPPP2/RNB1KB2 w Qkq - 0 8")
                .unwrap();
        let eval = engine.search(3);
        assert_eq!(engine.get_best_mov().unwrap().to_string().as_str(), "f2g3");
    }

    #[test]
    fn eval_board_black() {
        let engine = Engine::from_str("8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1").unwrap();
        let eval = engine.eval_board(&engine.board);
        assert_eq!(eval, 1100);
    }

    #[test]
    fn eval_board_white() {
        let engine = Engine::from_str("8/8/1P2K3/8/2n5/1q6/8/5k2 w - - 0 1").unwrap();
        let eval = engine.eval_board(&engine.board);
        assert_eq!(eval, -1100);
    }

    #[test]
    fn best_move_capture_free_pawn() {
        let mut engine =
            Engine::from_str("1nbqkbnr/1ppppppp/8/8/r1PP4/8/PP2PPPP/R1BQKBNR b KQk - 0 1").unwrap();
        let eval = engine.search(1);
        assert_eq!(engine.get_best_mov().unwrap().to_string().as_str(), "a4c4");
    }

    #[test]
    fn test_move_repetition() {
        let mut engine = Engine::new();
    }
}
