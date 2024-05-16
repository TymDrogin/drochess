pub const PIECE_TYPES_NUM: usize = 6; 
pub type Bitboard = u64;
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

// Constants for piece symbols in FEN notation
pub const WHITE_KING: char = 'K';
pub const WHITE_QUEEN: char = 'Q';
pub const WHITE_ROOK: char = 'R';
pub const WHITE_BISHOP: char = 'B';
pub const WHITE_KNIGHT: char = 'N';
pub const WHITE_PAWN: char = 'P';

pub const BLACK_KING: char = 'k';
pub const BLACK_QUEEN: char = 'q';
pub const BLACK_ROOK: char = 'r';
pub const BLACK_BISHOP: char = 'b';
pub const BLACK_KNIGHT: char = 'n';
pub const BLACK_PAWN: char = 'p';