use crate::{piece::Piece, piece::PieceColor, piece::PieceType, position::Position};

pub struct Board {
    board: [[Option<Piece>; 8]; 8],
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board {
            board: std::array::from_fn(|_| std::array::from_fn(|_| None)),
        };
        board.add_piece(Piece::new(
            PieceType::Rook,
            PieceColor::White,
            Position::new('a', '1'),
        ));
        board.add_piece(Piece::new(
            PieceType::Knight,
            PieceColor::White,
            Position::new('b', '1'),
        ));
        board.add_piece(Piece::new(
            PieceType::Bishop,
            PieceColor::White,
            Position::new('c', '1'),
        ));
        board.add_piece(Piece::new(
            PieceType::Queen,
            PieceColor::White,
            Position::new('d', '1'),
        ));
        board.add_piece(Piece::new(
            PieceType::King,
            PieceColor::White,
            Position::new('e', '1'),
        ));
        board.add_piece(Piece::new(
            PieceType::Bishop,
            PieceColor::White,
            Position::new('f', '1'),
        ));
        board.add_piece(Piece::new(
            PieceType::Knight,
            PieceColor::White,
            Position::new('g', '1'),
        ));
        board.add_piece(Piece::new(
            PieceType::Rook,
            PieceColor::White,
            Position::new('h', '1'),
        ));
        for file in ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'] {
            board.add_piece(Piece::new(
                PieceType::Pawn,
                PieceColor::White,
                Position::new(file, '2'),
            ));
            board.add_piece(Piece::new(
                PieceType::Pawn,
                PieceColor::Black,
                Position::new(file, '7'),
            ));
        }
        board.add_piece(Piece::new(
            PieceType::Rook,
            PieceColor::Black,
            Position::new('a', '8'),
        ));
        board.add_piece(Piece::new(
            PieceType::Knight,
            PieceColor::Black,
            Position::new('b', '8'),
        ));
        board.add_piece(Piece::new(
            PieceType::Bishop,
            PieceColor::Black,
            Position::new('c', '8'),
        ));
        board.add_piece(Piece::new(
            PieceType::Queen,
            PieceColor::Black,
            Position::new('d', '8'),
        ));
        board.add_piece(Piece::new(
            PieceType::King,
            PieceColor::Black,
            Position::new('e', '8'),
        ));
        board.add_piece(Piece::new(
            PieceType::Bishop,
            PieceColor::Black,
            Position::new('f', '8'),
        ));
        board.add_piece(Piece::new(
            PieceType::Knight,
            PieceColor::Black,
            Position::new('g', '8'),
        ));
        board.add_piece(Piece::new(
            PieceType::Rook,
            PieceColor::Black,
            Position::new('h', '8'),
        ));
        board
    }
}

impl Board {
    pub fn get_piece_by_position(&self, pos: Position) -> &Option<Piece> {
        let (file, rank) = pos.get_index_values();
        &self.board[file][rank]
    }

    fn add_piece(&mut self, piece: Piece) -> () {
        let (file, rank) = piece.pos.get_index_values();

        self.board[file][rank].take();
        self.board[file][rank] = Some(piece);
    }

    fn move_piece_from_to(&mut self, from: Position, to: Position) {
        let (from_file, from_rank) = from.get_index_values();
        let (to_file, to_rank) = to.get_index_values();

        let piece = self.board[from_file][from_rank].take();
        let mut piece = piece.unwrap();
        piece.pos = to;
        self.board[to_file][to_rank] = Some(piece);
    }

    pub fn get_flat_pieces(&self) -> Vec<&Piece> {
        let mut ret: Vec<&Piece> = Vec::new();
        for file in &self.board {
            for piece in file {
                match piece {
                    Some(v) => ret.push(v),
                    _ => (),
                }
            }
        }
        ret
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // because we store board as [file][rank] ([column][row]) we can't simply iterate over board, because we can't return to previous lines
        // starting from the last to show white at bottom
        for column in (0..8).rev() {
            for row in 0..8 {
                let piece = &self.board[row][column];
                let piece_char = match piece {
                    Some(v) => v.get_char(),
                    None => ' ',
                };
                write!(f, "{}", piece_char);
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_piece_to_a_board() {
        let mut board = create_empty_board();
        board.add_piece(Piece::new(
            PieceType::Queen,
            PieceColor::Black,
            Position::new('a', '1'),
        ));

        assert!(match board.board[0][0] {
            Some(_) => true,
            None => false,
        });
    }

    #[test]
    fn it_returns_a_piece_by_position() {
        let mut board = create_empty_board();
        board.board[0][0] = Some(Piece::new(
            PieceType::Queen,
            PieceColor::Black,
            Position::new('a', '1'),
        ));

        let piece = board.get_piece_by_position(Position('a', '1'));

        assert!(match piece {
            Some(_) => true,
            None => false,
        });
    }

    #[test]
    fn it_returns_a_piece_by_position_from_default_board() {
        let board = Board::default();

        let piece = board
            .get_piece_by_position(Position('h', '1'))
            .as_ref()
            .unwrap();

        assert!(match piece.r#type {
            PieceType::Rook => true,
            _ => false,
        })
    }

    #[test]
    fn it_moves_a_piece() {
        let mut board = Board::default();

        board.move_piece_from_to(Position('a', '1'), Position('b', '1'));

        let old_square = board.get_piece_by_position(Position('a', '1'));
        assert!(match old_square {
            Some(_) => false,
            None => true,
        });

        let new_square = board.get_piece_by_position(Position('b', '1'));
        assert!(match new_square {
            Some(_) => true,
            None => false,
        })
    }

    #[test]
    fn it_gets_pieces() {
        let mut board = create_empty_board();
        board.add_piece(Piece::new(PieceType::King, PieceColor::Black, Position('e', '8')));
        board.add_piece(Piece::new(PieceType::Bishop, PieceColor::White, Position('c', '1')));

        let pieces = board.get_flat_pieces();
        assert_eq!(pieces.len(), 2)
    }

    fn create_empty_board() -> Board {
        Board {
            board: std::array::from_fn(|_| std::array::from_fn(|_| None)),
        }
    }
}
