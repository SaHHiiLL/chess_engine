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
    white_material: usize,
    black_material: usize,
}

lazy_static! {
    static ref PIECE_VALUE: Vec<(chess::Piece, i32)> = vec![
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
    white: usize,
    black: usize,
}
enum EngineGameState {
    Draw,
    Win,
    Lose,
    Ongoing(isize),
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
                    chess::Color::White => mat.white += *get_piece_worth as usize,
                    chess::Color::Black => mat.black += *get_piece_worth as usize,
                };
            }
        }
        mat
    }

    fn regen_legal_moves(&mut self) {
        self.current_pos_moves = MoveGen::new_legal(&self.board).collect();
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
        self.current_pos_moves.choose(&mut rand::thread_rng())
    }

    fn minimax(
        &self,
        board: Board,
        depth: u8,
        player_for: chess::Color,
        last_move: ChessMove,
    ) -> ChessMove {
        if depth == 0 {
            return last_move;
        }
        if let EngineGameState::Ongoing(eval) = self.eval_current_pos(player_for, board) {
        } else {
            return last_move;
        }

        todo!()
    }

    pub fn eval_current_pos(&self, side: chess::Color, board: Board) -> EngineGameState {
        match board.status() {
            chess::BoardStatus::Ongoing => {}
            chess::BoardStatus::Stalemate => return EngineGameState::Draw,
            chess::BoardStatus::Checkmate => {
                if side == board.side_to_move() {
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

        EngineGameState::Ongoing((white_weight - black_weight).try_into().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use chess::Board;

    use super::Engine;

    #[test]
    fn test_eval() {
        let engine = Engine::from_fen("1R1N4/2K1B3/8/4r3/2nk4/8/8/8 w - - 0 1");
        let board = engine.board;
        if let crate::engine::EngineGameState::Ongoing(eval) =
            engine.eval_current_pos(chess::Color::White, board)
        {
            assert_eq!(eval, 300);
        } else {
            panic!("Something went wrong here lol");
        }
    }
}
