use crate::board::piece::Color;

type Bitboard = u64;

// occupancy indices
const WHITE_OCC: usize = Color::White as usize;
const BLACK_OCC: usize = Color::Black as usize;
const BOTH_OCC: usize = 2;

// casteling rights
pub const CASTLE_WHITE_KING_SIDE: u8 = 0b0001;
pub const CASTLE_WHITE_QUEEN_SIDE: u8 = 0b0010;
pub const CASTLE_BLACK_KING_SIDE: u8 = 0b0100;
pub const CASTLE_BLACK_QUEEN_SIDE: u8 = 0b1000;

// Named Squares
pub const A1: u8 = 0;
pub const E1: u8 = 4;
pub const H1: u8 = 7;
pub const A8: u8 = 56;
pub const E8: u8 = 60;
pub const H8: u8 = 63;

// Main struct board
#[derive(Debug, Clone)]
pub struct Board {
    pub pieces: [[Bitboard; 6]; 2],
    pub occupancy: [Bitboard; 3],
    pub side: Color,
    pub castling: u8,
    pub en_passant: Option<u8>,
    pub halfmove: u8,
    pub fullmove: u16,
    pub hash: u64,
}

impl Board {
    pub fn new() -> Self {
        Board {
            pieces: [[0; 6]; 2],
            occupancy: [0; 3],
            side: Color::White,
            castling: 0,
            en_passant: None,
            halfmove: 0,
            fullmove: 1,
            hash: 0,
        }
    }
    // Get bitboard for a specific piece
    #[inline]
    pub fn piece_bb(&self, color: Color, piece: PieceType) -> Bitboard {
        self.pieces[color as usize][piece as usize]
    }

    // Set/clear a piece on a square
    #[inline]
    pub fn set_piece(&mut self, color: Color, piece: PieceType, sq: u8) {
        self.pieces[color as usize][piece as usize] |= 1u64 << sq;
        self.occupancy[color as usize] |= 1u64 << sq;
        self.occupancy[BOTH_OCC] |= 1u64 << sq;
    }

    #[inline]
    pub fn clear_piece(&mut self, color: Color, piece: PieceType, sq: u8) {
        self.pieces[color as usize][piece as usize] &= !(1u64 << sq);
        self.occupancy[color as usize] &= !(1u64 << sq);
        // Rebuild BOTH from white|black to stay consistent
        self.occupancy[BOTH_OCC] = self.occupancy[WHITE_OCC] | self.occupancy[BLACK_OCC];
    }

    // Check if a square is occupied
    #[inline]
    pub fn is_occupied(&self, sq: u8) -> bool {
        (self.occupancy[BOTH_OCC] >> sq) & 1 == 1
    }

    // Check if a square is occupied by a specific color
    #[inline]
    pub fn is_occupied_by(&self, sq: u8, color: Color) -> bool {
        (self.occupancy[color as usize] >> sq) & 1 == 1
    }

    // Count pieces (uses hardware popcount — single CPU instruction)
    #[inline]
    pub fn count_pieces(&self, color: Color, piece: PieceType) -> u32 {
        self.piece_bb(color, piece).count_ones()
    }
}

// Ranking function
const fn square_indexing_positioning(rank: u8, file: u8) -> u8 {
    rank * 8 + file
}

// Extract rank/file back from a square index
#[inline]
pub const fn rank_of(sq: u8) -> u8 {
    sq >> 3
} // sq / 8
#[inline]
pub const fn file_of(sq: u8) -> u8 {
    sq & 0b111
} // sq % 8
