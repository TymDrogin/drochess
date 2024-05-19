mod gamestate;
mod utils;
use utils::fen::*;


fn main() {
    let fen = Fen(DEFAULT_FEN.to_owned());
    fen.process();
    println!("Hello World!");
}
