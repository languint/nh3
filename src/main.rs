use crate::chess::bitboard::Bitboard;

mod chess;

fn main() {
    let bb = Bitboard::new(0xFFFF);
    println!("{}", bb);
}
