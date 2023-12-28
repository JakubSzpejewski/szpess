use crate::{piece::Piece, piece::PieceColor, piece::PieceType, position::Position};

pub struct Board {
    board: [[Option<Piece>; 8]; 8],
}

impl Default for Board {
    fn default() -> Self {
        let mut a = Board {
            board: std::array::from_fn(|_| std::array::from_fn(|_| None)),
        };
        a.add_piece(Piece::new(
            PieceType::Rook,
            PieceColor::White,
            Position::new('a', '1'),
        ));
        a
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
        self.board[rank][file] = Some(piece);
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.board {
            for piece in row {
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
        board.board[0][0] = Some(Piece::new(PieceType::Queen, PieceColor::Black, Position::new('a', '1')));

        let piece = board.get_piece_by_position(Position('a', '1'));

        assert!(match piece {
            Some(_) => true,
            None => false
        });
    }

    #[test]
    fn it_returns_a_piece_by_position_from_default_board() {
        let board = Board::default();

        let piece = board.get_piece_by_position(Position('a', '1')).as_ref().unwrap();

        assert!(match piece.r#type {
            PieceType::Rook => true,
            _ => false
        })

    }

    fn create_empty_board() -> Board {
        Board {
            board: std::array::from_fn(|_| std::array::from_fn(|_| None)),
        }
    }
}
