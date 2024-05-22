pub const PIECE_TYPES_NUM: usize = 6;
pub type Bitboard = u64;
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub white_pieces: [Bitboard; PIECE_TYPES_NUM],
    pub black_pieces: [Bitboard; PIECE_TYPES_NUM],
}
impl Board {
    pub fn set_square(&mut self, square: Square, pt: PieceType, side: Side) {
        let mask: Bitboard = 1;
        
        match side {
            Side::White => self.white_pieces[pt as usize] |= mask,
            Side::Black => self.black_pieces[pt as usize] |= mask,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}

const RANK_OFFSET: usize = 4;
pub struct Square(u8);
impl Square {
    fn new(rank: u8, file: u8) -> Self {
        if(rank > 7 || file > 7) {
            panic!("Attempted to create square with file: {} or rank: {} vith values more then 7", file, rank);
        }
        // Since values from 0 to 7 can be stored in only 3 bits (4 for ease of use) we can store file and rank as a single u8 value
        // Example: 00100011 -> 00
        let rank_file_as_index: u8 = rank << 3 + file;
        Self(rank_file_as_index)
    }
    fn get_index(&self) -> usize {
        self.0 as usize
    }
    fn get_file_rank
}

