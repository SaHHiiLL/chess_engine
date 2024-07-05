use std::str::FromStr;

use chess::{Board, ChessMove, MoveGen, Square};
use lazy_static::lazy_static;
use rand::seq::SliceRandom;

pub struct Engine {
    board: Board,
    eval: f64,
    current_pos_moves: Vec<ChessMove>,
    /// index form `current_pos_moves`
    current_best_move: usize,
    depth: u8,
    white_material: u32,
    black_material: u32,
}

lazy_static! {
    static ref PIECE_VALUE: Vec<(chess::Piece, u32)> = vec![
        (chess::Piece::King, 20_000),
        (chess::Piece::Queen, 900),
        (chess::Piece::Rook, 500),
        (chess::Piece::Knight, 300),
        (chess::Piece::Bishop, 300),
        (chess::Piece::Pawn, 100),
    ];
    static ref INITIAL_VALUE: u16 = 23_900;
}
struct BoardMaterial {
    white: u32,
    black: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EngineGameState {
    Draw,
    Win,
    Lose,
    Ongoing(isize),
}

impl EngineGameState {
    fn to_isize(&self) -> isize {
        match self {
            EngineGameState::Draw => 0,
            EngineGameState::Win => isize::MAX,
            EngineGameState::Lose => isize::MIN,
            EngineGameState::Ongoing(eval) => *eval,
        }
    }
    fn is_better(&self, other: &EngineGameState) -> bool {
        self.to_isize() < other.to_isize()
    }
}

impl Engine {
    pub fn new() -> Self {
        let board = Board::default();
        Self {
            board,
            eval: 0.0,
            current_pos_moves: MoveGen::new_legal(&board).collect(),
            current_best_move: 0,
            depth: 3,
            white_material: 0,
            black_material: 0,
        }
    }

    pub fn from_fen<S: ToString>(s: S) -> Self {
        let s = s.to_string();
        let board = Board::from_str(&s).unwrap();

        // TODO: evaluate the engine
        let mut engine = Self {
            board,
            eval: 0.0,
            current_pos_moves: MoveGen::new_legal(&board).collect(),
            current_best_move: 0,
            depth: 3,
            white_material: 0,
            black_material: 0,
        };

        let mat = engine.get_material_sum();
        engine.white_material = mat.white;
        engine.black_material = mat.black;
        engine
    }

    /// TODO: put this in a trait for `Board`
    fn get_material_sum(&self) -> BoardMaterial {
        let mut mat = BoardMaterial { white: 0, black: 0 };
        for sq in 0..64 {
            // SAFETY: squares are only created from 0 to 64 (not including 64)
            let sq = unsafe { Square::new(sq) };
            let piece_type = self.board.piece_on(sq);
            let color = self.board.color_on(sq);

            if let (Some(piece_type), Some(color)) = (piece_type, color) {
                let (_, get_piece_worth) = PIECE_VALUE
                    .iter()
                    .find(|(p, _)| *p == piece_type)
                    .expect("Imposible");
                match color {
                    chess::Color::White => mat.white += *get_piece_worth,
                    chess::Color::Black => mat.black += *get_piece_worth,
                };
            }
        }
        mat
    }

    fn regen_legal_moves(&mut self) {
        self.current_pos_moves = MoveGen::new_legal(&self.board).collect();
    }

    fn gen_board_legal_moves(&self, board: &Board) -> Vec<ChessMove> {
        MoveGen::new_legal(board).collect()
    }

    pub fn play_move(&mut self, moves: Vec<ChessMove>) {
        for m in moves.iter() {
            self.board = self.board.make_move_new(*m);
        }
        self.regen_legal_moves();
    }

