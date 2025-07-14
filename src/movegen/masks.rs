use crate::{
    gamestate::{
        board::*,
        constants::{BOARD_NUM_OF_SQUARES, BOARD_SIDE_LENGTH, SIDE_NUM},
    },
    movegen::constants::*,
};

pub const KING_ATTACKS: [Bitboard; BOARD_NUM_OF_SQUARES] = generate_king_attacks();
pub const KNIGHT_ATTACKS: [Bitboard; BOARD_NUM_OF_SQUARES] = generate_knight_attacks();

pub const PAWN_SINGLE_PUSHES: [[Bitboard; BOARD_NUM_OF_SQUARES]; SIDE_NUM] =
    [generate_pawn_single_pushes().0, generate_pawn_single_pushes().1];
pub const PAWN_DOUBLE_PUSHES: [[Bitboard; BOARD_SIDE_LENGTH]; SIDE_NUM] =
    [generate_pawn_double_pushes().0, generate_pawn_double_pushes().1];
pub const PAWN_ATTACKS: [[Bitboard; BOARD_NUM_OF_SQUARES]; SIDE_NUM] =
    [generate_pawn_attacks().0, generate_pawn_attacks().1];

pub const ROOK_MASKS: [Bitboard; BOARD_NUM_OF_SQUARES] = generate_rook_masks();
pub const BISHOP_MASKS: [Bitboard; BOARD_NUM_OF_SQUARES] = generate_bishop_masks();


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

const fn generate_knight_attacks() -> [Bitboard; 64] {
    let mut all_attacks: [Bitboard; 64] = [0; 64];
    let mut i: usize = 0;

    while i < 64 {
        let mut attacks_mask: Bitboard = 0;
        let position_mask = Square::new(i as u8).get_mask();

        // Right side clockwise
        attacks_mask |= shift(position_mask, NO_NO_EA) & NOT_A_FILE;
        attacks_mask |= shift(position_mask, NO_EA_EA) & NOT_AB_FILE;
        attacks_mask |= shift(position_mask, SO_EA_EA) & NOT_AB_FILE;
        attacks_mask |= shift(position_mask, SO_SO_EA) & NOT_A_FILE;

        // Left side clockwise
        attacks_mask |= shift(position_mask, SO_SO_WE) & NOT_H_FILE;
        attacks_mask |= shift(position_mask, SO_WE_WE) & NOT_GH_FILE;
        attacks_mask |= shift(position_mask, NO_WE_WE) & NOT_GH_FILE;
        attacks_mask |= shift(position_mask, NO_NO_WE) & NOT_H_FILE;

        all_attacks[i] = attacks_mask;
        i += 1;
    }

    all_attacks
}
const fn generate_pawn_attacks() -> ([Bitboard; 64], [Bitboard; 64]) {
    let mut all_white_attacks: [Bitboard; 64] = [0; 64];
    let mut all_black_attacks: [Bitboard; 64] = [0; 64];
    let mut i: usize = 0;

    while i < 64 {
        let position_mask = Square::new(i as u8).get_mask();

        let white_attacks_mask =
            (shift(position_mask, NORTHEAST) & NOT_A_FILE) |
            (shift(position_mask, NORTHWEST) & NOT_H_FILE);

        let black_attacks_mask =
            (shift(position_mask, SOUTHEAST) & NOT_A_FILE) |
            (shift(position_mask, SOUTHWEST) & NOT_H_FILE);

        all_white_attacks[i] = white_attacks_mask;
        all_black_attacks[i] = black_attacks_mask;
        i += 1;
    }

    (all_white_attacks, all_black_attacks)
}

