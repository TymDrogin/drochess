pub mod board;
pub mod defs;

use self::{
    board::{Board, Side},
    defs::CastlingRights,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Gamestate {
    pub board: Board,
    pub side_to_move: Side,
    pub castling_rights: CastlingRights,
    pub en_passant: u8,
    pub half_move_clock: u8,
    pub full_move_count: u8,
}