    pub fn get_default_board(&self) -> Self {
        Engine::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
    pub fn get_side_to_move(&self) -> chess::Color {
        self.board.side_to_move()
    }

    pub fn get_best_mov(&mut self) -> Option<&ChessMove> {
        self.current_pos_moves.get(self.current_best_move)
    }

    pub fn search(&mut self, depth: u8) -> EngineGameState {
        self.search_best_move(depth, &self.board.clone(), self.get_side_to_move())
    }

    fn search_best_move(
        &mut self,
        depth: u8,
        board: &Board,
        player_for: chess::Color,
    ) -> EngineGameState {
        if depth == 0 {
            return self.eval_board(player_for, board);
        }

        let moves = self.gen_board_legal_moves(board);

        // check if player has no more moves left
        if moves.is_empty() {
            // checks if the board is in check
            if board.checkers().0 != 0 {
                return EngineGameState::Lose;
            } else {
                // draw by stalement
                return EngineGameState::Draw;
            }
        }

        let mut best_eval = if player_for == board.side_to_move() {
            EngineGameState::Lose
        } else {
            EngineGameState::Win
        };
        for (idx, mov) in moves.iter().enumerate() {
            let new_board = board.make_move_new(*mov);
            let new_eval = self.search_best_move(depth - 1, &new_board, player_for);
            let is_better = best_eval.is_better(&new_eval);
            if is_better {
                self.current_best_move = idx;
                best_eval = new_eval;
            }
        }

        best_eval
    }

    pub fn eval_board(&self, player_for: chess::Color, board: &Board) -> EngineGameState {
        match board.status() {
            chess::BoardStatus::Ongoing => {}
            chess::BoardStatus::Stalemate => return EngineGameState::Draw,
            chess::BoardStatus::Checkmate => {
                if player_for == board.side_to_move() {
                    return EngineGameState::Lose;
                } else {
                    return EngineGameState::Win;
                }
            }
        };
        let mut white_weight = 0;
        let mut black_weight = 0;

        let mat_count = self.get_material_sum();
        white_weight += mat_count.white;
        black_weight += mat_count.black;

        // let final_eval = if player_for == self.board.side_to_move() {
        //     white_weight as isize - black_weight as isize
        // } else {
        //     -(white_weight as isize - black_weight as isize)
        // };

        let final_eval = if player_for == chess::Color::White {
            white_weight as isize - black_weight as isize
        } else {
            black_weight as isize - white_weight as isize
        };
        // if final_eval != 0 {
        //     println!("{}", final_eval);
        // }

        EngineGameState::Ongoing(final_eval)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chess::ChessMove;

    use crate::engine::EngineGameState;

    use super::Engine;

    #[test]
    fn test_eval() {
        let engine = Engine::from_fen("8/8/4k3/8/2p5/8/B2P2K1/8 b - - 0 1");
        let board = engine.board;
        if let crate::engine::EngineGameState::Ongoing(eval) =
            engine.eval_board(chess::Color::White, &board)
        {
            assert_eq!(eval, 300);
        } else {
            panic!("Something went wrong here lol");
        }
    }

    #[test]
    fn test_eval_again() {
        let mut engine = Engine::from_fen("2r1k3/8/8/5Q2/8/2K5/8/8 w - - 0 1");
        let board = engine.board;
        assert_eq!(
            EngineGameState::Ongoing(400),
            engine.eval_board(chess::Color::White, &board)
        );
        engine.play_move(vec![ChessMove::from_str("f5c8").unwrap()]);
        assert_eq!(
            EngineGameState::Ongoing(900),
            engine.eval_board(chess::Color::White, &board)
        );
    }

    #[test]
    fn test_search() {
        let mut engine = Engine::from_fen("2r1k3/8/8/5Q2/8/2K5/8/8 w - - 0 1");
        // match crate::engine::EngineGameState::Ongoing(eval) = engine.search(2) {
        //     println!("{}", engine.get_best_mov().unwrap());
        // }
        //
        match engine.search(2) {
            super::EngineGameState::Draw => println!("draw"),
            super::EngineGameState::Win => println!("WIN"),
            super::EngineGameState::Lose => println!("LOSE"),
            super::EngineGameState::Ongoing(eval) => {
                println!("BEST move: {}", engine.get_best_mov().unwrap());
                println!("eval: {eval}");
            }
        }
    }
}
