use crate::position::Position;

pub enum Piece {
    King(PieceColor, Position),
    Queen(PieceColor, Position),
    Rook(PieceColor, Position),
    Bishop(PieceColor, Position),
    Knight(PieceColor, Position),
    Pawn(PieceColor, Position),
}
#[derive(Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}

impl Piece {
    pub fn get_char(&self) -> char {
        match self {
            Piece::King(color, _) => match color {
                PieceColor::White => '♔',
                PieceColor::Black => '♚',
            },
            Piece::Queen(color, _) => match color {
                PieceColor::White => '♕',
                PieceColor::Black => '♛',
            },
            Piece::Rook(color, _) => match color {
                PieceColor::White => '♜',
                PieceColor::Black => '♖',
            },
            Piece::Bishop(color, _) => match color {
                PieceColor::White => '♝',
                PieceColor::Black => '♗',
            },
            Piece::Knight(color, _) => match color {
                PieceColor::White => '♞',
                PieceColor::Black => '♘',
            },
            Piece::Pawn(color, _) => match color {
                PieceColor::White => '♟',
                PieceColor::Black => '♙',
            },
        }
    }
    pub fn get_position(&self) -> &Position {
        match self {
            Piece::King(_, pos)
            | Piece::Queen(_, pos)
            | Piece::Rook(_, pos)
            | Piece::Bishop(_, pos)
            | Piece::Knight(_, pos)
            | Piece::Pawn(_, pos) => pos,
        }
    }
    pub fn get_color(&self) -> &PieceColor {
        match self {
            Piece::King(color, _)
            | Piece::Queen(color, _)
            | Piece::Rook(color, _)
            | Piece::Bishop(color, _)
            | Piece::Knight(color, _)
            | Piece::Pawn(color, _) => color,
        }
    }

    pub fn copy_with_new_position(&self, new_pos: Position) -> Piece {
        match self {
            Piece::King(color, _) => Piece::King(*color, new_pos),
            Piece::Queen(color, _) => Piece::Queen(*color, new_pos),
            Piece::Rook(color, _) => Piece::Rook(*color, new_pos),
            Piece::Bishop(color, _) => Piece::Bishop(*color, new_pos),
            Piece::Knight(color, _) => Piece::Knight(*color, new_pos) , 
            Piece::Pawn(color, _) => Piece::Pawn(*color, new_pos),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_piece_and_returns_char() {
        let piece = Piece::King(PieceColor::Black, Position('e', '8'));
        assert_eq!(piece.get_char(), '♚')
    }
}
