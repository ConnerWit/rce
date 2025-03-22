use piecetable::PieceTable;
use std::{
    path::Path,
    fs,
    io::Write
};

fn main() {

    {
        let mut file = fs::File::create("test/textnew.txt").unwrap();
        file.write_all(b"hello cro").unwrap();
    }

    let mut first_piece_table = PieceTable::from_file(Path::new("test/textnew.txt")).unwrap();
    first_piece_table.insert(" fortnite", 9);
    let _file_length: usize = first_piece_table.save_file().unwrap();

    let mut piece_table = PieceTable::from_file(Path::new("test/text.txt")).unwrap();
    
    println!("before insert: {}", piece_table.display_result().unwrap());

    let buffer = "this text got placed inbetween ";
    piece_table.insert(buffer, 11);

    println!("after insert: {}", piece_table.display_result().unwrap());

    piece_table.delete_char(9);

    println!("after deletion: {}", piece_table.display_result().unwrap());

    let _new_file_length = piece_table.save_to_file(Path::new("test/text.txt")).unwrap();


}
