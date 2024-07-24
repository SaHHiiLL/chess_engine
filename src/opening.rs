use std::{collections::VecDeque, path::Path};

use chess::ChessMove;

struct Opening {
    file_path: Box<Path>,
    lines: VecDeque<ChessMove>,
}
