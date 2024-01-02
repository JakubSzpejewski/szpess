// use crate::board::Board;
// use iced::{
//     executor,
//     widget::{Button, Column, Container, Row},
//     Application, Command, Element, Theme,
// };

// struct ChessBoardFlags {
//     board: Board,
// }

// pub struct ChessBoard {
//     board: Board,
// }

// impl Application for ChessBoard {
//     type Message = ();
//     type Executor = executor::Default;
//     type Flags = ChessBoardFlags;
//     type Theme = Theme;
    
//     fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
//         (
//             ChessBoard {
//                 board: flags.board,
//             },
//             Command::none(),
//         )
//     }

//     fn title(&self) -> String {
//         String::from("Chess")
//     }

//     fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
//         // This application has no interactions
//         Command::none()
//     }

//     fn view(&self) -> Element<Self::Message> {
//         let mut column = Column::new();

//         for board_row in &self.board.board {
//             let mut row: Row<'_, Self::Message> = Row::new();

//             for _ in board_row {
//                 row = row.push(Button::new("a"))
//             }
//             column = column.push(row);
//         }
//         Container::new(column).into()
//     }
// }
