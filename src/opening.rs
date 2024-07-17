use std::path::Path;

use chess::ChessMove;

struct Opening {
    file_path: Box<Path>,
    lines: Vec<ChessMove>,
}
