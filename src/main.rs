mod gamestate;
use crate::gamestate::Gamestate;
mod movegen;
mod utils;

mod engine;

use board::Square;
use movegen::*; // Assuming movegen is in your current crate or correctly referenced
use movegen::defs::*;

use utils::fen::*;
use utils::display::*;
use gamestate::*;

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
    print!("{}", MoveDisplayWrapper(moves))
}

