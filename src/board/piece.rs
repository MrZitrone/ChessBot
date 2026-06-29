#[repr(u8)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

#[repr(u8)]
pub enum Color {
    White = 1,
    Black = 0,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}
