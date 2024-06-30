pub mod defs;
pub mod magic_bitboards;
pub mod masks;

use crate::gamestate::{
    board::*,
    castling_rights::*,
    Gamestate,
    Move, 
    MoveFlags, 
    defs::*,
};
use defs::*;
use masks::*;
use rayon::prelude::*;


pub struct MoveGen<'a>{
    game: &'a mut Gamestate,
    combined_occupancy: Bitboard,
    white_occupancy: Bitboard,
    black_occupancy: Bitboard,
}

// There is a lot of functions here that are used for getting the moves. For the sake of differentiation,
// any function that starts with get - gives you a pseudo legal moves, and, any that starts with generete - gives you legal moves.
impl<'a> MoveGen<'a> {

    pub fn new(game: &'a mut Gamestate) -> Self {   
        let white_occupancy: Bitboard = game.board.white_pieces.par_iter()
            .fold(|| 0, |acc, &piece| acc | piece)
            .reduce(|| 0, |acc, piece| acc | piece);

        let black_occupancy: Bitboard = game.board.black_pieces.par_iter()
            .fold(|| 0, |acc, &piece| acc | piece)
            .reduce(|| 0, |acc, piece| acc | piece);

        let combined_occupancy: Bitboard = white_occupancy | black_occupancy;

        Self {
            game,
            combined_occupancy,
            white_occupancy,
            black_occupancy
        }
    }

    pub fn gererate(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        //moves.extend(self.generate_king_moves());
        moves.extend(self.get_knight_moves());
        moves
    }

    fn filter_valid_moves(&self, mut moves: &Vec<Move>) {
        todo!()
    }

    fn get_king_moves(&self) -> Vec<Move> {
        self.get_basic_moves_for_pieces(PieceType::King, &KING_ATTAKS_MASKS)
    }
    fn get_queen_moves(&self) -> Vec<Move> {
        // Implement queen move generation logic here
        todo!()
    }
    fn get_rook_moves(&self) -> Vec<Move> {
        // Implement rook move generation logic here
        todo!()
    }
    fn get_bishop_moves(&self) -> Vec<Move> {
        // Implement bishop move generation logic here
        todo!()
    }
    fn get_knight_moves(&self) -> Vec<Move> {
        self.get_basic_moves_for_pieces(PieceType::Knight, &KNIGHT_ATTACKS_MASKS)
    }
    fn get_pawn_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        todo!()
    }
    fn get_castling_moves(&self) -> Vec<Move> {
        // Implement castling move generation logic here
        todo!()
    }
    fn get_en_passant_moves(&self) {  
        // Implement en passant move generation logic here
    }

    // Basic moves generetes pseudo legal quiet and capture moves for pieces with attack masks.
    // For now, this function used only for knights and kings. In the case of the pawns they have different masks for attacks and captures,
    // Which creates the need for having separated functions for capture and quiet moves. I don't know yet if it will be possible to use them with sliding pieces, but for now that is the drill
    #[inline(always)]
    fn get_basic_moves_for_pieces(&self, pieces_to_move: PieceType, attack_masks: &[Bitboard; 64]) -> Vec<Move> {
        let mut moves = Vec::new(); 
        moves.extend(self.get_capture_moves_for_pieces(pieces_to_move, attack_masks));
        moves.extend(self.get_quiet_moves_for_pieces(pieces_to_move, attack_masks));  
        
        moves
    }
    #[inline(always)]
    fn get_capture_moves_for_pieces(&self, pieces_to_move: PieceType, attack_masks: &[Bitboard; 64]) -> Vec<Move> {
        match self.game.side_to_move {
            Side::White => {        
                let squares_with_pieces_to_move: Vec<Square> = Square::get_squares_from_bitboard(self.game.board.white_pieces[pieces_to_move as usize]);

                squares_with_pieces_to_move.par_iter().flat_map(|&from| {
                    let capture_moves_bitboard = (attack_masks[from.get_index()] & self.black_occupancy) & !self.white_occupancy;
                    let capture_moves_squares = Square::get_squares_from_bitboard(capture_moves_bitboard);
        
                    capture_moves_squares.into_par_iter().map(move |capture| 
                        Move::encode(from, capture, MoveFlags::Capture)
                    )
                }).collect()
            },
            Side::Black => {        
                let squares_with_pieces_to_move: Vec<Square> = Square::get_squares_from_bitboard(self.game.board.black_pieces[pieces_to_move as usize]);

                squares_with_pieces_to_move.par_iter().flat_map(|&from| {
                    let capture_moves_bitboard = (attack_masks[from.get_index()] & self.white_occupancy) & !self.black_occupancy;
                    let capture_moves_squares = Square::get_squares_from_bitboard(capture_moves_bitboard);
        
                    capture_moves_squares.into_par_iter().map(move |capture| 
                        Move::encode(from, capture, MoveFlags::Capture)
                    )
                }).collect()
            },
        }
    }
    #[inline(always)]
    fn get_quiet_moves_for_pieces(&self, pieces_to_move: PieceType, attack_masks: &[Bitboard; 64]) -> Vec<Move>{
        let squares_with_pieces_to_move: Vec<Square> = match self.game.side_to_move {
            Side::White => Square::get_squares_from_bitboard(self.game.board.white_pieces[pieces_to_move as usize]),
            Side::Black => Square::get_squares_from_bitboard(self.game.board.black_pieces[pieces_to_move as usize]),
        };

        squares_with_pieces_to_move.par_iter().flat_map(|&from| {
            let quiet_moves_bitboard = attack_masks[from.get_index()] & !self.combined_occupancy;
            let quiet_moves_squares = Square::get_squares_from_bitboard(quiet_moves_bitboard);
            quiet_moves_squares.into_par_iter().map(move |quiet| 
                Move::encode(from, quiet, MoveFlags::Quiet)
            )
        }).collect()
    }
}