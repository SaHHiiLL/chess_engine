use std::{collections::HashMap, str::FromStr};

use chess::{Board, ChessMove, MoveGen, Square};
use lazy_static::lazy_static;

pub struct Engine {
    board: Board,
    current_pos_moves: Vec<ChessMove>,
    /// index form `current_pos_moves`
    current_best_move: Option<ChessMove>,
    your_side: chess::Color,
}

lazy_static! {
    static ref PIECE_VALUE: Vec<(chess::Piece, u32)> = vec![
        (chess::Piece::King, 20_000),
        (chess::Piece::Queen, 900),
        (chess::Piece::Rook, 500),
        (chess::Piece::Knight, 300),
        (chess::Piece::Bishop, 300),
        (chess::Piece::Pawn, 100),
    ];
    static ref PIECE_VALUE_MAP: HashMap<chess::Piece, u32> = {
        let mut map = HashMap::new();
        map.insert(chess::Piece::King, 20_000);
        map.insert(chess::Piece::Queen, 900);
        map.insert(chess::Piece::Rook, 500);
        map.insert(chess::Piece::Knight, 300);
        map.insert(chess::Piece::Bishop, 300);
        map.insert(chess::Piece::Pawn, 100);
        map
    };
    static ref INITIAL_VALUE: u16 = 23_900;
    static ref KNIGHT_VALUE_PER_SQUARE_WHITE: Vec<i32> = vec![
        -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15,
        10, 0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15,
        15, 10, 5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
    ];
    static ref KNIGHT_VALUE_PER_SQUARE_BLACK: Vec<i32> = KNIGHT_VALUE_PER_SQUARE_WHITE
        .iter()
        .copied()
        .rev()
        .collect::<Vec<i32>>();
    static ref PAWN_VALUD_PER_SQUARE_WHITE: Vec<i32> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, -20, -20, 10, 10, 5, 5, -5, -10, 0, 0, -10, -5, 5, 0, 0,
        0, 20, 20, 0, 0, 0, 5, 5, 10, 25, 25, 10, 5, 5, 10, 10, 20, 30, 30, 20, 10, 10, 50, 50, 50,
        50, 50, 50, 50, 50, 0, 0, 0, 0, 0, 0, 0, 0
    ];
    static ref PAWN_VALUD_PER_SQUARE_BLACK: Vec<i32> =
        PAWN_VALUD_PER_SQUARE_WHITE.iter().copied().rev().collect();
}
#[derive(Debug)]
struct BoardMaterial {
    white: u32,
    black: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EngineGameState {
    Draw,
    Win,
    Lose,
    Ongoing(isize),
}

impl EngineGameState {
    fn to_isize(&self) -> isize {
        match self {
            EngineGameState::Draw => 0,
            EngineGameState::Win => isize::MAX,
            EngineGameState::Lose => isize::MIN,
            EngineGameState::Ongoing(eval) => *eval,
        }
    }
    /// checks if `other` is better than `self`
    fn is_better(&self, other: &EngineGameState) -> bool {
        self.to_isize() < other.to_isize()
    }
}

trait MaterialSum {
    fn material_sum(&self) -> BoardMaterial;
}

impl MaterialSum for chess::Board {
    /// TODO: put this in a trait for `Board`
    fn material_sum(&self) -> BoardMaterial {
        let mut mat = BoardMaterial { white: 0, black: 0 };
        for sq in 0..64 {
            // SAFETY: squares are only created from 0 to 64 (not including 64)
            let sq = unsafe { Square::new(sq) };
            let piece_type = self.piece_on(sq);
            let color = self.color_on(sq);

            if let (Some(piece_type), Some(color)) = (piece_type, color) {
                // let (_, get_piece_worth) = PIECE_VALUE
                //     .iter()
                //     .find(|(p, _)| *p == piece_type)
                //     .expect("Imposible");

                let p = PIECE_VALUE_MAP.get(&piece_type).expect("You idiot");
                match color {
                    chess::Color::White => mat.white += *p,
                    chess::Color::Black => mat.black += *p,
                };
            }
        }
        mat
    }
}

impl Engine {
    pub fn new() -> Self {
        let board = Board::default();
        Self {
            board,
            current_pos_moves: MoveGen::new_legal(&board).collect(),
            current_best_move: None,
            your_side: chess::Color::White,
        }
    }

    pub fn from_fen<S: ToString>(s: S) -> Self {
        let s = s.to_string();
        let board = Board::from_str(&s).unwrap();

        // TODO: evaluate the engine
        Self {
            board,
            current_pos_moves: MoveGen::new_legal(&board).collect(),
            current_best_move: None,
            your_side: board.side_to_move(),
        }
    }

    fn regen_legal_moves(&mut self) {
        self.current_pos_moves = MoveGen::new_legal(&self.board).collect();
    }

    fn gen_board_legal_moves(&self, board: &Board) -> Vec<ChessMove> {
        MoveGen::new_legal(board).collect()
    }

    pub fn play_best_move(&mut self) {
        if let Some(mov) = self.current_best_move {
            self.board = self.board.make_move_new(mov);
            self.regen_legal_moves();
        };
    }

    pub fn play_moves(&mut self, moves: Vec<ChessMove>) {
        for m in moves.iter() {
            self.board = self.board.make_move_new(*m);
        }
        self.regen_legal_moves();
        self.your_side = self.board.side_to_move();
    }

