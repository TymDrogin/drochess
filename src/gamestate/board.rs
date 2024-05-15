#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    white_pieces: [Bitboard; PIECE_TYPES_NUM],
    black_pieces: [Bitboard; PIECE_TYPES_NUM],
}

pub enum PieceType {
    Pawn,
    Knight, 
    Bishop,
    Rook,
    Queen, 
    King,
}

// Used mainly in loops 
pub const PIECE_TYPES_NUM: usize = 6; 

pub type Bitboard = u64;
