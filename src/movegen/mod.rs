mod constants;
pub mod masks;

use crate::gamestate::{
    board::*,
    castling_rights::*,
    chess_move::{Move, MoveFlags},
    constants::*,
    Gamestate,
};
use constants::*;
use masks::*;

pub struct MoveGen<'a> {
    game: &'a mut Gamestate,
    moves: Vec<Move>,
}


// Functior order: Pawn, Knight, Bishop, Rook, Queen, King - for the sake of consistency with the piece types.

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
        self.generate_king_quiet_moves();
        let mv = self.moves.clone();
        mv
    }

    fn generate_pawn_pushes(&mut self) {
        let pawns = self
            .game
            .board
            .get_squares_of(PieceType::Pawn, self.game.side_to_move);
        let combined_occupancy = self.game.board.occupancy[0] | self.game.board.occupancy[1];

        for sq in pawns {}

        todo!()
    }
    fn generate_knight_quiet_moves(&mut self) {
        let knights = self
            .game
            .board
            .get_squares_of(PieceType::Knight, self.game.side_to_move);
        let combined_occupancy = self.game.board.occupancy[0] | self.game.board.occupancy[1];

        for sq in knights {
            let attacks = KNIGHT_ATTACKS[sq.get_index() as usize];
            let mut quiet_mask = attacks & !combined_occupancy;

            while quiet_mask != 0 {
                let target_square = Square::new(quiet_mask.trailing_zeros() as u8);
                quiet_mask &= quiet_mask - 1; // Clear the least significant bit
                self.moves
                    .push(Move::encode(sq, target_square, MoveFlags::Quiet));
            }
        }
    }
    fn generate_king_quiet_moves(&mut self) {
        let king_mask = self.game.board.pieces[PieceType::King as usize + (self.game.side_to_move as usize * PIECE_TYPES_NUM)];
        let sq = Square::new(king_mask.trailing_zeros() as u8);

        let combined_occupancy = self.game.board.occupancy[0] | self.game.board.occupancy[1];
        let attacks = KING_ATTACKS[sq.get_index() as usize];
        let mut quiet_mask = attacks & !combined_occupancy;

        while quiet_mask != 0 {
            let target_square = Square::new(quiet_mask.trailing_zeros() as u8);
            quiet_mask &= quiet_mask - 1; // Clear the least significant bit
            self.moves
                .push(Move::encode(sq, target_square, MoveFlags::Quiet));
        }  
    }

}