const fn generate_pawn_single_pushes() -> ([Bitboard; 64], [Bitboard; 64]) {
    let mut all_white_pushes: [Bitboard; 64] = [0; 64];
    let mut all_black_pushes: [Bitboard; 64] = [0; 64];
    let mut i: usize = 0;

    while i < 64 {
        let position_mask = Square::new(i as u8).get_mask();
        let rank = Square::new(i as u8).get_file_rank().1;

        let white_pushes_mask = shift(position_mask, NORTH);
        let black_pushes_mask = shift(position_mask, SOUTH);

        all_white_pushes[i] = white_pushes_mask;
        all_black_pushes[i] = black_pushes_mask;
        i += 1;
    }

    (all_white_pushes, all_black_pushes)
}
const fn generate_pawn_double_pushes() -> ([Bitboard; BOARD_SIDE_LENGTH], [Bitboard; BOARD_SIDE_LENGTH]) {
    let mut all_white_pushes: [Bitboard; BOARD_SIDE_LENGTH] = [0; BOARD_SIDE_LENGTH];
    let mut all_black_pushes: [Bitboard; BOARD_SIDE_LENGTH] = [0; BOARD_SIDE_LENGTH];
    let mut i: usize = 0;
    while i < 8 {
        let white_position_mask = Square::new_from_file_rank(i as u8, 1).get_mask();
        let black_position_mask = Square::new_from_file_rank(i as u8, 6).get_mask();

        all_white_pushes[i] = shift(white_position_mask, NORTH * 2);
        all_black_pushes[i] = shift(black_position_mask, SOUTH * 2);
        
        i += 1;
    }
    (all_white_pushes, all_black_pushes)

}
const fn generate_king_attacks() -> [Bitboard; 64] {
    let mut all_attacks: [Bitboard; 64] = [0; 64];
    let mut i: usize = 0;

    while i < 64 {
        let mut attacks_mask: Bitboard = 0;
        let position_mask = Square::new(i as u8).get_mask();

        attacks_mask |= shift(position_mask, NORTH);
        attacks_mask |= shift(position_mask, NORTHEAST) & NOT_A_FILE;
        attacks_mask |= shift(position_mask, EAST) & NOT_A_FILE;
        attacks_mask |= shift(position_mask, SOUTHEAST) & NOT_A_FILE;

        attacks_mask |= shift(position_mask, SOUTH);
        attacks_mask |= shift(position_mask, SOUTHWEST) & NOT_H_FILE;
        attacks_mask |= shift(position_mask, WEST) & NOT_H_FILE;
        attacks_mask |= shift(position_mask, NORTHWEST) & NOT_H_FILE;

        all_attacks[i] = attacks_mask;
        i += 1;
    }

    all_attacks
}


const fn generate_rook_masks() -> [Bitboard; 64] {
    let mut masks = [0u64; 64];
    let mut i = 0;

    while i < 64 {
        let mut mask = 0u64;
        let (file, rank) = Square::new(i as u8).get_file_rank();

        // ─── NORTH (↑) ───
        let mut r = rank + 1;
        while r < 7 {
            mask |= Square::new((file + r * 8) as u8).get_mask();
            r += 1;
        }

        // ─── SOUTH (↓) ───
        let mut r = rank;
        while r > 1 {
            r -= 1;
            mask |= Square::new((file + r * 8) as u8).get_mask();
        }

        // ─── EAST (→) ───
        let mut f = file + 1;
        while f < 7 {
            mask |= Square::new((f + rank * 8) as u8).get_mask();
            f += 1;
        }

        // ─── WEST (←) ───
        let mut f = file;
        while f > 1 {
            f -= 1;
            mask |= Square::new((f + rank * 8) as u8).get_mask();
        }

        masks[i] = mask;
        i += 1;
    }

    masks
}

const fn generate_bishop_masks() -> [Bitboard; 64] {
    let mut masks = [0u64; 64];
    let mut i = 0;

    while i < 64 {
        let mut mask = 0u64;
        let (file, rank) = Square::new(i as u8).get_file_rank();

        // ─── NORTH‑EAST (↗) ───
        let mut f = file + 1;
        let mut r = rank + 1;
        while f < 7 && r < 7 {
            mask |= Square::new((f + r * 8) as u8).get_mask();
            f += 1;
            r += 1;
        }

        // ─── NORTH‑WEST (↖) ───
        let mut f = file;
        let mut r = rank;
        // stop one before file=0 and one before rank=7
        while f > 1 && r < 6 {
            f -= 1;
            r += 1;
            mask |= Square::new((f + r * 8) as u8).get_mask();
        }

        // ─── SOUTH‑WEST (↙) ───
        let mut f = file;
        let mut r = rank;
        // stop one before file=0 and one before rank=0
        while f > 1 && r > 1 {
            f -= 1;
            r -= 1;
            mask |= Square::new((f + r * 8) as u8).get_mask();
        }

        // ─── SOUTH‑EAST (↘) ───
        let mut f = file + 1;
        let mut r = rank;
        // stop one before file=7 and one before rank=0
        while f < 7 && r > 1 {
            f += 1;
            r -= 1;
            mask |= Square::new((f + r * 8) as u8).get_mask();
        }

        masks[i] = mask;
        i += 1;
    }

    masks
}

pub const fn shift(bitboard: Bitboard, dir: i32) -> Bitboard {
    if dir >= 0 {
        bitboard << dir
    } else {
        bitboard >> -dir
    }
}
