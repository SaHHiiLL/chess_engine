use std::{collections::HashMap, str::FromStr};

use chess::{Board, ChessMove, Color, MoveGen, Square};

lazy_static::lazy_static! {
    static ref PIECE_VALUE_MAP: HashMap<chess::Piece, u32> = {
        let mut map = HashMap::new();
        map.insert(chess::Piece::King, 20_000);
        map.insert(chess::Piece::Queen, 900);
        map.insert(chess::Piece::Rook, 500);
        map.insert(chess::Piece::Knight, 300);
        map.insert(chess::Piece::Bishop, 300);
        map.insert(chess::Piece::Pawn, 100);
        map
    };
    static ref INITIAL_VALUE: u16 = 23_900;
    static ref KNIGHT_VALUE_PER_SQUARE_WHITE: Vec<i32> = vec![
        -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15,
        10, 0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15,
        15, 10, 5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
    ];
    static ref KNIGHT_VALUE_PER_SQUARE_BLACK: Vec<i32> = KNIGHT_VALUE_PER_SQUARE_WHITE
        .iter()
        .copied()
        .rev()
        .collect::<Vec<i32>>();
    static ref PAWN_VALUD_PER_SQUARE_WHITE: Vec<i32> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, -20, -20, 10, 10, 5, 5, -5, -10, 0, 0, -10, -5, 5, 0, 0,
        0, 20, 20, 0, 0, 0, 5, 5, 10, 25, 25, 10, 5, 5, 10, 10, 20, 30, 30, 20, 10, 10, 50, 50, 50,
        50, 50, 50, 50, 50, 0, 0, 0, 0, 0, 0, 0, 0
    ];
    static ref PAWN_VALUD_PER_SQUARE_BLACK: Vec<i32> =
        PAWN_VALUD_PER_SQUARE_WHITE.iter().copied().rev().collect();
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
    side_playing_for: Color,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            board: Board::default(),
            best_move: None,
            side_playing_for: Color::White,
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
    }

    pub fn search(&mut self, depth: usize) -> isize {
        let legal_moves = self.gen_legal_moves(&self.board);
        let mut best_eval = isize::MIN;

        for m in legal_moves.iter() {
            // make the move
            let next_board = self.board.make_move_new(*m);
            let next_eval = self.search_further(depth, &next_board);

            if next_eval == isize::MAX {
                best_eval = next_eval;
                let _ = self.best_move.insert(*m);
                break;
            }

            if next_eval > best_eval {
                best_eval = next_eval;
                let _ = self.best_move.insert(*m);
            }
        }
        best_eval
    }

    fn search_further(&mut self, depth: usize, board: &Board) -> isize {
        if depth == 0 {
            return self.eval_board(board);
        }

        let moves = self.gen_legal_moves(board);
        let mut best_eval = self.eval_board(board);

        for m in moves.iter() {
            // make the move
            let next_board = board.make_move_new(*m);
            let next_eval = -dbg!(self.search_further(depth - 1, &next_board));

            if next_eval > best_eval {
                best_eval = next_eval;
            }
        }
        best_eval
    }

    fn eval_board(&self, board: &Board) -> isize {
        match board.status() {
            chess::BoardStatus::Ongoing => {
                let board_sum = board.material_sum();
                // we do `self.board` because we need to find out if the current eval is good for
                // what colour
                match board.side_to_move() {
                    Color::White => board_sum.black as isize - board_sum.white as isize,
                    Color::Black => board_sum.white as isize - board_sum.black as isize,
                }
            }
            chess::BoardStatus::Stalemate => 0,
            chess::BoardStatus::Checkmate => {
                if board.side_to_move() == self.side_playing_for {
                    isize::MIN
                } else {
                    isize::MAX
                }
            }
        }
    }
}

impl FromStr for Engine {
    type Err = chess::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board = Board::from_str(s)?;
        Ok(Self {
            board,
            best_move: None,
            side_playing_for: board.side_to_move(),
        })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::Engine;

    #[test]
    fn test_eval_new() {
        // let mut engine = Engine::new();
        // let eval = engine.search(3);
        // assert!(eval != 0);
        // assert_eq!(engine.best_move.unwrap().to_string().as_str(), "sd");

        let mut engine = Engine::from_str("3K4/7r/6r1/1k6/8/8/8/8 b - - 0 1").expect("IDIOT");
        let eval = engine.search(3);
        assert_eq!(eval, isize::MAX);
        assert_eq!(engine.best_move.unwrap().to_string(), "g6g8");
    }
}
