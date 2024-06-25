use std::io::{self, Stdin, Write};

use crate::engine::Engine;

trait RemoveFirst {
    type Output;
    fn remove_first(&mut self) -> Option<Self::Output>;
}

impl<T> RemoveFirst for Vec<T> {
    type Output = T;
    fn remove_first(&mut self) -> Option<T> {
        if self.len() == 0 {
            None
        } else {
            Some(self.swap_remove(0))
        }
    }
}

pub struct UCI {
    engine: Engine,
    stdin: Stdin,
}

impl UCI {
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
            stdin: std::io::stdin(),
        }
    }

    pub fn rx(&mut self) {
        let mut buffer = String::new();
        loop {
            // read
            let _ = io::stdout().flush();
            let _ = io::stdin().read_line(&mut buffer).unwrap();
            let mut input = buffer.split(' ').collect::<Vec<_>>();
            let cmd = match input.remove_first() {
                Some(s) => s,
                None => continue,
            };

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
                    let input = input.join(" ");
                    log::error!("Unknown Command: {cmd} -- args: {input}");
                }
            }

            buffer.clear();
        }
    }

    fn handle_debug_command(&mut self) {
        todo!("Handle debug command");
    }
    fn handle_stop_command(&mut self) {
        todo!("Handle stop commnand");
    }

    fn handle_ucinewgame_command(&mut self) {
        self.engine.set_default_board();
        log::debug!("Uci New Game");
    }

    fn handle_go_command(&mut self, cmd: Vec<&str>) {
        todo!("Handle go comnad");
    }

    fn handle_position_command(&mut self, cmd: Vec<&str>) {}

    fn tx<S: ToString>(&self, msg: S) {
        let msg = msg.to_string();
        log::debug!("Sending Message: {msg}");
        println!("msg");
    }
}
