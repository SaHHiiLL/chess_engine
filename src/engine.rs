use std::{
    cell::RefCell,
    cmp::Ordering,
    rc::Rc,
    str::FromStr,
    sync::{Arc, Mutex, RwLock},
    time::Instant,
};

use chess::{BitBoard, Board, ChessMove, Color, MoveGen, Piece, Square};

use crate::{
    eval::Evaluation, game_phase::GamePhases, game_state::GameState, BoardMaterial, OpeningDatabase,
};

enum MoveType {
    Normal,
    Castle,
    Capture,
    EnPassant,
    Promotion,
    Invalid,
}

trait PieceOnBoardExt {
    fn get_piece(&self, sq: Square) -> Option<(chess::Piece, chess::Color)>;
}

impl PieceOnBoardExt for Board {
    fn get_piece(&self, sq: Square) -> Option<(chess::Piece, chess::Color)> {
        Some((self.piece_on(sq)?, self.color_on(sq)?))
    }
}

pub struct Engine {
    board: Board,
    best_move: Option<ChessMove>,
    side_playing: chess::Color,
    board_history: Vec<u64>,
    game_state: GameState,
    opening_database: OpeningDatabase,
}

impl FromStr for Engine {
    type Err = chess::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board = Board::from_str(s)?;
        let board_history = vec![board.get_hash()];
        Ok(Self {
            board,
            best_move: None,
            side_playing: board.side_to_move(),
            board_history,
            game_state: GameState::new(),
            opening_database: OpeningDatabase::new(),
        })
    }
}

impl Engine {
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn history(&self) -> &[u64] {
        &self.board_history
    }

    pub fn add_opening_db(&mut self, op_db: OpeningDatabase) {
        self.opening_database = op_db;
    }

    pub fn new() -> Self {
        let board = Board::default();
        let board_history = vec![board.get_hash()];
        Self {
            opening_database: OpeningDatabase::new(),
            board,
            best_move: None,
            side_playing: chess::Color::White,
            board_history,
            game_state: GameState::new(),
        }
    }

    fn gen_legal_moves(&self, board: &Board) -> Vec<ChessMove> {
        MoveGen::new_legal(board).collect()
    }

    /// Sorts moves based on if the move captures a piece or does a promotion
    /// if a move is a capture or promotion it will be sent higher in the list
    /// this will help the `alpha-beta` pruning
    fn sort_moves_in_place(
        &self,
        board: &Board,
        moves: &mut [ChessMove],
        game_state: &Rc<RefCell<GameState>>,
    ) {
        moves.sort_by(|d: &ChessMove, other: &ChessMove| {
            if game_state.as_ref().borrow().game_phases() == &GamePhases::EndGame {
                let other_board = board.make_move_new(*other);
                if other_board.checkers() != &BitBoard(0) {
                    return Ordering::Greater;
                }
                let curr_board = board.make_move_new(*d);
                if curr_board.checkers() != &BitBoard(0) {
                    return Ordering::Less;
                }
            }

            let square: Square = d.get_dest();
            let piece = board.piece_on(square);

            let square_other: Square = other.get_dest();
            let piece_other = board.piece_on(square_other);

            if piece.is_some() && piece_other.is_some() {
                return Ordering::Equal;
            }

            if piece.is_some() {
                return Ordering::Less;
            }

            if piece_other.is_some() {
                return Ordering::Greater;
            }

            if d.get_promotion().is_some() {
                return Ordering::Less;
            }
            if other.get_promotion().is_some() {
                return Ordering::Greater;
            }

            Ordering::Equal
        });
    }

    pub fn get_best_mov(&self) -> Option<ChessMove> {
        self.best_move
    }

    /// the engine will play the best move on to it's inner `Board`
    pub fn play_best_move(&mut self) {
        self.play_move(self.best_move.unwrap());
    }

    pub fn play_move(&mut self, mov: ChessMove) {
        let board = self.board.make_move_new(mov);
        self.board = board;
        self.board_history.push(board.get_hash());
        self.game_state.set_lastmove(mov);
        self.side_playing = self.board.side_to_move();
    }

    pub fn search(&mut self, depth: usize, game_state: GameState) -> isize {
        let game_state = Rc::new(RefCell::new(game_state));
        let legal_moves = self.gen_legal_moves(&self.board);
        let mut best_eval = -isize::MAX;

        for m in legal_moves.iter() {
            // make the move
            let next_board = self.board.make_move_new(*m);
            let next_eval = self.search_alpha_beta(
                depth,
                &next_board,
                -isize::MAX,
                isize::MAX,
                false,
                &game_state,
            );

            if next_eval > best_eval || self.best_move.is_none() {
                best_eval = next_eval;
                let _ = self.best_move.insert(*m);
            }
        }
        best_eval
    }

