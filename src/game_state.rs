use chess::{Board, ChessMove};

use crate::{game_phase::GamePhases, BoardMaterial};

#[derive(Clone, Default)]
pub struct GameState {
    game_phases: GamePhases,
    last_move: Option<ChessMove>,
    has_black_castel: bool,
    has_white_castel: bool,
    black_castel_right: bool,
    white_castel_right: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            game_phases: GamePhases::Opening,
            last_move: None,
            ..Default::default()
        }
    }

    pub fn game_phases(&self) -> &GamePhases {
        &self.game_phases
    }

    pub fn last_move(&self) -> &Option<ChessMove> {
        &self.last_move
    }

    pub fn set_gamephases_middlegame(&mut self) {
        self.game_phases.set_middlegame()
    }

    pub fn set_lastmove(&mut self, mov: ChessMove) {
        self.last_move = Some(mov)
    }

    pub fn update_game_phase(&mut self, board_materail: BoardMaterial, board: &Board) {
        self.game_phases.update(board_materail, board);
    }
}
