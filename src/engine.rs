use std::{
    cmp::Ordering,
    default,
    str::FromStr,
    sync::{Arc, Mutex},
};

use chess::{Board, ChessMove, Color, MoveGen, Piece, Square};

use crate::{eval::Evaluation, BoardMaterial};

#[derive(Clone)]
pub enum GamePhases {
    Opening,
    MiddleGame,
    EndGame,
}

impl Default for GamePhases {
    fn default() -> Self {
        Self::Opening
    }
}

impl GamePhases {
    fn update(&mut self, mat: BoardMaterial, board: &Board) {
        if board.pieces(Piece::Queen).0 == 0 {
            self.set_endgame();
        } else {
            self.set_middlegame();
        }
    }

    fn set_endgame(&mut self) {
        *self = GamePhases::EndGame
    }

    fn set_middlegame(&mut self) {
        *self = GamePhases::EndGame
    }
}

#[derive(Clone)]
struct GameState {
    game_phases: GamePhases,
}

impl GameState {
    fn new() -> Self {
        Self {
            game_phases: GamePhases::Opening,
        }
    }
}

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

/// Gets the piece the move
trait MovePiecesExt {
    fn move_type(&self, chess_move: &ChessMove);
}

impl MovePiecesExt for Board {
    fn move_type(&self, chess_move: &ChessMove) {
        let source = chess_move.get_source();
        let dest = chess_move.get_dest();
        todo!()
    }
}

pub struct Engine {
    board: Board,
    best_move: Option<ChessMove>,
    side_playing: chess::Color,
    board_history: Vec<u64>,
    game_state: GameState,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Engine {
    type Err = chess::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board = Board::from_str(s)?;
        Ok(Self {
            board,
            best_move: None,
            side_playing: board.side_to_move(),
            board_history: vec![],
            game_state: GameState::new(),
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

    pub fn new() -> Self {
        Self {
            board: Board::default(),
            best_move: None,
            side_playing: chess::Color::White,
            board_history: vec![],
            game_state: GameState::new(),
        }
    }

    fn gen_legal_moves(&self, board: &Board) -> Vec<ChessMove> {
        MoveGen::new_legal(board).collect()
    }

    /// Sorts moves based on if the move captures a piece or does a promotion
    /// if a move is a capture or promotion it will be sent higher in the list
    /// this will help the `alpha-beta` pruning
    fn sort_moves_in_place(&self, board: &Board, moves: &mut [ChessMove]) {
        moves.sort_by(|d: &ChessMove, other: &ChessMove| {
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

    pub fn play_best_move(&mut self) {
        if let Some(mov) = self.best_move {
            self.board = self.board.make_move_new(mov);
            self.board_history.clear();
        };
    }

    pub fn play_moves(&mut self, moves: Vec<ChessMove>) {
        self.board_history.clear();
        for m in moves.iter() {
            let board = self.board.make_move_new(*m);
            self.board = board;
            self.board_history.push(board.get_hash());
        }
        self.side_playing = self.board.side_to_move();
    }

    pub fn search_slow(&mut self, depth: usize) -> isize {
        let legal_moves = self.gen_legal_moves(&self.board);
        let mut best_eval = -isize::MAX;

        for m in legal_moves.iter() {
            // make the move
            let next_board = self.board.make_move_new(*m);
            let next_eval = self.search_minimax(depth, &next_board, false);

            if next_eval > best_eval || self.best_move.is_none() {
                best_eval = next_eval;
                let _ = self.best_move.insert(*m);
            }
        }
        best_eval
    }

    pub fn search(&mut self, depth: usize) -> isize {
        let legal_moves = self.gen_legal_moves(&self.board);
        let mut best_eval = -isize::MAX;

        for m in legal_moves.iter() {
            // make the move
            let next_board = self.board.make_move_new(*m);
            let next_eval =
                self.search_alpha_beta(depth, &next_board, -isize::MAX, isize::MAX, false);

            if next_eval > best_eval || self.best_move.is_none() {
                best_eval = next_eval;
                let _ = self.best_move.insert(*m);
            }
        }
        best_eval
    }

    fn search_iterative_deeping(&mut self, search_cancel: Arc<bool>) -> isize {
        let mut best_eval = -isize::MAX;
        for x in 1..usize::MAX {
            let eval = self.search(x);
            best_eval = best_eval.max(eval);
            if search_cancel.clone() == true.into() {
                break;
            }
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
    ) -> isize {
        if depth == 0 {
            return self.eval(board);
        }

        let mut best_eval = if is_maximizing {
            -isize::MAX
        } else {
            isize::MAX
        };
        // Move Ordering based on -- if a piece can be captured from the move it can be a good move
        // thus should be looked before
        let mut moves = self.gen_legal_moves(board);
        self.sort_moves_in_place(board, &mut moves);
        let moves = moves;
        if moves.is_empty() {
            return self.eval(board);
        }

        for m in moves.iter() {
            // make the move
            let next_board = board.make_move_new(*m);
            let eval = self.search_alpha_beta(depth - 1, &next_board, alpha, beta, !is_maximizing);

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

    fn search_minimax(&mut self, depth: usize, board: &Board, is_maximizing: bool) -> isize {
        if depth == 0 {
            return if board.checkers().0 != 0 {
                self.search_minimax(depth + 1, board, !is_maximizing)
            } else {
                self.eval(board)
            };
        }

        let mut best_eval = if is_maximizing {
            -isize::MAX
        } else {
            isize::MAX
        };

        let moves = self.gen_legal_moves(board);

        if moves.is_empty() {
            return self.eval(board);
        }

        for m in moves.iter() {
            // make the move
            let next_board = board.make_move_new(*m);
            let next_eval = self.search_minimax(depth - 1, &next_board, !is_maximizing);

            if is_maximizing {
                best_eval = best_eval.max(next_eval);
            } else {
                best_eval = best_eval.min(next_eval);
            }
        }
        best_eval
    }

    pub fn eval(&self, board: &Board) -> isize {
        let eval = Evaluation::new(&self.board);
        let moves = self.gen_legal_moves(board);
        eval.eval_board(board, &self.board_history)
            .saturating_sub(eval.eval_mobility(&moves))
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::Engine;
    use crate::{eval::Evaluation, MaterialSumExt};

    #[test]
    fn best_move_checkmate() {
        let mut engine =
            Engine::from_str("r1b1kb2/pppp1p1p/2n1p2n/8/3q2r1/8/PPPPKPP1/RNBQ1BNR b q - 0 11")
                .expect("IDIOT");
        let eval = engine.search(1);
        assert_eq!(eval, isize::MAX);
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
        assert!(eval >= 1000);
    }

    #[test]
    fn eval_board_white() {
        let engine = Engine::from_str("8/8/1P2K3/8/2n5/1q6/8/5k2 w - - 0 1").unwrap();
        let eval = Evaluation::new(&engine.board).eval_board(engine.board(), engine.history());
        assert!(eval <= -1100);
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
