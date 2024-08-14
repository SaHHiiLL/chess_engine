use chess::{Board, Piece};

use crate::{BoardMaterial, PieceFromColor};

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
        // can only set the end if it's in middle or opening;
        if self.is_end() {
            return;
        }
        if board.pieces(Piece::Queen).0 == 0 {
            self.set_endgame();
        } else {
            let mut total = 0;
            let pawn_bitboard = board.pieces(Piece::Pawn);
            let pawn_count = pawn_bitboard.0.count_ones();
            if pawn_count <= 12 {
                total += 1;
            }

            let knight_bitboar = board.pieces(Piece::Knight);
            let knight_count = knight_bitboar.0.count_ones();
            if knight_count <= 3 {
                total += 1;
            }

            let bishop_bitboard = board.pieces(Piece::Bishop);
            let bishop_count = bishop_bitboard.0.count_ones();
            if bishop_count <= 3 {
                total += 1;
            }

            let rook_bitboard = board.pieces(Piece::Rook);
            let rook_count = rook_bitboard.0.count_ones();
            if rook_count <= 3 {
                total += 1;
            }

            if total >= 2 {
                self.set_endgame();
            }
        }
    }

    pub fn set_endgame(&mut self) {
        *self = GamePhases::EndGame
    }

    pub fn is_end(&self) -> bool {
        *self == GamePhases::EndGame
    }

    pub fn set_middlegame(&mut self) {
        *self = GamePhases::EndGame
    }
}
