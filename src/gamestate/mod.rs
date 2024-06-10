pub mod board;
pub mod castling_rights;

use self::{
    board::{Board, Side, Square, PieceType},
    castling_rights::CastlingRights,
};
pub enum MoveType {
    Quiet,
    Capture,
    EnPassant,
    Castling,
    Promotion(PieceType),
}
pub struct Move {
    pub from: Square,
    pub to:Square,
    pub type_of: MoveType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Gamestate {
    pub board: Board,
    pub side_to_move: Side,
    pub castling_rights: CastlingRights,
    pub en_passant: u8,
    pub half_move_clock: u8,
    pub full_move_count: u8,
}
impl Gamestate {
    pub fn apply_move(&mut self, mov: Move) {
        
    }
    pub fn undo_move(&mut self, mov: Move) {

    }
    
}