    /// TODO: make this better,
    /// BUG: the bot only plays the lines it can find, even if the other player has gone out of the line
    fn get_best_move_from_opening_database(&mut self) -> bool {
        if self.game_state.last_move().is_some() {
            match !self
                .opening_database
                .choose_opening_move(self.game_state.last_move().unwrap())
            {
                true => {
                    self.game_state.set_gamephases_middlegame();
                    return false;
                }
                false => (),
            }
        }

        if let Some(x) = self.opening_database.root().childern().keys().next() {
            let _ = self.best_move.insert(*x);
            self.opening_database
                .choose_opening_move(self.best_move.expect("SETTING IT RIGHT BEFORE THIS"));
            true
        } else {
            false
        }
    }

    pub fn search_iterative_deeping(&mut self, search_cancel_time: Instant) -> isize {
        let mut game_state = self.game_state.game_phases();
        if !self.opening_database.is_end()
            && *game_state == GamePhases::Opening
            && self.get_best_move_from_opening_database()
        {
            return 0;
        }
        println!("info starting Iterative Deepinnn");
        let mut best_eval = -isize::MAX;
        for x in 1..usize::MAX {
            let now = Instant::now();
            let game_state = self.game_state.clone();
            println!("info depth {}", x);
            if now >= search_cancel_time {
                break;
            }
            let eval = self.search(x, game_state);
            best_eval = best_eval.max(eval);
        }
        best_eval
    }

    fn search_alpha_beta(
        &mut self,
        depth: usize,
        board: &Board,
        mut alpha: isize,
        mut beta: isize,
        is_maximizing: bool,
        game_state: &Rc<RefCell<GameState>>,
    ) -> isize {
        if depth == 0 {
            return self.eval(board, game_state);
        }

        let mut best_eval = if is_maximizing {
            -isize::MAX
        } else {
            isize::MAX
        };
        // Move Ordering based on -- if a piece can be captured from the move it can be a good move
        // thus should be looked before
        let mut moves = self.gen_legal_moves(board);
        self.sort_moves_in_place(board, &mut moves, game_state);
        let moves = moves;
        if moves.is_empty() {
            return self.eval(board, game_state);
        }

        for m in moves.iter() {
            // make the move
            let next_board = board.make_move_new(*m);
            let eval = self.search_alpha_beta(
                depth - 1,
                &next_board,
                alpha,
                beta,
                !is_maximizing,
                game_state,
            );

            if is_maximizing {
                best_eval = best_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha {
                    break;
                }
            } else {
                best_eval = best_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break;
                }
            }
        }
        best_eval
    }

    pub fn eval(&self, board: &Board, game_state: &Rc<RefCell<GameState>>) -> isize {
        let mut eval = Evaluation::new(&self.board, game_state);
        let moves = self.gen_legal_moves(board);
        eval.eval_board(board, &self.board_history)
            .saturating_sub(eval.eval_mobility(&moves))
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::Engine;
    use crate::{eval::Evaluation, MaterialSumExt, OpeningDatabase};

    #[test]
    fn best_move_checkmate() {
        let mut engine =
            Engine::from_str("r1b1kb2/pppp1p1p/2n1p2n/8/3q2r1/8/PPPPKPP1/RNBQ1BNR b q - 0 11")
                .expect("IDIOT");
        let eval = engine.search(1);
        assert_eq!(engine.best_move.unwrap().to_string(), "d4e4");
    }

    #[test]
    fn test_best_move_capture_queen() {
        let mut engine =
            Engine::from_str("rn2k1nr/ppp2ppp/8/3pp3/8/P1P3qb/1PQPPP2/RNB1KB2 w Qkq - 0 8")
                .unwrap();
        let eval = engine.search(3);
        assert_eq!(engine.get_best_mov().unwrap().to_string().as_str(), "f2g3");
    }

    #[test]
    fn eval_board_black() {
        let engine = Engine::from_str("8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1").unwrap();

        let eval = Evaluation::new(&engine.board).eval_board(engine.board(), engine.history());
        assert!(eval > 0);
    }

    #[test]
    fn eval_board_white() {
        let engine = Engine::from_str("8/8/1P2K3/8/2n5/1q6/8/5k2 w - - 0 1").unwrap();
        let eval = Evaluation::new(&engine.board).eval_board(engine.board(), engine.history());
        assert!(eval < 0);
    }

    #[test]
    fn best_move_capture_free_pawn() {
        let mut engine =
            Engine::from_str("1nbqkbnr/1ppppppp/8/8/r1PP4/8/PP2PPPP/R1BQKBNR b KQk - 0 1").unwrap();
        let eval = engine.search(1);
        assert_eq!(engine.get_best_mov().unwrap().to_string().as_str(), "a4c4");
    }

    #[test]
    fn test_move_repetition() {
        let mut engine = Engine::new();
    }

    #[test]
    fn test_material_bitboard_sum() {
        let engine = Engine::new();
        let mat = engine.board.material_sum_bitboard();
        assert_eq!(mat.white, mat.black);
    }
}
