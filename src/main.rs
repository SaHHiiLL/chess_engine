#![feature(test)]
extern crate chess;
mod consts;
mod engine;
mod opening;
mod uci;

use std::panic;

fn main() {
    panic::set_hook(Box::new(|panic_info| {
        println!("Some Panic happened");
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            println!("panic occurred: {s:?}");
        } else {
            println!("panic occurred");
        }

        if let Some(location) = panic_info.location() {
            println!(
                "panic occurred in file '{}' at line {}",
                location.file(),
                location.line(),
            );
        } else {
            println!("panic occurred but can't get location information...");
        }
    }));
    {
        let mut uci = uci::UCI::new();
        uci.rx();
    }
}

extern crate test;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use self::{consts::FEN_STRING, engine::Engine};

    use super::*;
    use engine::MaterialSumExt;
    use test::Bencher;

    #[bench]
    fn bench_search_minimax(b: &mut Bencher) {
        b.iter(|| {
            FEN_STRING.iter().take(3).for_each(|fen| {
                Engine::from_str(fen).unwrap().search_slow(3);
            })
        });
    }

    #[bench]
    fn bench_eval_board(b: &mut Bencher) {
        b.iter(|| {
            let engine = Engine::from_str(&FEN_STRING[0]).unwrap();
            engine.eval_board(engine.board());
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
