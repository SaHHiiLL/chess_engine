#![feature(test)]

extern crate test;

pub mod engine;

use lazy_static::lazy_static;

lazy_static! {
    static ref FEN_STRING: Vec<String> = vec![
        String::from("r6r/1b2k1bq/8/8/7B/8/8/R3K2R b KQ - 3 2"),
        String::from("8/8/8/2k5/2pP4/8/B7/4K3 b - d3 0 3"),
        String::from("r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 2 2"),
        String::from("r3k2r/p1pp1pb1/bn2Qnp1/2qPN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQkq - 3 2"),
        String::from("2kr3r/p1ppqpb1/bn2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQ - 3 2"),
        String::from("rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R w KQ - 3 9"),
        String::from("2r5/3pk3/8/2P5/8/2K5/8/8 w - - 5 4"),
        String::from("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8"),
        String::from("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10"),
        String::from("3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1"),
        String::from("8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1"),
        String::from("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1"),
        String::from("5k2/8/8/8/8/8/8/4K2R w K - 0 1"),
        String::from("3k4/8/8/8/8/8/8/R3K3 w Q - 0 1"),
        String::from("r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1"),
        String::from("r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1"),
        String::from("2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1"),
        String::from("8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1"),
        String::from("4k3/1P6/8/8/8/8/K7/8 w - - 0 1"),
        String::from("8/P1k5/K7/8/8/8/8/8 w - - 0 1"),
        String::from("K1k5/8/P7/8/8/8/8/8 w - - 0 1"),
        String::from("8/k1P5/8/1K6/8/8/8/8 w - - 0 1"),
        String::from("8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1"),
    ];
}
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use self::engine::Engine;

    use super::*;
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
}
