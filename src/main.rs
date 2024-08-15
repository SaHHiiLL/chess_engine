#![allow(warnings)]
extern crate chess;

use chess_engine::*;
use std::panic;

fn main() {
    #[cfg(not(debug_assertions))]
    {
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
    }
    {
        let op = &[
            "1.e4 c6 2.d4 d5 3.e5 Bf5 4.Bd3 Bxd3 5.Qxd3 e6 6.f4 c5 7.c3 Nc6",
            "1. Nf3 Nf6 2. c4 e6 3. Nc3 c5 4. g3 b6 5. Bg2 Bb7",
            "1. b3 d5 2. Nf3 Nf6 3. Bb2 Bf5 4. Nh4 Bd7 5. e3 c5 6. c4 e6 7. Be2 Be7 8. O-O O-O 9. Nf3 Nc6 10. d4 cxd4 11. exd4 Rc8 12. Nbd2 dxc4 13. bxc4 Re8 14. Nb3 Bf8",
            "1. e4 c5 2. Nf3 d6 3. c3 Nf6 4. Bd3 Bg4 5. h3 Bh5 6. Be2 e6 7. O-O Be7 8. d3 O-O 9. Be3 Nc6 10. Nbd2 d5 11. Re1 e5 12. Bg5 dxe4 13. dxe4 Qc7 14. Qc2 h6 15. Bxf6 Bxf6 16. Nc4 Bxf3 17. Bxf3 g6 18. g3 h5",
            "1. e4 e6 2. h4 h6 3. d4 d5 4. Nc3 Bb4 5. e5 c5 6. a3 Bxc3+ 7. bxc3 Ne7 8. Qg4 Qa5 9. Bd2 Rg8 10. Bd3 Nf5 11. Ne2 Qa4 12. Rb1 c4 13. Bxf5 exf5 14. Qf3 Qxc2 15. Rb5 Be6 16. Nf4 Qe4+ 17. Qxe4 fxe4 18. Nxd5 a6",
            "1. d4 d5 2. Nc3 Nf6 3. Bf4 e6 4. Nb5 Na6 5. e3 Bb4+ 6. c3 Be7 7. a4 O-O 8. Bd3 c6 9. Na3 c5 10. Nf3 Ne4 ",
            "1. e4 Nf6 2. e5 Nd5 3. d4 d6 4. Bc4 Nb6 5. e6 Nxc4 6. exf7+ Kxf7 7. Qf3+ Ke8 8. Qh5+ g6 9. Qb5+ Qd7 10. Qxc4 Qe6+ ",
            "1. e4 Nf6 2. e5 Nd5 3. d4 d6 4. c4 Nb6 5. exd6 cxd6 6. Nf3 g6 7. Nc3 Bg7 8. h3 O-O 9. Be2 Nc6 10. O-O Bf5",
            "1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 4. Ba4 Nf6 5. O-O Nxe4 6. d4 b5 7. Bb3 d5 8. dxe5 Be6 9. Nbd2 Be7 10. c3 O-O 11. Bc2 f5",
            "1. e4 c5 2. Nf3 Nc6 3. Bb5 e6 4. Bxc6 bxc6 5. d3 Ne7 6. Qe2 f6 7. Nh4 g6 8. f4 Bg7 9. O-O O-O",
            "1. e4 d5 2. exd5 Qxd5 3. Nc3 Qa5"
        ];
        let mut opening_database = OpeningDatabase::new();
        op.iter()
            .for_each(|f| opening_database.add_png(f.to_string()));
        let mut uci = UCI::new();
        uci.add_db(opening_database);
        uci.rx();
    }
}
