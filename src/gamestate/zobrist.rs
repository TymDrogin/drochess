use crate::gamestate::*;
use crate::gamestate::defs::*;
use crate::gamestate::board::*;

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rayon::prelude::*;
use lazy_static::lazy_static;


const SEED: u64 = 1231231;
lazy_static! {
    // The map looks like this - side of the piece - square on which piece is located - type of the piece, resulting in 768 random values for each possible combination
    static ref PIECE_HASHES: [[[u64; PIECE_TYPES_NUM]; BOARD_NUM_OF_SQUARES]; SIDE_NUM] = generate_pieces_hashes(SEED);
    static ref SIDE_HASHES: [u64; SIDE_NUM] = generate_side_hashes(SEED);
    static ref CASTLING_HASHES: [u64; CASTLING_CONFIGURATIONS_NUM] = generate_castling_hashes(SEED);
    static ref EN_PASSANT_HASHES: [u64; BOARD_SIDE_LENGTH] = generate_enpassant_hashes(SEED);
}

pub struct Zobrist;
impl Zobrist {
    pub fn hash(game: &Gamestate) -> u64 {
        let mut zobrist_key:u64 = 0;

        // Pieces
        let piece_hashes: u64 = (0..BOARD_NUM_OF_SQUARES)
            .into_par_iter()
            .filter_map(|i| {
                let piece = game.board.get_piece_at_square(Square::new(i as u8));
                piece.map(|x| match x.1 {
                    Side::White => PIECE_HASHES[0][i][x.0 as usize],
                    Side::Black => PIECE_HASHES[1][i][x.0 as usize],
                })
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

    //Check this for more info https://www.chessprogramming.org/Incremental_Updates
    pub fn icremental_hash_update(game: &Gamestate, mov: &Move) -> u64 {
        let (from_square, to_square, piece, side) = (
            mov.get_from_square(),
            mov.get_to_square(),
            game.board.get_piece_at_square(from),
            game.side_to_move,
        );

        match mov.get_flags() {
            
        }
    }
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