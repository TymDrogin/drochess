pub mod board;
pub mod castling_rights;

use self::{
    board::{Board, Side, Square, PieceType, Bitboard},
    castling_rights::CastlingRights,
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
impl Gamestate {
    pub fn apply_move(&mut self, mov: Move) {
        todo!()
    }
    pub fn undo_move(&mut self, mov: Move) {
        todo!()
    }
    
}

pub enum MoveType {
    Quiet,
    Capture,
    EnPassant,
    Castling,
    Promotion(PieceType),
}

// REFACTOR: Make a use on move encoding
pub struct Move {
    pub from: Square,
    pub to:Square,
    pub type_of: MoveType,
}


pub fn print_bitboard(bitboard: Bitboard) {
    for rank in 0..8 {
        for file in 0..8 {
            let square_index = rank * 8 + file;
            if (bitboard & (1 << square_index)) != 0 {
                print!("1 ");
            } else {
                print!("0 ");
            }
        }
        println!();
    }
    println!();
}