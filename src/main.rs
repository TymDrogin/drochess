mod gamestate;
use crate::gamestate::Gamestate;
mod movegen;
use board::Square;
use movegen::*; // Assuming movegen is in your current crate or correctly referenced
use movegen::defs::*;
mod utils;
use utils::fen::*;
use gamestate::*;

fn main() {
    let mut game: Gamestate = Fen(DEFAULT_FEN.to_string()).process().unwrap();
    let moves = MoveGen::new(&mut game).gererate();
    print!("The move counter for pawns is {}", moves.len());
    println!();
    for mov in moves {
        print!("{}", mov.to.to_algebraic_notation());
        println!();
    }
}

