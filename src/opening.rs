use std::str::FromStr;

use chess::{Board, ChessMove};

use crate::trie::*;

#[derive(Clone)]
pub struct OpeningDatabase {
    opening_lines: Trie<ChessMove>,
}

impl OpeningDatabase {
    pub fn is_end(&self) -> bool {
        self.opening_lines.is_end()
    }
    pub fn new() -> Self {
        Self {
            opening_lines: Trie::default(),
        }
    }

    pub fn choose_opening_move(&mut self, chess_move: ChessMove) -> bool {
        self.opening_lines.change_root(chess_move)
    }

    pub fn add_png(&mut self, pgn: String) {
        let mut moves = vec![];
        let mut parser = PgnParser::new(pgn);
        let mut board = Board::default();
        while let Ok((white, black)) = parser.next_token() {
            let white_move = ChessMove::from_san(&board, &white).unwrap();
            board = board.make_move_new(white_move);
            moves.push(white_move);

            let black_move = ChessMove::from_san(&board, &black).unwrap();
            board = board.make_move_new(black_move);
            moves.push(black_move);
        }
        self.opening_lines.insert(&moves);
    }

    pub fn root(&self) -> &Node<ChessMove> {
        self.opening_lines.root()
    }

    pub fn print(&self) {
        self.opening_lines.print()
    }
}
trait ToChar {
    fn to_char(&self) -> char;
}

impl ToChar for u8 {
    fn to_char(&self) -> char {
        char::from_u32(*self as u32).expect("Expected UTF-8")
    }
}

#[derive(Debug)]
enum ParseError {
    NoValidChar,
    InvalidMove,
}

struct PgnParser {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    char: u8,
    eof: bool,
}

impl PgnParser {
    fn new<T: ToString>(input: T) -> Self {
        let input = input.to_string().as_bytes().to_vec();
        let mut res = Self {
            input,
            position: 0,
            read_position: 0,
            char: 0,
            eof: false,
        };
        res.read_char();
        res
    }

    pub(crate) fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            // Checking if the next position is
            // end of the file or exceeded the
            // file1. e4 Nc6 2. c4 e5 3. Nc3 Nf6 4. Be2 Bc5 5. Nf3 h6 6. a3 d5 7. cxd5 Bg4 8. b4 Be7 9. h3 Bh5 10. g4 Bg6 11. Qb3 Qd6 12. O-O O-O
            self.char = 0; // setting it to Zero to indicate NULL or EOF
            self.eof = true;
        } else {
            self.char = self.input[self.read_position];
        }
        self.position = self.read_position; // Advancing the position
        self.read_position = self.read_position.wrapping_add(1); // Advancing the read_position
    }

    fn skip_whitespace(&mut self) {
        while self.char.to_char().is_whitespace() {
            self.read_char();
        }
    }
    fn is_letter(&self) -> bool {
        self.char.to_char().is_alphabetic()
    }

    fn read_move(&mut self) -> String {
        self.skip_whitespace();
        let mut res = String::new();
        while !self.char.to_char().is_whitespace() {
            res.push(self.char.to_char());
            self.read_char();
            if self.eof {
                break;
            }
        }
        res
    }

    // (e4, e5)
    pub(crate) fn next_token(&mut self) -> Result<(String, String), ParseError> {
        self.skip_whitespace();
        let mut res = Err(ParseError::NoValidChar);

        loop {
            if self.eof {
                return Err(ParseError::NoValidChar);
            }
            let curr = self.char.to_char();
            if self.is_letter() {
                let left_move = self.read_move();
                let right_move = self.read_move();
                res = Ok((left_move, right_move));
                break;
            }
            if curr.is_ascii_digit() || curr == '.' || curr == ' ' {
                self.read_char();
            }
        }
        self.read_char();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opening() {
        let test_in = String::from(
            "1.e4 c6 2.d4 d5 3.e5 Bf5 4.Bd3 Bxd3 5.Qxd3 e6 6.f4 c5 7.c3 Nc6 8.Nf3 Qb6 9.O-O Nh6",
        );
        let mut db = OpeningDatabase::new();
        db.add_png(test_in);
        db.print();
    }
}
