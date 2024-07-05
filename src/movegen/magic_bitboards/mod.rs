use crate::gamestate::{
    board::*,
    defs::*,
};
use super::{
    defs::*,
    masks::*,
    
};

#[derive(Default, Copy, Clone)]
pub struct Magic {
    pub mask: Bitboard,
    pub shift: u8,
    pub offset: u64,
    pub nr: u64,
}
impl Magic {
    pub fn get_index(&self, occupancy: Bitboard) -> usize {
        let blocker_mask = occupancy & self.mask;

        ((blocker_mask.wrapping_mul(self.nr) >> self.shift) + self.offset) as usize
    }
}