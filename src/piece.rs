use crate::position::Position;

pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}
pub enum PieceColor {
    White,
    Black,
}

pub struct Piece {
    pub r#type: PieceType,
    pub color: PieceColor,
    pub pos: Position,
}

impl Piece {
    pub fn new(r#type: PieceType, color: PieceColor, pos: Position) -> Self {
        Piece { r#type, color, pos }
    }

    pub fn get_char(&self) -> char {
        match self.color {
            PieceColor::White => match self.r#type {
                PieceType::King => '♔',
                PieceType::Queen => '♕',
                PieceType::Rook => '♖',
                PieceType::Bishop => '♗',
                PieceType::Knight => '♘',
                PieceType::Pawn => '♙',
            },
            PieceColor::Black => match self.r#type {
                PieceType::King => '♚',
                PieceType::Queen => '♛',
                PieceType::Rook => '♜',
                PieceType::Bishop => '♝',
                PieceType::Knight => '♞',
                PieceType::Pawn => '♟',
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_piece_and_returns_char() {
        let piece = Piece::new(PieceType::King, PieceColor::Black, Position('e', '8'));
        assert_eq!(piece.get_char(), '♚')
    }
}
