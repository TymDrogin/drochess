pub mod defs;
pub mod masks;

use crate::gamestate::{
    board::*,
    castling_rights::*,
    Gamestate,
    chess_move::{Move, MoveFlags},
    defs::*,
};
use defs::*;
use masks::*;


pub struct MoveGen<'a>{
    game: &'a mut Gamestate,
    moves: Vec<Move>,
}

// There is a lot of functions here that are used for getting the moves. For the sake of differentiation,
// any function that starts with get - gives you a pseudo legal moves, and, any that starts with generete - gives you legal moves.
impl<'a> MoveGen<'a> {

    pub fn new(game: &'a mut Gamestate) -> Self {   
        Self {
            game,
            moves: Vec::new(),
        }
    }

    pub fn generate_moves(&mut self) -> Vec<Move> {
        self.moves.clear();
        self.generate_knight_quiet_moves();
        let mv = self.moves.clone();
        mv
    }

    fn generate_quiet_pawn_moves(&mut self) {
        let pawns = self.game.board.get_squares_of(PieceType::Pawn, self.game.side_to_move);
        let combined_occupancy = self.game.board.occupancy[0] | self.game.board.occupancy[1];

        for sq in pawns {

        }
    }
    fn generate_knight_quiet_moves(&mut self) {
        let knights = self.game.board.get_squares_of(PieceType::Knight, self.game.side_to_move);
        let combined_occupancy = self.game.board.occupancy[0] | self.game.board.occupancy[1];

        for sq in knights {
            let attacks = KNIGHT_ATTACKS[sq.get_index() as usize];
            let mut attacks_mask = attacks & !combined_occupancy;
            if attacks_mask == 0 {
                continue; // No valid moves for this knight
            }
            
            while attacks_mask != 0 {
                let target_square = Square::new(attacks_mask.trailing_zeros() as u8);
                attacks_mask &= attacks_mask - 1; // Clear the least significant bit
                self.moves.push(Move::encode(sq, target_square, MoveFlags::Quiet));
            }
        }
    }


}