pub mod board;
pub mod castling_rights;
pub mod zobrist;
pub mod defs;
pub mod chess_move;

use rayon::iter::FlatMap;

use crate::gamestate::defs::PIECE_TYPES_NUM;

use self::{
    board::{Board, Side, Square, PieceType, Bitboard},
    castling_rights::CastlingRights,
    zobrist::*,
};


#[derive(Debug, Clone, PartialEq)]
pub struct Gamestate {
    pub board: Board,
    pub zobrist_key: u64,

    pub side_to_move: Side,

    pub castling_rights: CastlingRights,

    pub en_passant: u8,

    pub half_move_clock: u8,
    pub full_move_count: u8,

}
impl Gamestate {
}

