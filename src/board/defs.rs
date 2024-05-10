/*
This files defines common structures and functions
that may be used in other modules.
*/

pub enum PieceType {
    King, 
    Queen, 
    Rook, 
    Knight,
    Pawn, 
    Bishop,
    Empty,
}

// Used mainly in loops 
pub const PIECE_TYPES_NUM: usize = 6; 

pub const DEFAULT_PAWN_POSITON:u64 = 0x000000000000ff00;

pub enum Side {
    White,
    Black,
    None
}

pub type Bitboard = u64;

