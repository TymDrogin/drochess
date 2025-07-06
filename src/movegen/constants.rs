use crate::gamestate::board::*;
//https://www.chessprogramming.org/Square_Mapping_Considerations
// Movegen uses LERF mapping

// File bitboards
#[allow(unused)]
pub const A_FILE: Bitboard = 0x0101010101010101;
#[allow(unused)]
pub const B_FILE: Bitboard = 0x0202020202020202;
#[allow(unused)]
pub const C_FILE: Bitboard = 0x0404040404040404;
#[allow(unused)]
pub const D_FILE: Bitboard = 0x0808080808080808;
#[allow(unused)]
pub const E_FILE: Bitboard = 0x1010101010101010;
#[allow(unused)]
pub const F_FILE: Bitboard = 0x2020202020202020;
#[allow(unused)]
pub const G_FILE: Bitboard = 0x4040404040404040;
#[allow(unused)]
pub const H_FILE: Bitboard = 0x8080808080808080;

#[allow(unused)]
pub const RANK_1: u64 = 0x00000000000000FF;
#[allow(unused)]
pub const RANK_2: u64 = 0x000000000000FF00;
#[allow(unused)]
pub const RANK_3: u64 = 0x0000000000FF0000;
#[allow(unused)]
pub const RANK_4: u64 = 0x00000000FF000000;
#[allow(unused)]
pub const RANK_5: u64 = 0x000000FF00000000;
#[allow(unused)]
pub const RANK_6: u64 = 0x0000FF0000000000;
#[allow(unused)]
pub const RANK_7: u64 = 0x00FF000000000000;
#[allow(unused)]
pub const RANK_8: u64 = 0xFF00000000000000;


pub const CLEAR_7TH_OR_2ND_RANK: [u64; 2] = [RANK_7, RANK_2];

// Inverted file bitmasks (to prevent wrapping)
#[allow(unused)]
pub const NOT_A_FILE: Bitboard = !A_FILE;
pub const NOT_B_FILE: Bitboard = !B_FILE;
#[allow(unused)]
pub const NOT_C_FILE: Bitboard = !C_FILE;
#[allow(unused)]
pub const NOT_D_FILE: Bitboard = !D_FILE;
#[allow(unused)]
pub const NOT_E_FILE: Bitboard = !E_FILE;
#[allow(unused)]
pub const NOT_F_FILE: Bitboard = !F_FILE;
pub const NOT_G_FILE: Bitboard = !G_FILE;
pub const NOT_H_FILE: Bitboard = !H_FILE;

// Combined file bitmasks to prevent wrapping)
pub const NOT_AB_FILE: Bitboard = NOT_A_FILE & NOT_B_FILE;
pub const NOT_GH_FILE: Bitboard = NOT_G_FILE & NOT_H_FILE;

// Check out https://www.chessprogramming.org/Knight_Pattern for idea what they are for
pub const NO_NO_EA: i32 = 17; // North-North-East
pub const NO_EA_EA: i32 = 10; // North-East-East
pub const SO_EA_EA: i32 = -6; // South-East-East
pub const SO_SO_EA: i32 = -15; // South-South-East
pub const SO_SO_WE: i32 = -17; // South-South-West
pub const SO_WE_WE: i32 = -10; // South-West-West
pub const NO_WE_WE: i32 = 6; // North-West-West
pub const NO_NO_WE: i32 = 15; // North-North-West

// King and pawn offsets for attacks
#[allow(unused)]
pub const NORTH: i32 = 8; // North
#[allow(unused)]
pub const NORTHEAST: i32 = 9; // Northeast
#[allow(unused)]
pub const EAST: i32 = 1; // East
#[allow(unused)]
pub const SOUTHEAST: i32 = -7; // Southeast
#[allow(unused)]
pub const SOUTH: i32 = -8; // South
#[allow(unused)]
pub const SOUTHWEST: i32 = -9; // Southwest
#[allow(unused)]
pub const WEST: i32 = -1; // West
#[allow(unused)]
pub const NORTHWEST: i32 = 7; // Northwest





// Castling start and end indecies for kings and rooks
// These are used to generate castling moves
pub const CASTLING_KING_START_INDEX: [u8; 2] = [4, 60]; // White and Black kings
pub const CASTLING_ROOK_KINGSIDE_START_INDEX: [u8; 2] = [7, 63]; // White and Black kingside rooks
pub const CASTLING_ROOK_QUEENSIDE_START_INDEX: [u8; 2] = [0, 56]; // White and Black queenside rooks

pub const CASTLING_KING_KINGSIDE_END_INDEX: [u8; 2] = [6, 62]; // White and Black kingside end squares
pub const CASTLING_ROOK_KINGSIDE_END_INDEX: [u8; 2] = [5, 61]; // White and Black kingside end squares

pub const CASTLING_KING_QUEENSIDE_END_INDEX: [u8; 2] = [2, 58]; // White and Black queenside end squares
pub const CASTLING_ROOK_QUEENSIDE_END_INDEX: [u8; 2] = [3, 59]; // White and Black queenside end squares

// Castling occupancy masks
// These masks are used to check if the squares between the king and rook are occupied
pub const CASTLING_KINGSIDE_OCCUPANCY_MASK: [Bitboard; 2] = [
  (1 << 5 | 1 << 6),
  (1 << 61 | 1 << 62)
];
pub const CASTLING_QUEENSIDE_OCCUPANCY_MASK: [Bitboard; 2] = [
  (1 << 1 | 1 << 2 | 1 << 3),
  (1 << 57 | 1 << 58 | 1 << 59)
];
