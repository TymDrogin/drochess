mod gamestate;
use crate::gamestate::Gamestate;
mod movegen;
mod utils;

mod engine;

use board::Side;
use board::Square;

use movegen::constants::*;
use movegen::*; // Assuming movegen is in your current crate or correctly referenced
use movegen::sliders::calculate_rook_attacks;

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


    let rook_sq = Square::new_from_file_rank(3, 3);
    let occ_empty = 0;
    let attacks = calculate_rook_attacks(rook_sq, occ_empty);
    println!("Rook attacks from d4 on empty board:");
    print_bitboard(attacks);

    // 2) Visual test: rook on a1 blocked by pieces on a3 and c1
    let rook_sq = Square::new_from_file_rank(0, 0);
    let blockers = 
        Square::new_from_file_rank(0, 2).get_mask() | // a3
        Square::new_from_file_rank(2, 0).get_mask(); // c1;

    let attacks = calculate_rook_attacks(rook_sq, blockers);
    println!("\nRook attacks from a1 with blockers on a3 & c1:");
    print_bitboard(attacks);
}
