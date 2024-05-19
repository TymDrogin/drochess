pub mod defs;
pub mod board;

use self::{
    defs::{
        Side,
        CastlingRights,
    },
    board::Board,
};



#[derive(Debug, Clone, PartialEq)]
pub struct Gamestate {
    pub board: Board,
    pub side_to_move: Side, 
    pub castling_rights: CastlingRights,   
    pub en_passant: Option<u8>,
    pub half_move_clock: usize,
    pub full_move_count: usize,
}

