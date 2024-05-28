pub mod gamestate;
mod utils;
use utils::fen::*;

fn main() {
    let fen = Fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2".to_string());
    let game = fen.process().unwrap();
    print!("{}", game);

    let fen = Fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR b KQkq c6 0 2".to_string());
    let game = fen.process().unwrap();
    print!("{}", game);

    let fen = Fen(DEFAULT_FEN.to_string());
    let game = fen.process().unwrap();
    print!("{}", game);
}
