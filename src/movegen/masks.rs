

use crate::{
    gamestate::board::*,
    movegen::defs::*,
};


// Masks used for fast computation of all the attacks for a single piece at the time.
// Get a mask by using square index
pub const KING_ATTAKS_MASKS:[Bitboard; 64] = generate_king_attacks_masks();
pub const KNIGHT_ATTACKS_MASKS:[Bitboard; 64] = generate_knight_attacks_masks();

pub const WHITE_PAWN_ATTACKS_MASKS: [Bitboard; 64] = generate_pawn_attacks_masks().0;
pub const WHITE_PAWN_PUSHES_MASKS: [Bitboard; 64] = generate_pawn_pushes_masks().0;

pub const BLACK_PAWN_ATTACKS_MASKS: [Bitboard; 64] = generate_pawn_attacks_masks().1;
pub const BLACK_PAWN_PUSHES_MASKS: [Bitboard; 64] = generate_pawn_pushes_masks().1;
pub const ROOK_RAYS: [Bitboard; 64] = generate_rook_rays();
pub const BISHOP_RAYS: [Bitboard; 64] = generate_bishop_rays();


//pub const ROOK_RAYS: [Bitboard; 64] = generate_rook_rays();
//pub const BISHOP_RAYS: [Bitboard; 64] = generate_bishop_rays();

const fn generate_knight_attacks_masks() -> [Bitboard; 64] {
    let mut all_attacks: [Bitboard; 64] = [0; 64];

    let mut i: usize = 0;
    while i < 63 {
        let mut attacks_mask: Bitboard = 0;
        let position_mask = Square::new(i as u8).get_mask();

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

        let position_mask = Square::new(i as u8).get_mask();

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

        let position_mask = Square::new(i as u8).get_mask();

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
const fn generate_pawn_pushes_masks()  -> ([Bitboard; 64], [Bitboard; 64]) {
    let mut all_white_pushes: [Bitboard; 64] = [0; 64];
    let mut all_black_pushes: [Bitboard; 64] = [0; 64];

    let mut i: usize = 0;
    while i < 64 {
        let mut white_pushes_mask: Bitboard = 0;
        let mut black_pushes_mask: Bitboard = 0;

        let position_mask = Square::new(i as u8).get_mask();
        let rank = Square::new(i as u8).get_file_rank().1;

        white_pushes_mask |= position_mask << NORTH;
        if rank == 1 { // Second file applies 2 square move
            white_pushes_mask |= position_mask << (NORTH * 2);
        }

        black_pushes_mask |= position_mask >> -SOUTH;
        if rank == 6 { // Fifth file applies 2 square move
            white_pushes_mask |= position_mask >> (-SOUTH * 2);
        }
        
        all_white_pushes[i] = white_pushes_mask;
        all_black_pushes[i] = black_pushes_mask;
        i = i + 1;
    }
    (all_white_pushes, all_black_pushes)
}

const fn generate_rook_rays() -> [Bitboard; 64] {
    let mut all_rays: [Bitboard; 64] = [0; 64];

    let mut i: usize = 0;
    while i < 64 {
        let mut rays_mask: Bitboard = 0;
        let position_mask = Square::new(i as u8).get_mask();

        // North
        let mut pos = i as i32;
        while pos + NORTH < 64 {
            pos += NORTH;
            rays_mask |= 1 << pos;
        }

        // South
        pos = i as i32;
        while pos + SOUTH >= 0 {
            pos += SOUTH;
            rays_mask |= 1 << pos;
        }

        // East
        pos = i as i32;
        while pos % 8 != 7 {
            pos += EAST;
            rays_mask |= 1 << pos;
        }

        // West
        pos = i as i32;
        while pos % 8 != 0 {
            pos += WEST;
            rays_mask |= 1 << pos;
        }

        all_rays[i] = rays_mask;
        i += 1;
    }

    all_rays
}
const fn generate_bishop_rays() -> [Bitboard; 64] {
    let mut all_rays: [Bitboard; 64] = [0; 64];

    let mut i: usize = 0;
    while i < 64 {
        let mut rays_mask: Bitboard = 0;

        // North-East
        let mut pos = i as i32;
        while pos + NORTHEAST < 64 && pos % 8 != 7 {
            pos += NORTHEAST;
            rays_mask |= 1 << pos;
        }

        // North-West
        pos = i as i32;
        while pos + NORTHWEST < 64 && pos % 8 != 0 {
            pos += NORTHWEST;
            rays_mask |= 1 << pos;
        }

        // South-East
        pos = i as i32;
        while pos + SOUTHEAST >= 0 && pos % 8 != 7 {
            pos += SOUTHEAST;
            rays_mask |= 1 << pos;
        }

        // South-West
        pos = i as i32;
        while pos + SOUTHWEST >= 0 && pos % 8 != 0 {
            pos += SOUTHWEST;
            rays_mask |= 1 << pos;
        }

        all_rays[i] = rays_mask;
        i += 1;
    }

    all_rays
}

