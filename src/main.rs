mod board;
use crate::board::Board;

mod piece;


fn main() {
    let board = Board::default();
    println!("{:}", board);
}
