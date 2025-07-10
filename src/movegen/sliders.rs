use crate::gamestate::{
    board::*,
    constants::*,
};
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
    ROOK_ATTACKS[square][index]
}
pub fn get_bishop_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    let mask = BISHOP_MASKS[square.get_index()];
    let index = unsafe { pext(occupancy, mask) } as usize;
    BISHOP_ATTACKS[square][index]
}