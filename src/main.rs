use std::io::Write;
use std::{fs::File, path::Path};

use chrono::Utc;
use log::{debug, error, LevelFilter};

extern crate chess;
mod engine;
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
    // let file_path = Path::new("/home/Sahil/programing/rust/chess_engine/log/chess_engine.txt");
    // let target = Box::new({
    //     if file_path.exists() {
    //         let time = Utc::now();
    //
    //         let new_name = format!(
    //             "/home/Sahil/programing/rust/chess_engine/log/chess_engine{}.txt",
    //             time.to_rfc3339()
    //         );
    //         let new_name = Path::new(&new_name);
    //         let _ = std::fs::rename(file_path, new_name);
    //
    //         File::create(file_path)
    //             .map(|file| {
    //                 let msg = format!("Created File: {file:?}");
    //                 send_noti(msg);
    //                 file
    //             })
    //             .map_err(|err| {
    //                 let msg = format!("Could not create the file: {}", err);
    //                 send_noti(msg);
    //                 err
    //             })
    //             .unwrap()
    //     } else {
    //         File::create(file_path)
    //             .map(|file| {
    //                 let msg = format!("Created File: {file:?}");
    //                 send_noti(msg);
    //                 file
    //             })
    //             .map_err(|err| {
    //                 let msg = format!("Could not create the file: {}", err);
    //                 send_noti(msg);
    //                 err
    //             })
    //             .unwrap()
    //     }
    // });
    {
        // env_logger::Builder::new()
        //     .target(env_logger::Target::Pipe(target))
        //     .filter(None, LevelFilter::Debug)
        //     .format(|buf, record| {
        //         writeln!(
        //             buf,
        //             "[{} {} {}:{}] {}",
        //             chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
        //             record.level(),
        //             record.file().unwrap_or("unknown"),
        //             record.line().unwrap_or(0),
        //             record.args()
        //         )
        //     })
        //     .init();
        // debug!("Starting Engine");

        send_noti("Engine Started newww");

        let mut uci = uci::UCI::new();
        uci.rx();
    }

    // let time = Utc::now();
    //
    // let new_name = format!(
    //     "/home/Sahil/programing/rust/chess_engine/log/chess_engine{}.txt",
    //     time.to_rfc3339()
    // );
    // let new_name = Path::new(&new_name);
    // let _ = std::fs::rename(file_path, new_name);
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
