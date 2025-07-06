mod gamestate;
use crate::gamestate::Gamestate;
mod movegen;
mod utils;

mod engine;

use board::Side;
use board::Square;
use movegen::constants::*;
use movegen::*; // Assuming movegen is in your current crate or correctly referenced

use gamestate::*;
use gamestate::board::{PieceType, Bitboard};
use utils::display::*;
use utils::fen::*;

fn print_bitboard(bitboard: u64) {
    for rank in (0..8).rev() {
        // Ranks from 7 to 0 (a8 to h8)
        for file in 0..8 {
            // Files from 0 to 7 (a to h)
            let square_index = Square::new_from_file_rank(file, rank).get_index();
            let mask = 1u64 << square_index;
            let is_set = (bitboard & mask) != 0;
            print!("{} ", if is_set { '*' } else { '.' });
        }
        println!(); // Newline after each rank
    }
}

fn main() {
    let pawn_double_push_fen: &str = "rnbqkbnr/8/8/p6p/1ppp4/4ppp1/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let castling_fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1";
    let mut game: Gamestate = Fen(castling_fen.to_string()).process().unwrap();
    let moves = MoveGen::new(&mut game).generate_moves();
    print!("The total moves at this position is: {}", moves.len());
    println!();
    for chm in &moves {
        println!("{}", chm);
    }
    print!("King is at square {}" ,game.board.get_piece_at_square(Square::new(4)).unwrap().0 as usize);

    println!("{}", game);
}
