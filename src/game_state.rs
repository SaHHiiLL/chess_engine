use chess::{Board, ChessMove};

use crate::{game_phase::GamePhases, BoardMaterial};

#[derive(Clone, Copy, Default)]
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

    pub fn update_from_last_move(&mut self) {
        if let Some(mov) = self.last_move {}
    }

    pub fn has_castel(&self, color: chess::Color) -> bool {
        match color {
            chess::Color::White => self.has_white_castel,
            chess::Color::Black => self.has_black_castel,
        }
    }

    pub fn has_opp_castel(&self, color: chess::Color) -> bool {
        match color {
            chess::Color::Black => self.has_white_castel,
            chess::Color::White => self.has_black_castel,
        }
    }

    pub fn has_castel_right(&self, color: chess::Color) -> bool {
        match color {
            chess::Color::White => self.white_castel_right,
            chess::Color::Black => self.black_castel_right,
        }
    }

    pub fn has_opp_castel_right(&self, color: chess::Color) -> bool {
        match color {
            chess::Color::Black => self.white_castel_right,
            chess::Color::White => self.black_castel_right,
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
