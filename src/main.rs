use crate::board::Board;

mod board;

mod piece;

mod ui;

mod position;
mod castling_rights;


fn main() {
    let brd = Board::default();
    print!("{}", brd);
}
