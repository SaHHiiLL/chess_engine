extern crate chess;
mod engine2;
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
