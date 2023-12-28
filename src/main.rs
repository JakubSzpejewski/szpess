mod board;
use iced::{Settings, Application};

use crate::board::Board;

mod piece;

mod ui;

mod position;


fn main() {
    let brd = Board::default();

    print!("{}", brd);
}
