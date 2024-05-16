mod gamestate;
mod utils;
use utils::fen::*;


fn main() {
    let fen = Fen(DEFAULT_FEN);
    fen.process();
    println!("Hello World!");
}
