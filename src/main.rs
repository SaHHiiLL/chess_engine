extern crate chess;

use chess_engine::*;
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
        let caro_kan =
            String::from("1.e4 c6 2.d4 d5 3.e5 Bf5 4.Bd3 Bxd3 5.Qxd3 e6 6.f4 c5 7.c3 Nc6");

        let mag = String::from("1. Nf3 Nf6 2. c4 e6 3. Nc3 c5 4. g3 b6 5. Bg2 Bb7");
        let mut opening_database = OpeningDatabase::new();
        opening_database.add_png(caro_kan);
        opening_database.add_png(mag);
        let mut uci = UCI::new();
        uci.add_db(opening_database);
        uci.rx();
    }
}
