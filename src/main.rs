mod gamestate;
use crate::gamestate::Gamestate;
mod movegen;
mod utils;

mod engine;

use board::Square;
use masks::ROOK_RAYS;
use movegen::*; // Assuming movegen is in your current crate or correctly referenced
use movegen::defs::*;

use utils::fen::*;
use utils::display::*;
use gamestate::*;

fn print_bitboard(bitboard: u64) {
    for rank in (0..8).rev() { // Ranks from 7 to 0 (a8 to h8)
        for file in 0..8 { // Files from 0 to 7 (a to h)
            let square_index = Square::new_from_file_rank(file, rank).unwrap().get_index();
            let mask = 1u64 << square_index;
            let is_set = (bitboard & mask) != 0;
            print!("{} ", if is_set { '*' } else { '.' });
        }
        println!(); // Newline after each rank
    }
}

fn main() {
    let mut game: Gamestate = Fen(DEFAULT_FEN.to_string()).process().unwrap();
    let moves = MoveGen::new(&mut game).gererate();
    print!("The move counter for knights is {}", moves.len());
    println!();
    for mov in &moves {
        print!("{}", mov);
        println!();
    }
    print!("Game hash: {}", game.zobrist_key);
    print!("{}", MoveDisplayWrapper(moves));


}

