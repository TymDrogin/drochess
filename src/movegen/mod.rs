pub mod defs;
pub mod magic_bitboards;

use crate::gamestate::{
    board::*,
    castling_rights::*,
    Gamestate,
    Move
};
use defs::*;
use rayon::prelude::*;

// Get a mask by using square index
const KNIGHT_ATTACKS_MASK:[Bitboard; 64] = generete_knight_attacks_masks();


pub struct MoveGen<'a>{
    game: &'a mut Gamestate,
    occupancy: Bitboard,
}

impl<'a> MoveGen<'a> {

    pub fn new(game: &'a mut Gamestate) -> Self {
        let occupancy = game.board.white_pieces
            .par_iter()
            .chain(game.board.black_pieces.par_iter())
            .cloned()
            .reduce(|| 0, |acc, x| acc | x);
    
        Self {
            game,
            occupancy
        }
    }

    pub fn gererate(&self) -> Vec<Move> {
        let moves = self.generate_all_moves();
        self.filter_valid_moves(&moves);
        moves
    }

    fn filter_valid_moves(&self, mut moves: &Vec<Move>) {
        todo!()
    }

    fn generate_all_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        self.generate_pawn_moves(&mut moves);
        self.generate_en_passant_moves(&mut moves);
        self.generate_knight_moves(&mut moves);
        self.generate_bishop_moves(&mut moves);
        self.generate_rook_moves(&mut moves);
        self.generate_queen_moves(&mut moves);
        self.generate_king_moves(&mut moves);
        self.generate_castling_moves(&mut moves);

        moves
    }

    fn generate_king_moves(&self, moves: &mut Vec<Move>) {
        // Implement king move generation logic here
    }

    fn generate_queen_moves(&self, moves: &mut Vec<Move>) {
        // Implement queen move generation logic here
    }

    fn generate_rook_moves(&self, moves: &mut Vec<Move>) {
        // Implement rook move generation logic here
    }

    fn generate_bishop_moves(&self, moves: &mut Vec<Move>) {
        // Implement bishop move generation logic here
    }

    fn generate_knight_moves(&self, moves: &mut Vec<Move>) {
        // Implement knight move generation logic here
    }

    fn generate_pawn_moves(&self, moves: &mut Vec<Move>) {
        // Implement pawn move generation logic here
        // Example for adding a move
        todo!()
    }

    fn generate_castling_moves(&self, moves: &mut Vec<Move>) {
        // Implement castling move generation logic here
    }

    fn generate_en_passant_moves(&self, moves: &mut Vec<Move>) {
        // Implement en passant move generation logic here
    }
}


const fn generete_knight_attacks_masks() -> [Bitboard; 64] {
    let mut all_attacks: [Bitboard; 64] = [0; 64];

    let mut i: usize = 0;
    while i < 64 {
        let mut attacks_mask: Bitboard = 0;

        let position_mask = ((1 as u64) << i) as Bitboard;
        /* 
                noNoWe    noNoEa
                    +15  +17
                    |     |
        noWeWe +6 __|     |__ +10 noEaEa
                     \   /
                      >0<
                  __ /   \ __
        soWeWe -10  |     |   -6  soEaEa
                    |     |
                    -17  -15
                soSoWe    soSoEa
        */
        
        // Right side clockwise
        attacks_mask |= (position_mask <<  NO_NO_EA) & NOT_A_FILE;
        attacks_mask |= (position_mask <<  NO_EA_EA) & NOT_AB_FILE;
        attacks_mask |= (position_mask >> -SO_EA_EA) & NOT_AB_FILE;
        attacks_mask |= (position_mask >> -SO_SO_EA) & NOT_A_FILE;

        // Left side clockwise
        attacks_mask |= (position_mask >> -SO_SO_WE) & NOT_H_FILE;
        attacks_mask |= (position_mask >> -SO_WE_WE) & NOT_GH_FILE;
        attacks_mask |= (position_mask <<  NO_WE_WE) & NOT_GH_FILE;
        attacks_mask |= (position_mask <<  NO_NO_WE) & NOT_H_FILE;

        all_attacks[i] = attacks_mask;
        i = i + 1;
    }
    all_attacks
}
