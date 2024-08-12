use std::time::Duration;
use std::time::Instant;
use std::{
    collections::VecDeque,
    io::{self, Write},
    ops::Add,
};

use std::str::FromStr;

use chess::ChessMove;

use crate::engine::Engine;
use crate::OpeningDatabase;

pub struct UCI {
    engine: Engine,
    curr_think_time: u64,
    opening_db: OpeningDatabase,
}

impl UCI {
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
            curr_think_time: 0,
            opening_db: OpeningDatabase::new(),
        }
    }

    pub fn add_db(&mut self, opening_database: OpeningDatabase) {
        self.opening_db = opening_database.clone();
        self.engine.add_opening_db(opening_database);
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
        self.engine.add_opening_db(self.opening_db.clone());
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

        let now = Instant::now().add(Duration::from_secs(1));
        self.engine.search_iterative_deeping(now);

        if let Some(mov) = self.engine.get_best_mov() {
            let msg = format!("bestmove {mov}");
            self.engine.play_best_move();
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

        fn parse_moves(mut str: VecDeque<&str>) -> Vec<ChessMove> {
            let mut res = Vec::new();
            while let Some(mov) = str.pop_front() {
                let _ = mov
                    .parse::<ChessMove>()
                    .map(|chessmove| res.push(chessmove));
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
                if let Some(mov) = moves.iter().last() {
                    println!("info playing last move {mov}");
                    self.engine.play_move(*mov);
                }
            }
            "startpos" => {
                let moves = parse_moves(cmd);
                if let Some(mov) = moves.iter().last() {
                    println!("info playing last move {mov}");
                    self.engine.play_move(*mov);
                }
            }
            _ => {
                println!("info invalid cmd: {position_type}");
            }
        };
    }

    fn tx<S: ToString>(&self, msg: S) {
        let msg = msg.to_string();
        println!("{msg}");
    }
}

impl Default for UCI {
    fn default() -> Self {
        Self::new()
    }
}
