pub mod constants;
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
        //self.generate_knight_quiet_moves();
        //self.generate_king_quiet_moves();
        //self.generate_pawn_pushes();
        self.generate_castling_moves();
        let mv = self.moves.clone();
        mv
    }
    // Have to substract pawns on the 7th and 2nd ranks, because they can promote
    fn generate_pawn_pushes(&mut self) {
        // Used to clear the 7th rank for white pawns and 2nd rank for black pawns
        // To prevent promotion moves, which are handled separately

        let pawns_mask = self
            .game
            .board
            .get_bitboard_of(PieceType::Pawn, self.game.side_to_move);
        let pawns = Square::from_bitboard(pawns_mask & !CLEAR_7TH_OR_2ND_RANK[self.game.side_to_move as usize]);
        let combined_occupancy = self.game.board.occupancy[0] | self.game.board.occupancy[1];

        // CLEAR THE RANK

        for sq in pawns {
            // This mask is used to check if the pawn can move forward by one square
            let single_push_mask = PAWN_SINGLE_PUSHES[self.game.side_to_move as usize][sq.get_index()] & !combined_occupancy;
            if single_push_mask != 0 {
                let target_square = Square::new(single_push_mask.trailing_zeros() as u8);
                self.moves
                    .push(Move::encode(sq, target_square, MoveFlags::Quiet));
            }
            // If the pawn can move forward by one square, we check if it can move forward by two squares
            // This is only possible if the pawn is on its starting rank
            // And the two square in front of it is empty.
            let double_push_mask = PAWN_DOUBLE_PUSHES[self.game.side_to_move as usize][sq.get_file() as usize] & !combined_occupancy;
            let can_double_push = single_push_mask != 0 && double_push_mask != 0;
            let is_on_start_rank = match self.game.side_to_move {
                Side::White => sq.get_rank() == 1,
                Side::Black => sq.get_rank() == 6,
            };
            if can_double_push && is_on_start_rank {
                let target_mask = double_push_mask;
                let target_square = Square::new(target_mask.trailing_zeros() as u8);
                self.moves
                    .push(Move::encode(sq, target_square, MoveFlags::DoublePawnPush));
            }
        }



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
    fn generate_bishop_quiet_moves(&mut self) {
        todo!()
    }
    fn generate_rook_quiet_moves(&mut self) {
        todo!()
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
    fn generate_queen_quiet_moves(&mut self) {
        todo!()
    }
    fn generete_promotion_quiet_moves(&mut self) {
        todo!()
    }

    fn generate_pawn_capture_moves(&mut self) {
        todo!()
    }
    fn generate_knight_capture_moves(&mut self) {
        todo!()
    }
    fn generate_bishop_capture_moves(&mut self) {
        todo!()
    }
    fn generate_rook_capture_moves(&mut self) {
        todo!()
    }
    fn generate_queen_capture_moves(&mut self) {
        todo!()
    }
    fn generate_promotion_capture_moves(&mut self) {
        todo!()
    }

    
    // This function relies on the castling rights of the game state.
    // If gamestate is not properly updated, it will not check for it.
    // It also does not check if the rook is even at the right square for castling.
    // It only checks if the squares between the king and rook are empty.
    fn generate_castling_moves(&mut self) {
        let rights = self.game.castling_rights;
        
        if !rights.can_castle(self.game.side_to_move) {
            return; // No castling rights for this side
        }

        let combined_occupancy = self.game.board.occupancy[0] | self.game.board.occupancy[1];

        if rights.can_castle_kingside(self.game.side_to_move) {
            // Check if the squares between the king and rook are empty
            let empty_mask = CASTLING_KINGSIDE_OCCUPANCY_MASK[self.game.side_to_move as usize] & combined_occupancy;
            if empty_mask == 0 {
                let king_start = CASTLING_KING_START_INDEX[self.game.side_to_move as usize];
                let king_end = CASTLING_KING_KINGSIDE_END_INDEX[self.game.side_to_move as usize];

                self.moves.push(Move::encode(
                    Square::new(king_start),
                    Square::new(king_end),
                    MoveFlags::KingCastle,
                ));
            }
        }
        if rights.can_castle_queenside(self.game.side_to_move) {
            // Check if the squares between the king and rook are empty
            let empty_mask = CASTLING_QUEENSIDE_OCCUPANCY_MASK[self.game.side_to_move as usize] & combined_occupancy;
            if empty_mask == 0 {
                let king_start = CASTLING_KING_START_INDEX[self.game.side_to_move as usize];
                let king_end = CASTLING_KING_QUEENSIDE_END_INDEX[self.game.side_to_move as usize];

                self.moves.push(Move::encode(
                    Square::new(king_start),
                    Square::new(king_end),
                    MoveFlags::QueenCastle,
                ));
            }
        }
    }


}
