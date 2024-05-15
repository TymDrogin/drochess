mod gamestate;
mod utils;
use crate::board::defs::*;
use crate::utils::print_bitboard;
use crate::gamestate::Gamestate;
fn main() {
    let board: Bitboard = 0;
    print_bitboard(board);
}
