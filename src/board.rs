pub mod defs;
use defs::*;
use std::unimplemented;


// Each array of pieces should consist of element type of piece, which are UNIQUE
// If you were to add new piece it whould have to implement ChesPiece trait
pub struct Board {
    white_pieces: [Piece; PIECE_TYPES_NUM],
    black_pieces: [Piece; PIECE_TYPES_NUM],
    side_to_move: Side,
}

pub struct Square {

}

impl Board {
    pub fn new() {
        unimplemented!()
    }

    pub fn get_free_squares(&self) -> Bitboard {
        let mut free_squares: Bitboard = 0;
        for i in 0..PIECE_TYPES_NUM - 1 {
            // For each step its gettiing bitmaps of white and black pieces, bitwise OR gives 
            // all the squares that are occupied by this pieces. Bitvise NOT gives all the free once
            // And we are making OR with all free squares until all pieces are accounted for. 
            let white: Bitboard = Some(self.white_pieces[i]);
            let black: Bitboard = self.black_pieces[i].unwrap();

            free_squares |= !(white | black);
            
            
        }
        free_squares
    }
}

pub trait ChessPiece {
    fn get_legal_moves(&self, board: &Board) -> Bitboard;
}

