#![feature(test)]
#![allow(warnings)]
use chess::{Board, Color, Square};
pub(crate) mod consts;
pub(crate) mod engine;
pub(crate) mod eval;
pub(crate) mod game_phase;
pub(crate) mod opening;
pub(crate) mod trie;
pub mod uci;

pub use consts::*;
pub use opening::OpeningDatabase;
pub use uci::*;

pub struct BoardMaterial {
    pub white: u32,
    pub black: u32,
}
pub trait MaterialSumExt {
    fn material_sum(&self) -> BoardMaterial;
    fn material_sum_bitboard(&self) -> BoardMaterial;
}

impl MaterialSumExt for chess::Board {
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
                    chess::Color::White => mat.white += *p as u32,
                    chess::Color::Black => mat.black += *p as u32,
                };
            }
        }
        mat
    }

    fn material_sum_bitboard(&self) -> BoardMaterial {
        let mut mat = BoardMaterial { white: 0, black: 0 };
        let board: &Board = self;
        let pieces = &[
            chess::Piece::Pawn,
            chess::Piece::Bishop,
            chess::Piece::Knight,
            chess::Piece::King,
            chess::Piece::Queen,
            chess::Piece::Rook,
        ];

        let white_bitboard = board.color_combined(Color::White);
        let black_bitboard = board.color_combined(Color::Black);

        for p in pieces.iter() {
            let piece_bitboard = board.pieces(*p);
            let white_piece = piece_bitboard & white_bitboard;
            let black_piece = piece_bitboard & black_bitboard;
            let piece_value = PIECE_VALUE_MAP.get(p).expect("YOU IDIOT");
            mat.white += white_piece.0.count_ones() * *piece_value as u32;
            mat.black += black_piece.0.count_ones() * *piece_value as u32;
        }
        mat
    }
}

extern crate test;
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use self::{consts::FEN_STRING, engine::Engine, eval::Evaluation};

    use super::*;
    use crate::MaterialSumExt;
    use test::Bencher;

    #[bench]
    fn bench_eval_board(b: &mut Bencher) {
        b.iter(|| {
            let engine = Engine::from_str(&FEN_STRING[0]).unwrap();
            Evaluation::new(&engine.board()).eval_board(engine.board(), engine.history());
        })
    }

    #[bench]
    fn bench_search_alpha_beta(b: &mut Bencher) {
        b.iter(|| {
            FEN_STRING.iter().take(3).for_each(|fen| {
                Engine::from_str(fen).unwrap().search(3);
            })
        });
    }

    #[bench]
    fn bench_material_count_bitboard(b: &mut Bencher) {
        b.iter(|| {
            let engine = engine::Engine::new();
            engine.board().material_sum_bitboard();
        })
    }

    #[bench]
    fn bench_material_count(b: &mut Bencher) {
        b.iter(|| {
            let engine = engine::Engine::new();
            engine.board().material_sum();
        })
    }
}
