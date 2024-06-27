use std::io::Write;
use std::{fs::File, path::Path};

use log::{debug, error, LevelFilter};

extern crate chess;
mod engine;
mod uci;

fn main() {
    let file_path = Path::new("/home/Sahil/programing/rust/chess_engine/chess_engine.txt");
    let target = Box::new({
        if file_path.exists() {
            File::open(file_path)
                .map(|file| {
                    let msg = format!("Opened File: {file:?}");
                    send_noti(msg);
                    file
                })
                .map_err(|err| {
                    let msg = format!("Could not open file: {}", err);
                    send_noti(msg);
                    err
                })
                .unwrap()
        } else {
            File::create(file_path)
                .map(|file| {
                    let msg = format!("Created File: {file:?}");
                    send_noti(msg);
                    file
                })
                .map_err(|err| {
                    let msg = format!("Could not create the file: {}", err);
                    send_noti(msg);
                    err
                })
                .unwrap()
        }
    });

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

    send_noti("Engine Started 1");

    let mut uci = uci::UCI::new();
    uci.rx();
}

pub fn send_noti<S: ToString>(msg: S) {
    let msg = msg.to_string();

    if let Err(err) = std::process::Command::new("notify-send")
        .arg(msg.as_str())
        .spawn()
    {
        error!("Could Not send notification for restart: {err}");
    };
}
