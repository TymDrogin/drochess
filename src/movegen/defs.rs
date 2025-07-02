use crate::gamestate::board::*;
//https://www.chessprogramming.org/Square_Mapping_Considerations
// Movegen uses LERF mapping

// File bitboards
pub const A_FILE: Bitboard = 0x0101010101010101;
pub const B_FILE: Bitboard = 0x0202020202020202;
#[allow(unused)]
pub const C_FILE: Bitboard = 0x0404040404040404;
pub const D_FILE: Bitboard = 0x0808080808080808;
#[allow(unused)]
pub const E_FILE: Bitboard = 0x1010101010101010;
pub const F_FILE: Bitboard = 0x2020202020202020;
pub const G_FILE: Bitboard = 0x4040404040404040;
pub const H_FILE: Bitboard = 0x8080808080808080;

// Rank bitmasks
#[allow(unused)]
const FIRST_RANK: Bitboard = 0x0101010101010101;
#[allow(unused)]
const EIGHTH_RANK: Bitboard = 0x8080808080808080;

// Inverted file bitmasks (to prevent wrapping)
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
