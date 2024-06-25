use std::str::FromStr;

use chess::{Board, ChessMove, Color, Game, MoveGen};
use rand::{random, seq::IteratorRandom, thread_rng};

pub struct Engine {
    board: Board,
    eval: f64,
    turn: Color,
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
            turn: Color::White,
            current_pos_moves: MoveGen::new_legal(&board).collect(),
            current_best_move: 0,
        }
    }

    pub fn from_fen<S: ToString>(s: S) -> Self {
        let s = s.to_string();
        let board = Board::from_str(&s).unwrap();
        let game = Game::new_with_board(board);

        let turn = game.side_to_move();

        // TODO: evaluate the engine
        Self {
            board,
            eval: 0.0,
            turn,
            current_pos_moves: MoveGen::new_legal(&board).collect(),
            current_best_move: 0,
        }
    }

    pub fn set_default_board(&mut self) {
        self.board = Board::default();
        self.eval = 0.0;
        self.turn = Color::White;
    }

    //TODO:
    pub fn get_best_mov(&mut self) -> &ChessMove {
        self.current_pos_moves.get(self.current_best_move).unwrap()
    }

    pub fn eval_current_pos(&self) -> Self {
        todo!()
    }
}
