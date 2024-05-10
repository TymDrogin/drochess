/*
This files defines common structures and functions
that may be used in other modules.
*/

pub enum Piece {
    King(Bitboard), 
    Queen(Bitboard), 
    Rook(Bitboard), 
    Knight(Bitboard),
    Pawn(Bitboard), 
    Bishop(Bitboard),
}

pub const PIECE_TYPES_NUM: usize = 6;

pub const DEFAULT_PAWN_POSITON:u64 = 0x000000000000ff00;

pub enum Side {
    White,
    Black,
    None
}

pub type Bitboard = u64;

pub fn print_bitboard(bitboard: Bitboard) {
    let bytes = bitboard.to_ne_bytes();
    
    for byte in bytes {
        for i in 0..8 {
            let bit = (byte >> i) & 1;
            print!("{} ", bit);
        }
        println!();
    }
}