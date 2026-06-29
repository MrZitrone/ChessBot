use super::bitboard::Board;
use super::piece::{Color, PieceType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveFlags {
    Quiet,
    DoublePush,
    KingsideCastle,
    QueensideCastle,
    Capture,
    EnPassant,
    Promotion,
    PromoCapture,
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub flags: MoveFlags,
    pub promotion: Option<PieceType>,
}

impl Board {
    pub fn generate_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(218); // max legal moves in any position
        moves
    }
}
