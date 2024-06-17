use crate::gamestate::board::*;
//https://www.chessprogramming.org/Square_Mapping_Considerations
// Movegen uses LERF mapping

// File bitboards
pub const A_FILE: Bitboard = 0x0101010101010101;
pub const B_FILE: Bitboard = 0x0202020202020202;
pub const C_FILE: Bitboard = 0x0404040404040404;
pub const D_FILE: Bitboard = 0x0808080808080808;
pub const E_FILE: Bitboard = 0x1010101010101010;
pub const F_FILE: Bitboard = 0x2020202020202020;
pub const G_FILE: Bitboard = 0x4040404040404040;
pub const H_FILE: Bitboard = 0x8080808080808080;

 
// Rank bitmasks
const FIRST_RANK: Bitboard = 0x0101010101010101;
const EIGHTH_RANK: Bitboard = 0x8080808080808080;
 
// Inverted file bitmasks (to prevent wrapping)
pub const NOT_A_FILE: Bitboard = !A_FILE;
pub const NOT_B_FILE: Bitboard = !B_FILE;
pub const NOT_C_FILE: Bitboard = !C_FILE;
pub const NOT_D_FILE: Bitboard = !D_FILE;
pub const NOT_E_FILE: Bitboard = !E_FILE;
pub const NOT_F_FILE: Bitboard = !F_FILE;
pub const NOT_G_FILE: Bitboard = !G_FILE;
pub const NOT_H_FILE: Bitboard = !H_FILE;

// Combined file bitmasks to prevent wrapping)
pub const NOT_AB_FILE: Bitboard = NOT_A_FILE & NOT_B_FILE;
pub const NOT_GH_FILE: Bitboard = NOT_G_FILE & NOT_H_FILE;

// Ckeckout https://www.chessprogramming.org/Knight_Pattern for idea what they are for
pub const NO_NO_EA: i32 = 17;  // North-North-East
pub const NO_EA_EA: i32 = 10;  // North-East-East
pub const SO_EA_EA: i32 = -6;  // South-East-East
pub const SO_SO_EA: i32 = -15; // South-South-East
pub const SO_SO_WE: i32 = -17; // South-South-West
pub const SO_WE_WE: i32 = -10; // South-West-West
pub const NO_WE_WE: i32 = 6;   // North-West-West
pub const NO_NO_WE: i32 = 15;  // North-North-West

// King and pawn offsets for attacks
pub const NORTH:     i32 = 8;         // North
pub const NORTHEAST: i32 = 9;     // Northeast
pub const EAST:      i32 = 1;          // East
pub const SOUTHEAST: i32 = -7;    // Southeast
pub const SOUTH:     i32 = -8;        // South
pub const SOUTHWEST: i32 = -9;    // Southwest
pub const WEST:      i32 = -1;         // West
pub const NORTHWEST: i32 = 7;     // Northwest

// Masks used for fast computation of all the attacks for a single piece at the time.
// Get a mask by using square index
pub const KING_ATTAKS_MASKS:[Bitboard; 64] = generate_king_attacks_masks();
pub const KNIGHT_ATTACKS_MASKS:[Bitboard; 64] = generate_knight_attacks_masks();
pub const WHITE_PAWN_ATTACKS_MASKS: [Bitboard; 64] = generate_pawn_attacks_masks().0;
pub const BLACK_PAWN_ATTACKS_MASKS: [Bitboard; 64] = generate_pawn_attacks_masks().1;

const fn generate_knight_attacks_masks() -> [Bitboard; 64] {
    let mut all_attacks: [Bitboard; 64] = [0; 64];

    let mut i: usize = 0;
    while i < 63 {
        let mut attacks_mask: Bitboard = 0;
        let position_mask = 1 << i;

        // Right side clockwise
        attacks_mask |= (position_mask << NO_NO_EA) & NOT_A_FILE;
        attacks_mask |= (position_mask << NO_EA_EA) & NOT_AB_FILE;
        attacks_mask |= (position_mask >> -SO_EA_EA) & NOT_AB_FILE;
        attacks_mask |= (position_mask >> -SO_SO_EA) & NOT_A_FILE;

        // Left side clockwise
        attacks_mask |= (position_mask >> -SO_SO_WE) & NOT_H_FILE;
        attacks_mask |= (position_mask >> -SO_WE_WE) & NOT_GH_FILE;
        attacks_mask |= (position_mask << NO_WE_WE) & NOT_GH_FILE;
        attacks_mask |= (position_mask << NO_NO_WE) & NOT_H_FILE;

        all_attacks[i] = attacks_mask;
        i += 1;
    }
    all_attacks
}
const fn generate_king_attacks_masks() -> [Bitboard; 64] {
    let mut all_attacks: [Bitboard; 64] = [0; 64];

    let mut i: usize = 0;
    while i < 64 {
        let mut attacks_mask: Bitboard = 0;

        let position_mask = ((1 as u64) << i) as Bitboard;

        attacks_mask |=  position_mask << NORTH;
        attacks_mask |= (position_mask << NORTHEAST) & NOT_A_FILE;
        attacks_mask |= (position_mask << EAST) & NOT_A_FILE;
        attacks_mask |= (position_mask >> -SOUTHEAST) & NOT_A_FILE;

        attacks_mask |=  position_mask >> -SOUTH;
        attacks_mask |= (position_mask >> -SOUTHWEST) & NOT_H_FILE;
        attacks_mask |= (position_mask >> -WEST) & NOT_H_FILE;
        attacks_mask |= (position_mask << NORTHWEST) & NOT_H_FILE;
        

        all_attacks[i] = attacks_mask;
        i = i + 1;
    }
    all_attacks
}
const fn generate_pawn_attacks_masks() -> ([Bitboard; 64], [Bitboard; 64]) {
    let mut all_white_attacks: [Bitboard; 64] = [0; 64];
    let mut all_black_attacks: [Bitboard; 64] = [0; 64];

    let mut i: usize = 0;
    while i < 64 {
        let mut white_attacks_mask: Bitboard = 0;
        let mut black_attacks_mask: Bitboard = 0;

        let position_mask = ((1 as u64) << i) as Bitboard;

        white_attacks_mask |= (position_mask << NORTHEAST) & NOT_A_FILE;
        white_attacks_mask |= (position_mask << NORTHWEST) & NOT_H_FILE;

        black_attacks_mask |= (position_mask >> -SOUTHEAST) & NOT_A_FILE;
        black_attacks_mask |= (position_mask >> -SOUTHWEST) & NOT_H_FILE;

        


        
        all_white_attacks[i] = white_attacks_mask;
        all_black_attacks[i] = black_attacks_mask;
        i = i + 1;
    }
    (all_white_attacks, all_black_attacks)
}
