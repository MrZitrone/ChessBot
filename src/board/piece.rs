pub enum Color {
    White = 1,
    Black = 0,
}

pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl Piece {
    pub const fn new(piece_type: PieceType, color: Color) -> Self {
        Piece { piece_type, color }
    }
}
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

// u8 for piece value for evaluation, not the best but it works
const fn piece_value(kind: PieceType) -> u16 {
    match kind {
        PieceType::Pawn => 100,
        PieceType::Knight => 300,
        PieceType::Bishop => 300,
        PieceType::Rook => 500,
        PieceType::Queen => 900,
        PieceType::King => 2000,
    }
}
