use std::{
    collections::VecDeque,
    io::{self, Write},
    str::FromStr,
    time::Duration,
};

use chess::ChessMove;

use crate::engine2::Engine;

pub struct UCI {
    engine: Engine,
    curr_think_time: u64,
}

impl UCI {
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
            curr_think_time: 0,
        }
    }

    pub fn rx(&mut self) {
        let mut buffer = String::new();
        loop {
            // read
            let _ = io::stdout().flush();
            match io::stdin().read_line(&mut buffer) {
                Ok(_) => {
                    let mut input = buffer.trim().split(' ').collect::<VecDeque<_>>();
                    let cmd = match input.pop_front() {
                        Some(s) => s,
                        None => continue,
                    };

                    if cmd.is_empty() {
                        continue;
                    }

                    match cmd {
                        "uci" => {
                            self.tx("id name NotSoBrightBot");
                            self.tx("id author Sahil");
                            self.tx("uciok");
                        }
                        "isready" => self.tx("readyok"),
                        "position" => self.handle_position_command(input),
                        "ucinewgame" => self.handle_ucinewgame_command(),
                        "go" => self.handle_go_command(input),
                        "stop" => self.handle_stop_command(),
                        "quit" => {
                            break;
                        }
                        "d" => self.handle_debug_command(),
                        " " => {}
                        _ => {
                            let input = input.into_iter().collect::<Vec<_>>().join(" ");
                        }
                    }
                }
                Err(_) => todo!(),
            };
            buffer.clear();
        }
    }

    fn handle_debug_command(&mut self) {}
    fn handle_stop_command(&mut self) {
        if let Some(mov) = self.engine.get_best_mov() {
            self.engine.play_best_move();
            let msg = format!("bestmove {mov}");
            self.tx(msg);
        }
    }

    fn handle_ucinewgame_command(&mut self) {
        self.engine = Engine::new();
    }

    fn handle_go_command(&mut self, mut args: VecDeque<&str>) {
        match args.pop_front() {
            Some("movetime") => {}
            Some("infinite") => {
                let time_to_sleep: u64 = 5;
                std::thread::sleep(Duration::from_secs(time_to_sleep));
            }
            _ => {}
        };
        self.engine.search(3);
        if let Some(mov) = self.engine.get_best_mov() {
            let msg = format!("bestmove {mov}");
            self.tx(msg);
        }
    }

    fn handle_position_command(&mut self, mut cmd: VecDeque<&str>) {
        let mut position_type = match cmd.pop_front() {
            Some(pt) => pt,
            None => {
                return;
            }
        };

        fn parse_moves(mut str: VecDeque<&str>) -> VecDeque<ChessMove> {
            let mut res = VecDeque::new();
            while let Some(mov) = str.pop_front() {
                let _ = mov
                    .parse::<ChessMove>()
                    .map(|chessmove| res.push_back(chessmove));
            }
            res
        }

        match position_type {
            "fen" => {
                let mut fen_part = Vec::new();
                while let Some(fp) = cmd.pop_front() {
                    if fp == "moves" {
                        break;
                    } else {
                        fen_part.push(fp);
                    }
                }
                let fen = fen_part.join(" ");
                self.engine = Engine::from_str(&fen).unwrap();

                let moves = parse_moves(cmd);
                // self.engine = Engine::new();
                self.engine.play_moves(moves.into());
            }
            "startpos" => {
                let moves = parse_moves(cmd);
                self.engine = Engine::new();
                self.engine.play_moves(moves.into());
            }
            _ => {
                let msg = "invalid args";
            }
        };
    }

    fn tx<S: ToString>(&self, msg: S) {
        let msg = msg.to_string();
        println!("{msg}");
    }
}
