mod defs;
mod fen;
use defs::*;
use crate::board;



pub struct Gamestate {
    board: Board,
    side_to_move: Side,

}

impl Gamestate {
    pub fn new(&str: fen) -> Self {
        parse_fen_to_gamestate(fen)
    }
}


pub fn get_piece_squares(&self, piece_type: PieceType, side: Side) -> Bitboard {
    match side {
        Side::White => self.white_pieces[piece_type as usize],
        Side::Black => self.black_pieces[piece_type as usize],
        _ => panic!("Can't get piece bitmap -- no side selected")
    }
}