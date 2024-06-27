use std::{
    collections::VecDeque,
    io::{self, Write},
    str::FromStr,
    thread::sleep,
    time::{Duration, SystemTime},
};

use chess::ChessMove;

use crate::{engine::Engine, send_noti};

pub struct UCI {
    engine: Engine,
}

impl UCI {
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
        }
    }

    pub fn rx(&mut self) {
        let mut buffer = String::new();
        loop {
            // read
            let _ = io::stdout().flush();
            let _ = io::stdin().read_line(&mut buffer).unwrap();
            let mut input = buffer.trim().split(' ').collect::<VecDeque<_>>();
            let cmd = match input.pop_front() {
                Some(s) => s,
                None => continue,
            };

            if cmd.is_empty() {
                continue;
            }

            log::debug!("Message Received: `{cmd}` args: {input:?}");

            match cmd {
                "uci" => self.tx("uciok"),
                "isready" => self.tx("readyok"),
                "position" => self.handle_position_command(input),
                "ucinewgame" => self.handle_ucinewgame_command(),
                "go" => self.handle_go_command(input),
                "stop" => self.handle_stop_command(),
                "quit" => {
                    log::debug!("Quit..");
                    break;
                }
                "d" => self.handle_debug_command(),
                " " => {}
                _ => {
                    let input = input.into_iter().collect::<Vec<_>>().join(" ");
                    log::error!("Unknown Command: `{cmd}` -- args: `{input}`");
                }
            }

            buffer.clear();
        }
    }

    fn handle_debug_command(&mut self) {
        todo!("Handle debug command");
    }
    fn handle_stop_command(&mut self) {
        let mov = self.engine.get_best_mov().to_string();
        let msg = format!("bestmove {mov}");
        self.tx(msg);
    }

    fn handle_ucinewgame_command(&mut self) {
        self.engine.set_default_board();
        log::debug!("Uci New Game");
    }

    fn handle_go_command(&mut self, _args: VecDeque<&str>) {
        log::debug!("thinking...");
        let now = SystemTime::now();

        let msg = format!( "info depth 1 seldepth 0\n info score cp 13  depth 1 nodes 13 time 15 pv f1b5\n info depth 2 seldepth 2\n info nps 15937\n info score cp 14  depth 2 nodes 255 time 15 pv f1c4 f8c5 \n info depth 2 seldepth 7 nodes 255\n info depth 3 seldepth 7\n info nps 26437\n info score cp 20  depth 3 nodes 423 time 15 pv f1c4 g8f6 b1c3\n info nps 41562");

        println!("{}", msg);

        // we sleep for 2 seconds
        sleep(Duration::from_secs(2));
        match now.elapsed() {
            Ok(elapsed) => {
                let mov = self.engine.get_best_mov().to_string();
                let s_mov = self.engine.get_best_mov().to_string();
                let msg = format!("bestmove {mov} ponder {s_mov}");
                send_noti(msg.clone());
                self.tx(msg);
            }
            Err(e) => {
                log::error!("Could not sleep");
            }
        }
    }

    fn handle_position_command(&mut self, mut cmd: VecDeque<&str>) {
        log::debug!("Handling position command");

        let mut position_type = match cmd.pop_front() {
            Some(pt) => pt,
            None => {
                log::error!("Expected Args -- found none");
                return;
            }
        };

        fn add_moves(mut str: VecDeque<&str>) -> VecDeque<ChessMove> {
            let mut res = VecDeque::new();
            while let Some(mov) = str.pop_front() {
                let _ = mov
                    .parse::<ChessMove>()
                    .map(|chessmove| res.push_back(chessmove));
            }
            res
        }

        let mut last_move = None;
        let mut last_number = None;
        if position_type != "fen" && position_type != "startpos" {
            let _ = ChessMove::from_str(position_type)
                .map(|d| last_move = Some(d))
                .map_err(|_| {
                    let _ = position_type
                        .parse::<usize>()
                        .map(|d| last_number = Some(d));
                });
            position_type = match cmd.pop_front() {
                Some(pt) => pt,
                None => {
                    log::error!("Expected Args -- found none");
                    return;
                }
            };
        }

        match position_type {
            "fen" => {
                log::debug!("position command received: parsing fen");
                let msg = format!(
                    "current vec: {:?}",
                    cmd.clone().into_iter().collect::<Vec<_>>()
                );
                crate::send_noti(msg);

                let mut fen_part = Vec::new();
                while let Some(fp) = cmd.pop_front() {
                    if fp == "moves" {
                        break;
                    } else {
                        fen_part.push(fp);
                    }
                }
                let fen = fen_part.join(" ");
                log::debug!("FEN: {fen}");
                self.engine = Engine::from_fen(fen);

                let moves = add_moves(cmd);
                log::debug!("Moves: {moves:?}");

                self.engine.play_move(moves.into());
            }
            "startpos" => {
                log::debug!("start position");
                let moves = add_moves(cmd);
                log::debug!("Moves: {moves:?}");

                self.engine.play_move(moves.into());
            }
            _ => {
                log::error!("Invalid args");
                let msg = "invalid args";
                crate::send_noti(msg);
            }
        };

        // match position_type {
        //     "fen" => {
        //         //TODO: have to fix this asap
        //         let cmd: Vec<String> = cmd.into_iter().collect();
        //         let fen_part = match cmd.iter().position(|d| *d == "moves") {
        //             Some(idx) => &cmd[1..idx],
        //             None => &cmd[1..cmd.len()],
        //         };
        //         self.engine = Engine::from_fen(fen_part.join(" "));
        //         log::debug!("Engine Set to fen position {fen_part:?}");
        //
        //         add_moves(cmd);
        //     }
        //     "startpos" => {
        //         log::debug!("setting board to default position");
        //         self.engine.set_default_board();
        //         add_moves(cmd);
        //     }
        //     _ => log::error!("Invalid Argument for position command: {cmd:?}"),
        // }
    }

    fn tx<S: ToString>(&self, msg: S) {
        let msg = msg.to_string();
        log::debug!("Sending Message: {msg}");
        println!("{msg}");
    }
}
