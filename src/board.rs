pub mod defs;
use defs::*;
use std::unimplemented;


pub struct Board {
    white_pieces: [Bitboard; PIECE_TYPES_NUM],
    black_pieces: [Bitboard; PIECE_TYPES_NUM],
    side_to_move: Side,
}

impl Board {
    pub fn new() {
        unimplemented!()
    }
    pub fn get_free_squares(&self) -> Bitboard {
        let mut free_squares: Bitboard = 0;
        for i in 0..PIECE_TYPES_NUM {
            free_squares |= !(self.white_pieces[i] | self.black_pieces[i]);
        }
        free_squares
    }

    pub fn get_piece_squares(&self, piece_type: PieceType, side: Side) -> Bitboard {
        match side {
            Side::White => self.white_pieces[piece_type as usize],
            Side::Black => self.black_pieces[piece_type as usize],
            _ => panic!("Can't get piece bitmap -- no side selected")
        }
    }
}
