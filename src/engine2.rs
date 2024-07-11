use std::{collections::HashMap, str::FromStr};

use chess::{Board, ChessMove, Color, MoveGen, Square};

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
        -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15,
        10, 0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15,
        15, 10, 5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
    ];
    static ref KNIGHT_VALUE_PER_SQUARE_BLACK: Vec<isize> = KNIGHT_VALUE_PER_SQUARE_WHITE
        .iter()
        .copied()
        .rev()
        .collect();
    static ref PAWN_VALUD_PER_SQUARE_WHITE: Vec<isize> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, -20, -20, 10, 10, 5, 5, -5, -10, 0, 0, -10, -5, 5, 0, 0,
        0, 20, 20, 0, 0, 0, 5, 5, 10, 25, 25, 10, 5, 5, 10, 10, 20, 30, 30, 20, 10, 10, 50, 50, 50,
        50, 50, 50, 50, 50, 0, 0, 0, 0, 0, 0, 0, 0
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
}

struct BoardMaterial {
    white: u32,
    black: u32,
}

trait MaterialSum {
    fn material_sum(&self) -> BoardMaterial;
}

impl MaterialSum for chess::Board {
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
        }
    }

    fn gen_legal_moves(&self, board: &Board) -> Vec<ChessMove> {
        MoveGen::new_legal(board).collect()
    }

    pub fn get_best_mov(&self) -> Option<ChessMove> {
        self.best_move
    }

    pub fn play_best_move(&mut self) {
        if let Some(mov) = self.best_move {
            self.board = self.board.make_move_new(mov);
        };
    }

    pub fn play_moves(&mut self, moves: Vec<ChessMove>) {
        for m in moves.iter() {
            self.board = self.board.make_move_new(*m);
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
        let moves = self.gen_legal_moves(board);

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
            return self.eval_board(board);
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

    pub fn eval_board(&self, board: &Board) -> isize {
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
                    _ => 0,
                };

                value_based_on_pos += piece_value;
            }
        }

        let mat_val = match board.status() {
            chess::BoardStatus::Ongoing => {
                let board_sum = board.material_sum();
                let eval = match board.side_to_move() {
                    Color::White => board_sum.white as isize - board_sum.black as isize,
                    Color::Black => board_sum.black as isize - board_sum.white as isize,
                };

                if board.pinned().0 != 0 {
                    eval.saturating_add(5);
                }

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

impl FromStr for Engine {
    type Err = chess::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board = Board::from_str(s)?;
        Ok(Self {
            board,
            best_move: None,
            side_playing: board.side_to_move(),
        })
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
}
