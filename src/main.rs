mod gamestate;
mod utils;
use core::mem;

use utils::fen::*;

use crate::gamestate::Gamestate;

fn main() {
    println!("Size of MyStruct: {} bytes", mem::size_of::<Gamestate>());
}
