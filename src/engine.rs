use std::str::FromStr;

use chess::{Board, ChessMove, MoveGen};
use log::info;
use rand::seq::SliceRandom;

pub struct Engine {
    board: Board,
    eval: f64,
    current_pos_moves: Vec<ChessMove>,
    /// index form `current_pos_moves`
    current_best_move: usize,
}

impl Engine {
    pub fn new() -> Self {
        let board = Board::default();
        Self {
            board,
            eval: 0.0,
            current_pos_moves: MoveGen::new_legal(&board).collect(),
            current_best_move: 0,
        }
    }

    pub fn from_fen<S: ToString>(s: S) -> Self {
        let s = s.to_string();
        let board = Board::from_str(&s).unwrap();

        // TODO: evaluate the engine
        Self {
            board,
            eval: 0.0,
            current_pos_moves: MoveGen::new_legal(&board).collect(),
            current_best_move: 0,
        }
    }

    fn regen_legal_moves(&mut self) {
        self.current_pos_moves = MoveGen::new_legal(&self.board).collect();
        log::info!("New Genrated Moves: {:?}", self.current_pos_moves);
    }

    pub fn play_move(&mut self, moves: Vec<ChessMove>) {
        for m in moves.iter() {
            self.board = self.board.make_move_new(*m);
        }
        log::debug!("Board Side: {:?}", self.board.side_to_move());
        self.regen_legal_moves();
    }

    pub fn set_default_board(&mut self) {
        self.board = Board::default();
        self.eval = 0.0;
    }
    pub fn get_side_to_move(&self) -> chess::Color {
        self.board.side_to_move()
    }

    //TODO:
    pub fn get_best_mov(&mut self) -> &ChessMove {
        self.current_pos_moves
            .choose(&mut rand::thread_rng())
            .unwrap()
    }

    pub fn eval_current_pos(&self) -> Self {
        todo!()
    }
}
