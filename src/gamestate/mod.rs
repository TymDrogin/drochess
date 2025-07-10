pub mod board;
pub mod castling_rights;
pub mod chess_move;
pub mod constants;
pub mod zobrist;


use crate::gamestate::constants::PIECE_TYPES_NUM;

use self::{
    board::{Bitboard, Board, PieceType, Side, Square},
    castling_rights::CastlingRights,
    zobrist::*,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Gamestate {
    pub board: Board,
    pub side_to_move: Side,
    pub castling_rights: CastlingRights,
    pub en_passant: u8,
    pub half_move_clock: u8,
    pub full_move_count: u8,
    pub zobrist_key: u64,
}
impl Gamestate {
    pub fn new(
        board: Board,
        side_to_move: Side,
        castling_rights: CastlingRights,
        en_passant: u8,
        half_move_clock: u8,
        full_move_count: u8,
        zobrist_key: u64,
    ) -> Self {
        Self {
            board,
            side_to_move,
            castling_rights,
            en_passant,
            half_move_clock,
            full_move_count,
            zobrist_key,
        }
    }
}
