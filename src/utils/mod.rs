use crate::board::defs::Bitboard;

pub fn print_bitboard(bitboard: Bitboard) {
    let bytes = bitboard.to_ne_bytes();
    
    for byte in bytes {
        for i in 0..8 {
            let bit = (byte >> i) & 1;
            print!("{} ", bit);
        }
        println!();
    }
}