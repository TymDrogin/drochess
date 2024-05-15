mod defs;
mod board;
use defs::*;
use board::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Gamestate {
    board: Board,
    side_to_move: Side,    
    en_passant: Option<u8>,
    half_move_clock: u32,
    full_move_count: u32,
}

impl Gamestate {
    pub fn new() -> Self {
        todo!()
}
}
