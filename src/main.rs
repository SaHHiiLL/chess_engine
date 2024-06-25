use std::io::Write;
use std::{
    fs::{File, OpenOptions},
    sync::Arc,
};

use log::{debug, error, info, warn, LevelFilter};

extern crate chess;
mod engine;
mod uci;

fn main() {
    let target = Box::new(
        File::create("/home/Sahil/programing/rust/chess_engine/chess_engine.log")
            .expect("Can't create file"),
    );

    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(target))
        .filter(None, LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
    debug!("Starting Engine");

    if let Err(err) = std::process::Command::new("notify-send")
        .arg("Helolo")
        .spawn()
    {
        error!("Could Not send notification for restart: {err}");
    };

    let mut uci = uci::UCI::new();
    uci.rx();
}
