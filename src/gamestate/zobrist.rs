use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use once_cell::sync::Lazy;

use crate::gamestate::board::*;
use crate::gamestate::chess_move::*;
use crate::gamestate::constants::*;
use crate::gamestate::*;
use castling_rights::CastlingSide;


const SEED: u64 = 1231231;

static PIECE_HASHES: Lazy<[[u64; PIECE_TYPES_NUM * 2]; BOARD_NUM_OF_SQUARES]> =
    Lazy::new(|| generate_pieces_hashes(SEED));

static SIDE_HASHES: Lazy<[u64; SIDE_NUM]> =
    Lazy::new(|| generate_side_hashes(SEED));

static CASTLING_HASHES: Lazy<[u64; CASTLING_CONFIGURATIONS_NUM]> =
    Lazy::new(|| generate_castling_hashes(SEED));

static EN_PASSANT_HASHES: Lazy<[u64; BOARD_SIDE_LENGTH]> =
    Lazy::new(|| generate_enpassant_hashes(SEED));

pub struct Zobrist;
impl Zobrist {
    // Note that this function generetes always a new hash, incremental hash updates will be added soon
    pub fn hash(game: &Gamestate) -> u64 {
        let mut hash: u64 = 0;
        // xor all pieces

        // 0..5 = White pieces, 6..11 = Black pieces
        for i in 0..6 {
            let piece_type = PieceType::from_u8(i as u8);

            // White pieces
            let mut bb_white = game.board.get_bitboard_of(piece_type, Side::White);
            while bb_white != 0 {
                let sq_index = bb_white.trailing_zeros() as usize;
                bb_white &= bb_white - 1; 
                hash ^= PIECE_HASHES[sq_index][i];
            }

            // Black pieces
            let mut bb_black = game.board.get_bitboard_of(piece_type, Side::Black);
            let black_index = i + PIECE_TYPES_NUM; // 6..11
            while bb_black != 0 {
                let sq_index = bb_black.trailing_zeros() as usize;
                bb_black &= bb_black - 1;
                hash ^= PIECE_HASHES[sq_index][black_index];
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


}


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
