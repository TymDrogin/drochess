use std::arch::x86_64::_MM_FLUSH_ZERO_ON;

use crate::gamestate::*;
use crate::gamestate::defs::*;
use crate::gamestate::board::*;

use castling_rights::CastlingSide;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rayon::prelude::*;
use lazy_static::lazy_static;

// REFACTOR: Rewrite to use polyglot http://hgm.nubati.net/book_format.html

const SEED: u64 = 1231231;
lazy_static! {
    // The map looks like this - side of the piece - square on which piece is located - type of the piece, resulting in 768 random values for each possible combination
    static ref PIECE_HASHES: [[[u64; PIECE_TYPES_NUM]; BOARD_NUM_OF_SQUARES]; SIDE_NUM] = generate_pieces_hashes(SEED);
    static ref SIDE_HASHES: [u64; SIDE_NUM] = generate_side_hashes(SEED);
    static ref CASTLING_HASHES: [u64; CASTLING_CONFIGURATIONS_NUM] = generate_castling_hashes(SEED);
    static ref EN_PASSANT_HASHES: [u64; BOARD_SIDE_LENGTH] = generate_enpassant_hashes(SEED);
}
const WHITE_SIDE: usize = 0;
const BLACK_SIDE: usize = 1;

const WHITE_KINGSIDE_ROOK_INDEX: usize = 0;
const WHITE_QUEENSIDE_ROOK_INDES: usize = 0;
const BLACK_KINGSIDE_ROOK_INDEX: usize = 0;
const BLACK_QUEENSIDE_ROOK_INDES: usize = 0;
const WHITE_KING_STARTING_INDEX: usize = 0;
const BLACK_KING_STARTING_INDEX: usize = 0;




pub struct Zobrist;
impl Zobrist {
    // Note that this function generetes always a new hash, incremental hash updates will be added soon
    pub fn hash(game: &Gamestate) -> u64 {
        let mut zobrist_key:u64 = 0;

        // Pieces
        let piece_hashes: u64 = (0..BOARD_NUM_OF_SQUARES)
            .into_par_iter()
            .filter_map(|i| {
                // Get the piece at the given square
                let square = Square::new(i as u8);
                let piece = game.board.get_piece_at_square(square);
                // Map the piece to its corresponding Zobrist hash
                piece.map(|(piece_type, side)| 
                    PIECE_HASHES[side as usize][square.get_index()][piece_type as usize]
                )
            })
            .reduce(|| 0, |acc, x| acc ^ x);

        zobrist_key ^= piece_hashes;

        // Side to move
        match game.side_to_move {
            Side::White => {
                zobrist_key ^= SIDE_HASHES[0];
            }, 
            Side::Black => {
                zobrist_key ^= SIDE_HASHES[1];
            },
        }

        // Castling rights
        zobrist_key ^= CASTLING_HASHES[game.castling_rights.get() as usize];

        // En passant
        for i in 0..BOARD_SIDE_LENGTH {
            if ((1 << i) & game.en_passant as usize) != 0 {
                zobrist_key ^= EN_PASSANT_HASHES[i];
            }
        }

        zobrist_key // Return the Zobrist hash key
    }

