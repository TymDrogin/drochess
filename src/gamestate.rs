pub mod defs;
mod board;

use self::{
    defs::{
        Side,
        CastlingRights,
    },
    board::Board,
};



#[derive(Debug, Clone, PartialEq)]
pub struct Gamestate {
    board: Board,
    side_to_move: Side, 
    castling_rights: CastlingRights,   
    en_passant: Option<u8>,
    half_move_clock: u32,
    full_move_count: u32,
}

impl Gamestate {
    pub fn new() -> Self {
        todo!()
    }
}
