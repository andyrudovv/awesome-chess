use crate::bitboard::Bitboard;

#[derive(Debug, Clone, Copy)]
pub struct ChessBoard {
    pub white_pieces: Bitboard,
    pub black_pieces: Bitboard,
    pub white_king: Bitboard,
    pub black_king: Bitboard,
    pub white_pawns: Bitboard,
    pub black_pawns: Bitboard,
    pub white_knights: Bitboard,
    pub black_knights: Bitboard,
    pub white_rooks: Bitboard,
    pub black_rooks: Bitboard,
    pub occupied: Bitboard,
    pub empty: Bitboard,
}

impl ChessBoard {
    pub fn new() -> Self {
        Self {
            white_pieces: Bitboard::new(),
            black_pieces: Bitboard::new(),
            white_king: Bitboard::new(),
            black_king: Bitboard::new(),
            white_pawns: Bitboard::new(),
            black_pawns: Bitboard::new(),
            white_knights: Bitboard::new(),
            black_knights: Bitboard::new(),
            white_rooks: Bitboard::new(),
            black_rooks: Bitboard::new(),
            occupied: Bitboard::new(),
            empty: Bitboard::new(),
        }
    }
}
