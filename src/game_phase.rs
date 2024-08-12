use chess::{Board, Piece};

use crate::BoardMaterial;

#[derive(Clone, PartialEq)]
pub enum GamePhases {
    Opening,
    MiddleGame,
    EndGame,
}

impl Default for GamePhases {
    fn default() -> Self {
        Self::Opening
    }
}

impl GamePhases {
    pub fn update(&mut self, mat: BoardMaterial, board: &Board) {
        if board.pieces(Piece::Queen).0 == 0 {
            self.set_endgame();
        } else {
            self.set_middlegame();
        }
    }

    pub fn set_endgame(&mut self) {
        *self = GamePhases::EndGame
    }

    pub fn set_middlegame(&mut self) {
        *self = GamePhases::EndGame
    }
}
