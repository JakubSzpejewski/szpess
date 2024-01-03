
use crate::{board::Board};

mod board;

mod piece;

mod ui;

mod position;


fn main() {
    let brd = Board::default();
    print!("{}", brd);
}
