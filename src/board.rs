use crate::{piece::Piece, piece::PieceColor, position::{Position, BoardIndex}};

type BoardType = [[Option<Piece>; 8]; 8];
pub struct Board {
    board: BoardType,
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board {
            board: std::array::from_fn(|_| std::array::from_fn(|_| None)),
        };
        board.add_default_pieces();
        board
    }
}

impl Board {
    pub fn get_piece_by_position(&self, pos: Position) -> &Option<Piece> {
        let BoardIndex(file, rank) = pos.get_indices();
        &self.board[file][rank]
    }

    fn add_piece(&mut self, piece: Piece) {
        let BoardIndex(file, rank) = piece.get_position().get_indices();

        self.board[file][rank].take();
        self.board[file][rank] = Some(piece);
    }

    fn move_piece_from_to(&mut self, from: Position, to: Position) -> Result<(), &str> {
        let BoardIndex(from_file, from_rank) = from.get_indices();
        let BoardIndex(to_file, to_rank) = to.get_indices();

        let piece = self.board[from_file][from_rank].take();
        if piece.is_some() {
            let new_piece = piece.unwrap().copy_with_new_position(to);
            self.board[to_file][to_rank] = Some(new_piece);

            Ok(())
        } else {
            Err("Moved from position with no piece")
        }
    }

    pub fn get_flat_pieces(&self) -> Vec<&Piece> {
        self.board.iter().flatten().filter_map(Option::as_ref).collect::<Vec<&Piece>>()
        
    }

    fn add_default_pieces(&mut self) {
        self.add_piece(Piece::Rook(PieceColor::White, Position::new('a', '1')));
        self.add_piece(Piece::Knight(PieceColor::White, Position::new('b', '1')));
        self.add_piece(Piece::Bishop(PieceColor::White, Position::new('c', '1')));
        self.add_piece(Piece::Queen(PieceColor::White, Position::new('d', '1')));
        self.add_piece(Piece::King(PieceColor::White, Position::new('e', '1')));
        self.add_piece(Piece::Bishop(PieceColor::White, Position::new('f', '1')));
        self.add_piece(Piece::Knight(PieceColor::White, Position::new('g', '1')));
        self.add_piece(Piece::Rook(PieceColor::White, Position::new('h', '1')));
        for file in 'a'..='h' {
            self.add_piece(Piece::Pawn(PieceColor::White, Position::new(file, '2')));
            self.add_piece(Piece::Pawn(PieceColor::Black, Position::new(file, '7')));
        }
        self.add_piece(Piece::Rook(PieceColor::Black, Position::new('a', '8')));
        self.add_piece(Piece::Knight(PieceColor::Black, Position::new('b', '8')));
        self.add_piece(Piece::Bishop(PieceColor::Black, Position::new('c', '8')));
        self.add_piece(Piece::Queen(PieceColor::Black, Position::new('d', '8')));
        self.add_piece(Piece::King(PieceColor::Black, Position::new('e', '8')));
        self.add_piece(Piece::Bishop(PieceColor::Black, Position::new('f', '8')));
        self.add_piece(Piece::Knight(PieceColor::Black, Position::new('g', '8')));
        self.add_piece(Piece::Rook(PieceColor::Black, Position::new('h', '8')));
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
                write!(f, "{}", piece_char)?;
            }
            writeln!(f)?;
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
        board.add_piece(Piece::Queen(PieceColor::Black, Position::new('a', '1')));

        assert!(board.board[0][0].is_some());
    }

    #[test]
    fn it_returns_a_piece_by_position() {
        let mut board = create_empty_board();
        board.board[0][0] = Some(Piece::Queen(PieceColor::Black, Position::new('a', '1')));

        let piece = board.get_piece_by_position(Position('a', '1'));

        assert!(piece.is_some());
    }

    #[test]
    fn it_returns_a_piece_by_position_from_default_board() {
        let board = Board::default();

        let piece = board
            .get_piece_by_position(Position('h', '1'));

        assert!(piece.is_some());
    }

    #[test]
    fn it_moves_a_piece() {
        let mut board = Board::default();

        let res = board.move_piece_from_to(Position('a', '1'), Position('b', '1'));
        assert!(res.is_ok());

        let old_square = board.get_piece_by_position(Position('a', '1'));
        assert!(old_square.is_none());

        let new_square = board.get_piece_by_position(Position('b', '1'));
        assert!(new_square.is_some())
    }

    #[test]
    fn it_gets_pieces() {
        let mut board = create_empty_board();
        board.add_piece(Piece::King(PieceColor::Black, Position('e', '8')));
        board.add_piece(Piece::Bishop(PieceColor::White, Position('c', '1')));

        let pieces = board.get_flat_pieces();
        assert_eq!(pieces.len(), 2)
    }

    #[test]
    fn it_returns_error_when_moving_from_position_with_no_piece() {
        let mut board = Board::default();
        let res = board.move_piece_from_to(Position('e', '4'), Position('e', '5'));
        assert!(res.is_err())
    }

    fn create_empty_board() -> Board {
        Board {
            board: std::array::from_fn(|_| std::array::from_fn(|_| None)),
        }
    }
}
