pub mod defs;
pub mod magic_bitboards;
pub mod masks;

use crate::gamestate::{
    board::*,
    castling_rights::*,
    Gamestate,
    Move, 
    MoveType
};
use defs::*;
use rayon::prelude::*;


pub struct MoveGen<'a>{
    game: &'a mut Gamestate,
    combined_occupancy: Bitboard,
    white_occupancy: Bitboard,
    black_occupancy: Bitboard,
}
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
        moves.extend(self.generate_pawn_moves());
        moves
    }

    fn filter_valid_moves(&self, mut moves: &Vec<Move>) {
        todo!()
    }

    fn generate_king_moves(&self) -> Vec<Move> {
        self.get_basic_moves_for_pieces(PieceType::King, &KING_ATTAKS_MASKS)
    }
    fn generate_queen_moves(&self) -> Vec<Move> {
        // Implement queen move generation logic here
        todo!()
    }
    fn generate_rook_moves(&self) -> Vec<Move> {
        // Implement rook move generation logic here
        todo!()
    }
    fn generate_bishop_moves(&self) -> Vec<Move> {
        // Implement bishop move generation logic here
        todo!()
    }
    fn generate_knight_moves(&self) -> Vec<Move> {
        self.get_quiet_moves_for_pieces(PieceType::Knight, &KNIGHT_ATTACKS_MASKS)
    }
    fn generate_pawn_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let attack_moves = match self.game.side_to_move {
            Side::White => self.get_capture_moves_for_pieces(PieceType::Pawn, &WHITE_PAWN_ATTACKS_MASKS),
            Side::Black => self.get_capture_moves_for_pieces(PieceType::Pawn, &BLACK_PAWN_ATTACKS_MASKS),
        };
        let quiet_moves = match self.game.side_to_move {
            Side::White => self.get_quiet_moves_for_pieces(PieceType::Pawn, &WHITE_PAWN_PUSHES_MASKS),
            Side::Black => self.get_quiet_moves_for_pieces(PieceType::Pawn, &BLACK_PAWN_PUSHES_MASKS),
        };
        moves.extend(attack_moves);
        moves.extend(quiet_moves);
        moves
    }
    fn generate_castling_moves(&self) -> Vec<Move> {
        // Implement castling move generation logic here
        todo!()
    }
    fn generate_en_passant_moves(&self) {  
        // Implement en passant move generation logic here
    }

    // This function is a helper function used for knights, kings and pawns(not handels promotion).
    // In case where we have a location of all pieces(knights for example), we pass a attacks mask.
    // All the pieces have location, so we are ending up with a vector of squares on which pieces are located. 
    // Then, for each square we get all the possible moves, and they are only of two types - Capture or Quiet 
    // PAwns are in thinking
    // REFACTOR: Possible split on two identical in princeple function to avoid match statements everytime 
    #[inline(always)]
    fn get_basic_moves_for_pieces(&self, pieces_to_move: PieceType, attack_masks: &[Bitboard; 64]) -> Vec<Move> {
        let mut moves = Vec::new(); 
        //moves.extend(self.get_capture_moves_for_pieces(pieces_to_move, attack_masks)); 
        moves.extend(self.get_capture_moves_for_pieces(pieces_to_move, attack_masks));
        moves.extend(self.get_quiet_moves_for_pieces(pieces_to_move, attack_masks));  
        
        moves
    }
    #[inline(always)]
    fn get_capture_moves_for_pieces(&self, pieces_to_move: PieceType, attack_masks: &[Bitboard; 64]) -> Vec<Move> {
        match self.game.side_to_move {
            Side::White => self.get_capture_moves_for_white_pieces(pieces_to_move, attack_masks),
            Side::Black => self.get_capture_moves_for_black_pieces(pieces_to_move, attack_masks),
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
            quiet_moves_squares.into_par_iter().map(move |quiet| Move {
                from,
                to: quiet,
                type_of: MoveType::Quiet,
            })
        }).collect()
    }

    // This functions are used to generete moves for piece on the side based on attack tables.
    // It is used for knights, pawns and kings. Sliding pieces requier a lot more additional logic,
    #[inline(always)]
    fn get_capture_moves_for_white_pieces(&self, pieces_to_move: PieceType, attack_masks: &[Bitboard; 64]) -> Vec<Move> {
        let squares_with_pieces_to_move: Vec<Square> = Square::get_squares_from_bitboard(self.game.board.white_pieces[pieces_to_move as usize]);

        squares_with_pieces_to_move.par_iter().flat_map(|&from| {
            let capture_moves_bitboard = (attack_masks[from.get_index()] & self.black_occupancy) & !self.white_occupancy;
            let capture_moves_squares = Square::get_squares_from_bitboard(capture_moves_bitboard);

            capture_moves_squares.into_par_iter().map(move |capture| Move {
                from,
                to: capture,
                type_of: MoveType::Capture,
            })
        }).collect()
    }
    #[inline(always)]
    fn get_capture_moves_for_black_pieces(&self, pieces_to_move: PieceType, attack_masks: &[Bitboard; 64]) -> Vec<Move> {
        let squares_with_pieces_to_move: Vec<Square> = Square::get_squares_from_bitboard(self.game.board.black_pieces[pieces_to_move as usize]);

        squares_with_pieces_to_move.par_iter().flat_map(|&from| {
            let capture_moves_bitboard = (attack_masks[from.get_index()] & self.white_occupancy) & !self.black_occupancy;
            let capture_moves_squares = Square::get_squares_from_bitboard(capture_moves_bitboard);

            capture_moves_squares.into_par_iter().map(move |capture| Move {
                from,
                to: capture,
                type_of: MoveType::Capture,
            })
        }).collect()
    }
}