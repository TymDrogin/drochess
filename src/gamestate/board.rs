pub const PIECE_TYPES_NUM: usize = 6;
pub type Bitboard = u64;
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub white_pieces: [Bitboard; PIECE_TYPES_NUM],
    pub black_pieces: [Bitboard; PIECE_TYPES_NUM],
}

impl Board {
    pub fn set_square(&mut self, ranks: usize, files: usize, pt: PieceType, side: Side) {
        let mask: Bitboard = 1 << (files * 8 + rank);
        match side {
            Side::White => self.white_pieces[PieceType] |= mask,
            Side::Black => self.black_pieces[PieceType] |= mask,
        }
    }
}

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}
