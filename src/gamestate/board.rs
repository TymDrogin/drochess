pub const PIECE_TYPES_NUM: usize = 6;
pub const BOARD_SIDE_LENGTH: u8 = 8;

pub type Bitboard = u64;

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub white_pieces: [Bitboard; PIECE_TYPES_NUM],
    pub black_pieces: [Bitboard; PIECE_TYPES_NUM],
}
impl Board {
    pub fn new() -> Self {
        Self {
            white_pieces: [0; PIECE_TYPES_NUM],
            black_pieces: [0; PIECE_TYPES_NUM],
        }
    }
    pub fn set_square(&mut self, square: Square, pt: PieceType, side: Side) {
        let piece_mask = square.get_mask();

        match side {
            Side::White => self.white_pieces[pt as usize] |= piece_mask,
            Side::Black => self.black_pieces[pt as usize] |= piece_mask,
        }
    }
    pub fn get_piece_at_square(&self, square: Square) -> Option<(PieceType, Side)> {
        let piece_mask = square.get_mask();

        for i in 0..=PIECE_TYPES_NUM - 1 {
            if piece_mask & self.white_pieces[i] != 0 {
                return Some((PieceType::new_from_usize(i), Side::White));
            }
            if piece_mask & self.black_pieces[i] != 0 {
                return Some((PieceType::new_from_usize(i), Side::Black));
            }
        }
        return None
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
impl PieceType {
    pub fn new_from_usize(i: usize) -> Self {
        match i {
            0 => PieceType::Pawn,
            1 => PieceType::Knight,
            2 => PieceType::Bishop,
            3 => PieceType::Rook,
            4 => PieceType::Queen,
            5 => PieceType::King,
            _ => panic!("Invalid int to piece type convertion"),
        }
    }
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
            panic!(
                "Attempted to create square with index {}, which is more then max of 63",
                index
            );
        }
        Self(index)
    }
    // NOTE: Unlike in algebraic notation where files and ranks are from 1 to 8, 
    // this function accepts values from 0 to 7, because it simplify using it in the loops
    pub fn new_from_file_rank(file: u8, rank: u8) -> Option<Square> {
        if rank > 7 || file > 7 {
            return None;
        }

        // << 3 is equal to 2^3 or 8, but faster to compute (like its even matter aha)
        let rank_file_as_index: u8 = (rank << 3) + file;
        Some(Square(rank_file_as_index))
    }
    pub fn new_from_algebraic_notation(coords: &str) -> Option<Square> {
        let file = match coords.chars().nth(0).unwrap() {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return None,
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
            _ => return None,
        };
        // It does not make sense to unwrup it to then wrap to some,
        // Even that we know that on this stage values for file and rank are 100% legal
        Self::new_from_file_rank(file, rank)
    }
    pub fn get_index(&self) -> usize {
        self.0 as usize
    }
    pub fn get_file_rank(&self) -> (u8, u8) {

        let rank: u8 = self.0 >> 3;
        let file: u8 = self.0 % BOARD_SIDE_LENGTH;

        (file, rank)
    } 
    pub fn get_mask(&self) -> Bitboard {
        ((1 as u64) << (self.0 as u64)) as Bitboard
    }
}