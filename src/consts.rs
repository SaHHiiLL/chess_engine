#![feature(const_evaluatable_checked, const_for)]
use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref PIECE_VALUE_MAP: HashMap<chess::Piece, u16> = {
        let mut map = HashMap::new();
        map.insert(chess::Piece::King, 20_000);
        map.insert(chess::Piece::Queen, 900);
        map.insert(chess::Piece::Rook, 500);
        map.insert(chess::Piece::Knight, 300);
        map.insert(chess::Piece::Bishop, 330);
        map.insert(chess::Piece::Pawn, 100);
        map
    };

    pub static ref KNIGHT_VALUE_PER_SQUARE_WHITE: Vec<isize> = vec![
        -50,-40,-30,-30,-30,-30,-40,-50,
        -40,-20,  0,  0,  0,  0,-20,-40,
        -30,  0, 10, 15, 15, 10,  0,-30,
        -30,  5, 15, 20, 20, 15,  5,-30,
        -30,  0, 15, 20, 20, 15,  0,-30,
        -30,  5, 10, 15, 15, 10,  5,-30,
        -40,-20,  0,  5,  5,  0,-20,-40,
        -50,-40,-30,-30,-30,-30,-40,-50,
    ];
    pub static ref KNIGHT_VALUE_PER_SQUARE_BLACK: Vec<isize> = KNIGHT_VALUE_PER_SQUARE_WHITE
        .iter()
        .copied()
        .rev()
        .collect();
    pub static ref PAWN_VALUE_PER_SQUARE_WHITE: Vec<isize> = vec![
         0,  0,  0,  0,  0,  0,  0,  0,
        50, 50, 50, 50, 50, 50, 50, 50,
        10, 10, 20, 30, 30, 20, 10, 10,
         5,  5, 10, 25, 25, 10,  5,  5,
         0,  0,  0, 20, 20,  0,  0,  0,
         5, -5,-10,  0,  0,-10, -5,  5,
         5, 10, 10,-20,-20, 10, 10,  5,
         0,  0,  0,  0,  0,  0,  0,  0
    ];
    pub static ref PAWN_VALUE_PER_SQUARE_BLACK: Vec<isize> =
        PAWN_VALUE_PER_SQUARE_WHITE.iter().copied().rev().collect();
    pub static ref BISHOP_VALUE_PER_SQUARE_WHITE: Vec<isize> = vec![
        -20,-10,-10,-10,-10,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5, 10, 10,  5,  0,-10,
        -10,  5,  5, 10, 10,  5,  5,-10,
        -10,  0, 10, 10, 10, 10,  0,-10,
        -10, 10, 10, 10, 10, 10, 10,-10,
        -10,  5,  0,  0,  0,  0,  5,-10,
        -20,-10,-10,-10,-10,-10,-10,-20,
    ];
    pub static ref BISHOP_VALUE_PER_SQUARE_BLACK: Vec<isize> = BISHOP_VALUE_PER_SQUARE_WHITE
        .iter()
        .copied()
        .rev()
        .collect();
    pub static ref KING_VALUE_PER_SQUARE_MIDDLE_GAME_WHITE: Vec<isize> = vec![
        -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40,
        -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40,
        -40, -30, -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20,
        30, 10, 0, 0, 10, 30, 20
    ];
    pub static ref KING_VALUE_PER_SQUARE_MIDDLE_GAME_BLACK: Vec<isize> =
        KING_VALUE_PER_SQUARE_MIDDLE_GAME_WHITE
            .iter()
            .copied()
            .rev()
            .collect();
    pub static ref QUEEN_VALUE_PER_SQUARE_WHITE: Vec<isize> = vec![
        -20,-10,-10, -5, -5,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5,  5,  5,  5,  0,-10,
         -5,  0,  5,  5,  5,  5,  0, -5,
          0,  0,  5,  5,  5,  5,  0, -5,
        -10,  5,  5,  5,  5,  5,  0,-10,
        -10,  0,  5,  0,  0,  0,  0,-10,
        -20,-10,-10, -5, -5,-10,-10,-20
    ];
    pub static ref QUEEN_VALUE_PER_SQUARE_BLACK: Vec<isize> =
        QUEEN_VALUE_PER_SQUARE_WHITE.iter().copied().rev().collect();
    pub static ref ROOK_VALUE_PER_SQUARE_WHITE: Vec<isize> = vec![
  0,  0,  0,  0,  0,  0,  0,  0,
  5, 10, 10, 10, 10, 10, 10,  5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
  0,  0,  0,  5,  5,  0,  0,  0
    ];
    pub static ref ROOK_VALUE_PER_SQUARE_BLACK: Vec<isize> =
        ROOK_VALUE_PER_SQUARE_WHITE.iter().copied().rev().collect();

    pub static ref KING_VALUE_PER_SQUARE_ENDGAME_WHITE: Vec<isize> = vec![
        -50,-40,-30,-20,-20,-30,-40,-50,
        -30,-20,-10,  0,  0,-10,-20,-30,
        -30,-10, 20, 30, 30, 20,-10,-30,
        -30,-10, 30, 40, 40, 30,-10,-30,
        -30,-10, 30, 40, 40, 30,-10,-30,
        -30,-10, 20, 30, 30, 20,-10,-30,
        -30,-30,  0,  0,  0,  0,-30,-30,
        -50,-30,-30,-30,-30,-30,-30,-50
];
    pub static ref KING_VALUE_PER_SQUARE_ENDGAME_BLACK: Vec<isize> = KING_VALUE_PER_SQUARE_ENDGAME_WHITE.iter().copied().rev().collect();
    pub static ref FEN_STRING: Vec<String> = vec![
        String::from("r6r/1b2k1bq/8/8/7B/8/8/R3K2R b KQ - 3 2"),
        String::from("8/8/8/2k5/2pP4/8/B7/4K3 b - d3 0 3"),
        String::from("r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 2 2"),
        String::from("r3k2r/p1pp1pb1/bn2Qnp1/2qPN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQkq - 3 2"),
        String::from("2kr3r/p1ppqpb1/bn2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQ - 3 2"),
        String::from("rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R w KQ - 3 9"),
        String::from("2r5/3pk3/8/2P5/8/2K5/8/8 w - - 5 4"),
        String::from("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8"),
        String::from("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10"),
        String::from("3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1"),
        String::from("8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1"),
        String::from("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1"),
        String::from("5k2/8/8/8/8/8/8/4K2R w K - 0 1"),
        String::from("3k4/8/8/8/8/8/8/R3K3 w Q - 0 1"),
        String::from("r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1"),
        String::from("r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1"),
        String::from("2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1"),
        String::from("8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1"),
        String::from("4k3/1P6/8/8/8/8/K7/8 w - - 0 1"),
        String::from("8/P1k5/K7/8/8/8/8/8 w - - 0 1"),
        String::from("K1k5/8/P7/8/8/8/8/8 w - - 0 1"),
        String::from("8/k1P5/8/1K6/8/8/8/8 w - - 0 1"),
        String::from("8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1"),
    ];
}
const fn reverse<const N: usize>(arr: [isize; N]) -> [isize; N] {
    let mut reversed = [0; N];
    let mut i = 0;
    while i < N {
        reversed[i] = arr[N - 1 - i];
        i += 1;
    }
    reversed
}

pub const KING_MIDDLE_WHITE: [isize; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40,
    -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40, -40, -30,
    -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0,
    10, 30, 20,
];

pub const KING_MIDDLE_BLACK: [isize; 64] = reverse(KING_MIDDLE_WHITE);

pub const KING_ENDGAME_WHITE: [isize; 64] = [
    -50, -40, -30, -20, -20, -30, -40, -50, -30, -20, -10, 0, 0, -10, -20, -30, -30, -10, 20, 30,
    30, 20, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30,
    -10, 20, 30, 30, 20, -10, -30, -30, -30, 0, 0, 0, 0, -30, -30, -50, -30, -30, -30, -30, -30,
    -30, -50,
];
pub const KING_ENDGAME_BLACK: [isize; 64] = reverse(KING_ENDGAME_WHITE);

pub const INITIAL_BOARD_VALUE: u16 = 23_900;
