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
