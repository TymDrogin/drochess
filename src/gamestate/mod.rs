pub mod defs;
pub mod board;

use self::{
    defs::CastlingRights,
    board:: {
        Board,
        Side,
    }
};

#[derive(Debug, Clone, PartialEq)]
pub struct Gamestate {
    pub board: Board,
    pub side_to_move: Side, 
    pub castling_rights: CastlingRights,   
    pub en_passant: Option<u8>,
    pub half_move_clock: u8,
    pub full_move_count: u8,
}