use crate::gamestate::board::*;
use crate::gamestate::chess_move::*;
use crate::gamestate::defs::*;
use crate::gamestate::*;

use castling_rights::CastlingSide;
use lazy_static::lazy_static;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

const SEED: u64 = 1231231;
lazy_static! {
    // The map looks like this - side of the piece - square on which piece is located - type of the piece, resulting in 768 random values for each possible combination
    static ref PIECE_HASHES: [[u64; PIECE_TYPES_NUM * 2]; BOARD_NUM_OF_SQUARES] = generate_pieces_hashes(SEED);
    static ref SIDE_HASHES: [u64; SIDE_NUM] = generate_side_hashes(SEED);
    static ref CASTLING_HASHES: [u64; CASTLING_CONFIGURATIONS_NUM] = generate_castling_hashes(SEED);
    static ref EN_PASSANT_HASHES: [u64; BOARD_SIDE_LENGTH] = generate_enpassant_hashes(SEED);
}

const WHITE_SIDE: usize = 0;
const BLACK_SIDE: usize = 1;
// Castling initial positions
const WHITE_KING_STARTING_INDEX: u8 = 4;
const BLACK_KING_STARTING_INDEX: u8 = 60;
// -- kingside
const WHITE_ROOK_KINGSIDE_STATING_INDEX: u8 = 7;
const BLACK_ROOK_KINGSIDE_STATING_INDEX: u8 = 63;
// --queenside
const WHITE_ROOK_QUEENSIDE_STARTING_INDEX: u8 = 0;
const BLACK_ROOK_QUEENSIDE_STARTING_INDEX: u8 = 56;

// Castling end positions
// -- kings kingside
const WHITE_KING_KINGSIDE_END_INDEX: u8 = 6;
const BLACK_KING_KINGSIDE_END_INDEX: u8 = 62;
// -- kings queenside
const WHITE_KING_QUEENSIDE_END_INDEX: u8 = 2;
const BLACK_KING_QUEENSIDE_END_INDEX: u8 = 58;
// -- rooks kingside
const WHITE_ROOK_KINGSIDE_END_INDEX: u8 = 5;
const BLACK_ROOK_KINGSIDE_END_INDEX: u8 = 61;
// -- rooks queenside
const WHITE_ROOK_QUEENSIDE_END_INDEX: u8 = 3;
const BLACK_ROOK_QUEENSIDE_END_INDEX: u8 = 59;

pub struct Zobrist;
impl Zobrist {
    // Note that this function generetes always a new hash, incremental hash updates will be added soon
    pub fn hash(game: &Gamestate) -> u64 {
        let mut hash: u64 = 0;
        // xor all pieces

        // 0..5 = White pieces, 6..11 = Black pieces
        for i in 0..6 {
            // Hash white pieces            
            if game.board.pieces[i] != 0 {
                for sq in game.board.get_squares_of(PieceType::from_u8(i as u8), Side::White) {
                    let piece_hash = PIECE_HASHES[sq.get_index()][i];
                    hash ^= piece_hash;
                };
            }
            // Hash black pieces
            let black_index = i + PIECE_TYPES_NUM; // 6..11
            if game.board.pieces[black_index] != 0 {
                for sq in game.board.get_squares_of(PieceType::from_u8(i as u8), Side::Black) {
                    let piece_hash = PIECE_HASHES[sq.get_index()][black_index];
                    hash ^= piece_hash;
                };
            }
        }
        // xor castling rights
        hash ^= CASTLING_HASHES[game.castling_rights.as_u8() as usize];

        // xor en passant square
        if game.en_passant != 0 {
            let ep_file = game.en_passant.trailing_zeros() as usize;
            hash ^= EN_PASSANT_HASHES[ep_file];
        }
        // xor side to move
        hash ^= SIDE_HASHES[game.side_to_move as usize];


        hash
    }
    /// This function is used to update the hash of the game state after a move is made.
    /// It takes the current game state and the move that was made, and updates the hash accordingly.
    /// Used in transposition tables to quickly check if a position has been seen before.
    pub fn incremental_hash(game: &mut Gamestate, c_move: &Move) {

    }

}


// This functiond are used to generete static arrays of random values
fn generate_pieces_hashes(seed: u64) -> [[u64; PIECE_TYPES_NUM * 2]; BOARD_NUM_OF_SQUARES] {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut array: [[u64; 12]; 64]= [[0; PIECE_TYPES_NUM * 2]; BOARD_NUM_OF_SQUARES];

    let mut i = 0;
    while i < BOARD_NUM_OF_SQUARES {
        let mut j = 0;
        while j < PIECE_TYPES_NUM * 2 {
            let hash = rng.gen();
            array[i][j] = hash;
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
    while i < BOARD_SIDE_LENGTH {
        let hash = rng.gen();
        array[i] = hash;
        i += 1;
    }
    array
}
