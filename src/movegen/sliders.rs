use crate::gamestate::{
    board::*,
    constants::*,
};
use crate::movegen::constants::NORTH;
use crate::movegen::masks::*;


use core::arch::x86_64::_pext_u64;


// Pext is used instead of magic bitboards for easier and faster development.
// Also there where some instances it proved to be faster
#[inline(always)]
unsafe fn pext(x: u64, mask: u64) -> u64 {
    _pext_u64(x, mask)
}

pub fn get_rook_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    let mask = ROOK_MASKS[square.get_index()];
    let index = unsafe { pext(occupancy, mask) } as usize;
    //ROOK_ATTACKS[square][index]
    todo!()
}
pub fn get_bishop_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    let mask = BISHOP_MASKS[square.get_index()];
    let index = unsafe { pext(occupancy, mask) } as usize;
    //BISHOP_ATTACKS[square][index]
    todo!()
}

pub fn calculate_rook_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    let (file, rank) = square.get_file_rank();
    // only keep bits on this rook’s rays
    let blocker_mask = ROOK_MASKS[square.get_index()];
    let occ = occupancy & blocker_mask;

    let mut attack: Bitboard = 0;

    // ─── NORTH (↑) ───
    for r in (rank + 1)..8 {
        let bb = Square::new_from_file_rank(file, r).get_mask();
        attack |= bb;
        if bb & occ != 0 {
            break;
        }
    }

    // ─── SOUTH (↓) ───
    for r in (0..rank).rev() {
        let bb = Square::new_from_file_rank(file, r).get_mask();
        attack |= bb;
        if bb & occ != 0 {
            break;
        }
    }

    // ─── EAST (→) ───
    for f in (file + 1)..8 {
        let bb = Square::new_from_file_rank(f, rank).get_mask();
        attack |= bb;
        if bb & occ != 0 {
            break;
        }
    }

    // ─── WEST (←) ───
    for f in (0..file).rev() {
        let bb = Square::new_from_file_rank(f, rank).get_mask();
        attack |= bb;
        if bb & occ != 0 {
            break;
        }
    }

    attack
}
pub fn calculate_bishop_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    let idx = square.get_index();
    let (file, rank) = square.get_file_rank();

    // Only keep occupancy bits on the bishop’s diagonals
    let blocker_mask = BISHOP_MASKS[idx];
    let occ = occupancy & blocker_mask;

    let mut attack: Bitboard = 0;

    // ─── NORTH‑EAST (↗) ───
    {
        let mut f = file + 1;
        let mut r = rank + 1;
        while f < 8 && r < 8 {
            let bb = Square::new_from_file_rank(f, r).get_mask();
            attack |= bb;
            if bb & occ != 0 { break; }
            f += 1; r += 1;
        }
    }

    // ─── NORTH‑WEST (↖) ───
    {
        let mut f = file;
        let mut r = rank;
        while f > 0 && r < 7 {
            f -= 1; r += 1;
            let bb = Square::new_from_file_rank(f, r).get_mask();
            attack |= bb;
            if bb & occ != 0 { break; }
        }
    }

    // ─── SOUTH‑EAST (↘) ───
    {
        let mut f = file;
        let mut r = rank;
        while f < 7 && r > 0 {
            f += 1; r -= 1;
            let bb = Square::new_from_file_rank(f, r).get_mask();
            attack |= bb;
            if bb & occ != 0 { break; }
        }
    }

    // ─── SOUTH‑WEST (↙) ───
    {
        let mut f = file;
        let mut r = rank;
        while f > 0 && r > 0 {
            f -= 1; r -= 1;
            let bb = Square::new_from_file_rank(f, r).get_mask();
            attack |= bb;
            if bb & occ != 0 { break; }
        }
    }

    attack
}

