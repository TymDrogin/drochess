pub mod defs;
use defs::*;

pub struct Board {
    white_pieces: [Bitboard; PIECE_TYPES_NUM],
    black_pieces: [Bitboard; PIECE_TYPES_NUM],
}