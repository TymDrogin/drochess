pub mod gamestate;
mod utils;
use utils::fen::*;

fn main() {
    let fen = Fen(DEFAULT_FEN.to_string());
    let game = fen.process().unwrap();
    print!("{}", game);
}
