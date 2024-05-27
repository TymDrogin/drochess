pub const PIECE_TYPES_NUM: usize = 6;
pub const BOARD_SIDE_LENGHT: usize = 8;

pub type Bitboard = u64;


#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub white_pieces: [Bitboard; PIECE_TYPES_NUM],
    pub black_pieces: [Bitboard; PIECE_TYPES_NUM],
}
impl Board {
    pub fn new() -> Self {
        Self {
            white_pieces:[0; PIECE_TYPES_NUM],
            black_pieces:[0; PIECE_TYPES_NUM],
        }
    }
    pub fn set_square(&mut self, square: Square, pt: PieceType, side: Side) {
        let mask: Bitboard = 1 << square.get_index();
        
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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square(u8);
impl Square {
    pub fn new(index: u8) -> Self {
        if index > 63 {
            panic!("Attempted to create square with index {}, which is more then max of 63", index);
        }
        Self(index)
    }
    pub fn new_from_file_rank(file: u8, rank: u8) -> Self {
        if rank > 7 || file > 7 {
            panic!("Attempted to create square with file: {} or rank: {} vith values more then 7", file, rank);
        }
        // Since values from 0 to 7 can be stored in only 3 bits (4 for ease of use) we can store file and rank as a single u8 value
        // Example: 00100011 -> 00
        
        // << 3 is equal to 2^3 or 8, but faster to compute (like its even matter aha)
        let rank_file_as_index: u8 = (rank << 3) + file;
        Square(rank_file_as_index)
    }
    pub fn new_from_algebraic_notation(coords: &str) -> Self {
        let file = match coords.chars().nth(0).unwrap() {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!("Invalid file in coordinates"),
        };
        let rank = match coords.chars().nth(1).unwrap() {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => panic!("Invalid rank in coordinates"),
        };
        Self::new_from_file_rank(file, rank)
    }
    pub fn get_index(&self) -> usize {
        self.0 as usize
    }
    pub fn get_file_rank(&self) -> (u8, u8) {
        let rank: u8 = self.0 >> 3;
        let file: u8 = self.0 % BOARD_SIDE_LENGHT as u8;
        
        (file, rank)
    }
}