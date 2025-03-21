use piecetable::PieceTable;
use std::path::Path;

fn main() {
    let mut pieceTable = PieceTable::from_file(Path::new("text.txt")).unwrap();

    
}