    // Check this for more info https://www.chessprogramming.org/Incremental_Updates
    // This function should be used BEFORE applying the move to the gamestate
    pub fn icremental_hash_update(game: &Gamestate, mov: &Move) -> u64 {

        let side_to_move = game.side_to_move;
        let square_from = mov.get_from_square();
        let square_to = mov.get_to_square();
        let piece_moved = game.board.get_piece_at_square(square_from).unwrap().0;
        
        let mut new_zobrist_key = game.zobrist_key;

        // Side part of hash
        update_side_hash(&mut new_zobrist_key, side_to_move);
        match mov.get_flags() {
            MoveFlags::Quiet | MoveFlags::DoublePawnPush => {
                update_piece_moved_hash(&mut new_zobrist_key, piece_moved, square_from, square_to, side_to_move);
                return new_zobrist_key
            },
            MoveFlags::Capture => {
                // Clear old positon and set new piece hash
                update_piece_moved_hash(&mut new_zobrist_key, piece_moved, square_from, square_to, side_to_move);

                // Clear captured piece hash
                let captured_piece: PieceType = game.board.get_piece_at_square(square_to).unwrap().0;
                update_captured_piece_hash(&mut new_zobrist_key, side_to_move, square_to, captured_piece);
                return new_zobrist_key
            },
            MoveFlags::EpCapture => {
                todo!()
            }
            MoveFlags::KingCastle => {
                todo!();
            },
            MoveFlags::QueenCastle => {
                todo!();
            },
            // Quiet promotions
            MoveFlags::QueenPromotion | MoveFlags::KnightPromotion | MoveFlags::BishopPromotion | MoveFlags::RookPromotion => {
                // Clear old piece position hash 
                new_zobrist_key ^= PIECE_HASHES[side_to_move as usize][square_from.get_index()][piece_moved as usize];

                // Set promoted piece hash
                new_zobrist_key ^= PIECE_HASHES[side_to_move as usize][square_to.get_index()][mov.get_flags().get_promotion_piece().unwrap() as usize];
                
                return new_zobrist_key;
            },
            // Capture promotions 
            MoveFlags::QueenPromoCapture | MoveFlags::KnightPromoCapture | MoveFlags::BishopPromoCapture | MoveFlags::RookPromoCapture => {
                // Clear old piece position hash 
                new_zobrist_key ^= PIECE_HASHES[side_to_move as usize][square_from.get_index()][piece_moved as usize];

                // Clear captured piece positon hash
                let captured_piece = game.board.get_piece_at_square(square_to).unwrap().0;
                update_captured_piece_hash(&mut new_zobrist_key, side_to_move, square_to, captured_piece);

                // Set promoted piece hash
                new_zobrist_key ^= PIECE_HASHES[side_to_move as usize][square_to.get_index()][mov.get_flags().get_promotion_piece().unwrap() as usize];
                
                return new_zobrist_key;
            },
        }
    }
}
fn update_piece_moved_hash(zobrist_key: &mut u64, piece: PieceType, square_from: Square, square_to: Square, side: Side) {
    // Clear old positon
    *zobrist_key ^= PIECE_HASHES[side as usize][square_from.get_index()][piece as usize];
    // Set new position
    *zobrist_key ^= PIECE_HASHES[side as usize][square_to.get_index()][piece as usize];
}
fn update_side_hash(zobrist_key: &mut u64, side_to_move: Side) {
    match side_to_move {
        Side::White => {
            // Undo white side hash
            *zobrist_key ^= SIDE_HASHES[WHITE_SIDE];
            // Set black side hash
            *zobrist_key ^= SIDE_HASHES[BLACK_SIDE];
        },
        Side::Black => {
            // Undo black side hash
            *zobrist_key ^= SIDE_HASHES[BLACK_SIDE];
            // Set white side hash
            *zobrist_key ^= SIDE_HASHES[WHITE_SIDE];
        },
    }
}
fn update_captured_piece_hash(zobrist_key: &mut u64, side_to_move: Side, positon: Square, captured_piece: PieceType) {
    match side_to_move {
        Side::White => {
            *zobrist_key ^= PIECE_HASHES[BLACK_SIDE][positon.get_index()][captured_piece as usize];
        },
        Side::Black => {
            *zobrist_key ^= PIECE_HASHES[WHITE_SIDE][positon.get_index()][captured_piece as usize];
        }, 
    }
}
fn update_castling_rights_disable_full_side_hash(zobrist_key: &mut u64, side: Side, castling_rights: CastlingRights) {
    // Clear old castling
    *zobrist_key ^= CASTLING_HASHES[castling_rights.get() as usize];

    // Get new rights
    let mut new_rights = castling_rights.clone();
    new_rights.disable_full_side(side);

    // Set new rights
    *zobrist_key ^= CASTLING_HASHES[new_rights.get() as usize];
}
fn update_castling_rights_disable_part_of_side_for_side_to_move_hash(zobrist_key: &mut u64, side: Side, castling_rights: CastlingRights, side_to_disable: CastlingSide) {
  // Clear old castling
  *zobrist_key ^= CASTLING_HASHES[castling_rights.get() as usize];

  // Get new rights
  let mut new_rights = castling_rights;
  new_rights.disable_part_of_side(side, side_to_disable);

  // Set new rights
  *zobrist_key ^= CASTLING_HASHES[new_rights.get() as usize];
}



fn generate_pieces_hashes(seed: u64) -> [[[u64; PIECE_TYPES_NUM]; BOARD_NUM_OF_SQUARES]; SIDE_NUM] {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut array = [[[0; PIECE_TYPES_NUM]; BOARD_NUM_OF_SQUARES]; SIDE_NUM];

    let mut i = 0;
    while i < SIDE_NUM {
        let mut j = 0;
        while j < BOARD_NUM_OF_SQUARES {
            let mut k = 0;
            while k < PIECE_TYPES_NUM {
                let hash = rng.gen();
                array[i][j][k] = hash;
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    array
}
fn generate_side_hashes(seed: u64) -> [u64; SIDE_NUM] {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut array = [0u64; SIDE_NUM];

    let mut i = 0;
    while i < SIDE_NUM {
        let hash = rng.gen();
        array[i] = hash;
        i += 1;
    }
    array
}
fn generate_castling_hashes(seed: u64) -> [u64; CASTLING_CONFIGURATIONS_NUM] {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut array = [0u64; CASTLING_CONFIGURATIONS_NUM];

    let mut i = 0;
    while i < CASTLING_CONFIGURATIONS_NUM {
        let hash = rng.gen();
        array[i] = hash;
        i += 1;
    }
    array
}
fn generate_enpassant_hashes(seed: u64) -> [u64; BOARD_SIDE_LENGTH] {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut array = [0u64; BOARD_SIDE_LENGTH];

    let mut i = 0;
    while i <BOARD_SIDE_LENGTH {
        let hash = rng.gen();
        array[i] = hash;
        i += 1;
    }
    array
}