    pub fn get_default_board(&self) -> Self {
        Engine::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
    pub fn get_side_to_move(&self) -> chess::Color {
        self.your_side
    }

    pub fn get_best_mov(&mut self) -> Option<ChessMove> {
        self.current_best_move
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn search(&mut self, depth: u8) -> EngineGameState {
        let moves = self.gen_board_legal_moves(&self.board);
        let mut best_eval: EngineGameState = EngineGameState::Lose;
        let mut best_mov = None;

        for mov in moves.iter() {
            if mov.to_string().as_str() == "f2g3" {
                println!("WHAT");
            }

            let piece = self
                .board
                .piece_on(mov.get_source())
                .expect("Should always have a source");

            let new_board = self.board.make_move_new(*mov);
            let mut eval = self.search_best_move(depth, &new_board);
            match eval {
                EngineGameState::Ongoing(ref mut e) => {
                    if chess::Piece::Pawn == piece {
                        // TODO: make this work
                    }
                },
                _ = {},
            };


            if best_eval.is_better(&eval) {
                best_eval = eval;
                let _ = best_mov.insert(*mov);
            }
        }
        self.current_best_move = best_mov;
        best_eval
    }

    fn search_best_move(&mut self, depth: u8, board: &Board) -> EngineGameState {
        if depth == 0 {
            return self.eval_board(board);
        }

        let moves = self.gen_board_legal_moves(board);

        // check if player has no more moves left
        if moves.is_empty() {
            // checks if the board is in check
            if board.checkers().0 != 0 {
                return EngineGameState::Lose;
            } else {
                // draw by stalement
                return EngineGameState::Draw;
            }
        }

        let mut best_eval = EngineGameState::Lose;

        for mov in moves.iter() {
            let new_board = board.make_move_new(*mov);
            let eval = self.search_best_move(depth - 1, &new_board);
            if best_eval.is_better(&eval) {
                best_eval = eval;
            }
        }

        best_eval
    }

    pub fn eval_board(&self, board: &Board) -> EngineGameState {
        match board.status() {
            chess::BoardStatus::Ongoing => {}
            chess::BoardStatus::Stalemate => return EngineGameState::Draw,
            chess::BoardStatus::Checkmate => {
                if self.get_side_to_move() == board.side_to_move() {
                    return EngineGameState::Lose;
                } else {
                    return EngineGameState::Win;
                }
            }
        };
        let mut white_weight = 0;
        let mut black_weight = 0;

        let mat_count = board.material_sum();
        white_weight += mat_count.white;
        black_weight += mat_count.black;

        let final_eval = match board.side_to_move() {
            chess::Color::White => black_weight as isize - white_weight as isize,
            chess::Color::Black => white_weight as isize - black_weight as isize,
        };

        // let final_eval = if self.get_side_to_move() != board.side_to_move() {
        //     -final_eval
        // } else {
        //     final_eval
        // };
        //
        EngineGameState::Ongoing(final_eval)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chess::ChessMove;

    use crate::engine::EngineGameState;

    use super::Engine;

    #[test]
    fn test_eval() {
        let engine = Engine::from_fen("8/8/4k3/8/2p5/8/B2P2K1/8 b - - 0 1");
        let board = engine.board;
        if let crate::engine::EngineGameState::Ongoing(eval) = engine.eval_board(&board) {
            assert_eq!(eval, 300);
        } else {
            panic!("Something went wrong here lol");
        }

        let mut engine = Engine::from_fen("2r1k3/8/8/5Q2/8/2K5/8/8 w - - 0 1");
        let board = engine.board;
        assert_eq!(EngineGameState::Ongoing(400), engine.eval_board(&board));
        engine.play_moves(vec![ChessMove::from_str("f5c8").unwrap()]);
        assert_eq!(EngineGameState::Ongoing(900), engine.eval_board(&board));
    }

    #[test]
    fn test_search_default_pos() {
        let mut engine = Engine::new();
        match engine.search(3) {
            super::EngineGameState::Draw => println!("draw"),
            super::EngineGameState::Win => println!("WIN"),
            super::EngineGameState::Lose => println!("LOSE"),
            super::EngineGameState::Ongoing(eval) => {
                println!("BEST move: {}", engine.get_best_mov().unwrap());
                println!("eval: {eval}");
            }
        }
    }

    #[test]
    fn test_best_move_capture_queen() {
        let mut engine =
            Engine::from_fen("rn2k1nr/ppp2ppp/8/3pp3/8/P1P3qb/1PQPPP2/RNB1KB2 w Qkq - 0 8");
        match engine.search(2) {
            super::EngineGameState::Draw => println!("draw"),
            super::EngineGameState::Win => println!("WIN"),
            super::EngineGameState::Lose => println!("LOSE"),
            super::EngineGameState::Ongoing(eval) => {
                println!("BEST move: {}", engine.get_best_mov().unwrap());
                println!("eval: {eval}");

                assert_eq!(
                    engine.get_best_mov(),
                    Some(ChessMove::from_str("f2g3").unwrap())
                );
            }
        }
    }

    #[test]
    fn test_search_fen() {
        let mut engine =
            Engine::from_fen("rnb1kbnr/pqpppppp/8/1p1N4/8/8/PPPPPPPP/R1BQKBNR b KQkq - 0 1");
        match engine.search(2) {
            super::EngineGameState::Draw => println!("draw"),
            super::EngineGameState::Win => println!("WIN"),
            super::EngineGameState::Lose => println!("LOSE"),
            super::EngineGameState::Ongoing(eval) => {
                println!("BEST move: {}", engine.get_best_mov().unwrap());
                println!("eval: {eval}");
            }
        }
    }
    #[test]
    fn test_check() {
        let mut engine =
            Engine::from_fen("rnb1k1nr/pppp1ppp/8/4p3/7q/RP6/2PPPPPP/1NBQKBNR w Kkq - 0 1");

        let board = engine.board;
        let moves = engine.gen_board_legal_moves(&board);

        dbg!(moves.iter().map(|d| d.to_string()).collect::<Vec<_>>());
    }
}
