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

pub type Piece = (PieceType, PieceColor);




pub fn get_piece_char(piece: &Option<Piece>) -> char {
    match piece {
        Some((PieceType::King, PieceColor::White)) => '♔',
        Some((PieceType::King, PieceColor::Black)) => '♚',
        Some((PieceType::Queen, PieceColor::White)) => '♕',
        Some((PieceType::Queen, PieceColor::Black)) => '♛',
        Some((PieceType::Rook, PieceColor::White)) => '♖',
        Some((PieceType::Rook, PieceColor::Black)) => '♜',
        Some((PieceType::Bishop, PieceColor::White)) => '♗',
        Some((PieceType::Bishop, PieceColor::Black)) => '♝',
        Some((PieceType::Knight, PieceColor::White)) => '♘',
        Some((PieceType::Knight, PieceColor::Black)) => '♞',
        Some((PieceType::Pawn, PieceColor::White)) => '♙',
        Some((PieceType::Pawn, PieceColor::Black)) => '♟',
        None => ' ',
    }
}