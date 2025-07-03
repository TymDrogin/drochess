use crate::gamestate::defs::*;
use rayon::prelude::*;



// Safe mode of removing something:  pieces =& !mask_of_a_thing to remove
// Unsafe mode of removing something: pieces ^= mask_of_a_thing to remove
// First one works, second one is faster, but can lead to bugs if you use it incorrectly, for example if the piece is not ther already

pub type Bitboard = u64;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
impl PieceType {
    pub fn from_u8(i: u8) -> Self {
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
#[repr(u8)]
pub enum Side {
    White,
    Black,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub pieces: [Bitboard; PIECE_TYPES_NUM * 2], // 0-5 are white pieces, 6-11 are black pieces
    pub occupancy: [Bitboard; 2],                // 0 is white occupancy, 1 is black occupancy
}
impl Default for Board {
    fn default() -> Self {
        Self {
            pieces: [0; PIECE_TYPES_NUM * 2],
            occupancy: [0; 2],
        }
    }
}
impl Board {
    #[inline(always)]
    pub fn get_bitboard_of(&self, pt: PieceType, side: Side) -> Bitboard {
        self.pieces[Self::piece_index(pt, side)]
    }
    #[inline(always)]
    pub fn get_squares_of(&self, pt: PieceType, side: Side) -> Vec<Square> {
        let bitboard = self.get_bitboard_of(pt, side);
        Square::get_squares_from_bitboard(bitboard)
    }
    pub fn get_piece_at_square(&self, square: Square) -> Option<(PieceType, Side)> {
        let piece_mask = square.get_mask();

        self.pieces.iter().enumerate()
            .find(|&(_, &bitboard)| bitboard & piece_mask != 0)
            .map(|(i, _)| {
                let pt = PieceType::from_u8((i % PIECE_TYPES_NUM) as u8);
                let side = if i < PIECE_TYPES_NUM { Side::White } else { Side::Black };
                (pt, side)
            })
    }
    #[inline(always)]
    pub fn place_piece_at_square(&mut self, square: Square, pt: PieceType, side: Side) {
        let piece_mask = square.get_mask();
        self.pieces[Self::piece_index(pt, side)] |= piece_mask;

        self.occupancy[side as usize] |= piece_mask;
    }
    #[inline(always)]
    pub fn remove_piece_at_square(&mut self, square: Square, pt: PieceType, side: Side) {
        let piece_mask = square.get_mask();

        self.pieces[Self::piece_index(pt, side)] ^= piece_mask;

        self.occupancy[side as usize] ^= piece_mask;
    }
    // Cannot be used before calling the capture_piece function
    pub fn move_piece(
        &mut self,
        from_square: Square,
        to_square: Square,
        pt: PieceType,
        side: Side,
    ) {
        let from_mask = from_square.get_mask();
        let to_mask = to_square.get_mask();

        // Remove piece from the old square
        self.pieces[Self::piece_index(pt, side)] ^= from_mask;
        self.occupancy[side as usize] ^= from_mask;

        // Place piece on the new square
        self.pieces[Self::piece_index(pt, side)] |= to_mask;
        self.occupancy[side as usize] |= to_mask;
    }
    
    #[inline(always)]
    pub fn is_square_occupied(&self, square: Square) -> bool {
        let piece_mask = square.get_mask();
        ((self.occupancy[0] | self.occupancy[1]) & piece_mask) != 0
    }
    #[inline(always)]
    pub fn piece_index(pt: PieceType, side: Side) -> usize {
        pt as usize + (side as usize * PIECE_TYPES_NUM)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square(u8);
impl Square {
    pub const fn new(index: u8) -> Self {
        assert!(
            index <= 63,
            "Attempted to create square with index more than max of 63"
        );
        Self(index)
    }
    #[inline(always)]
    pub const fn new_from_file_rank(file: u8, rank: u8) -> Option<Square> {
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
    #[inline(always)]
    pub const fn get_index(&self) -> usize {
        self.0 as usize
    }
    #[inline(always)]
    pub const fn get_file_rank(&self) -> (u8, u8) {
        let rank: u8 = self.0 >> 3;
        let file: u8 = self.0 & 7;

        (file, rank)
    }
    pub fn to_algebraic_notation(&self) -> String {
        let (file, rank) = self.get_file_rank();

        let file_char = match file {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => unreachable!(), // Since file is guaranteed to be within 0..7
        };

        let rank_char = match rank {
            0 => '1',
            1 => '2',
            2 => '3',
            3 => '4',
            4 => '5',
            5 => '6',
            6 => '7',
            7 => '8',
            _ => unreachable!(), // Since rank is guaranteed to be within 0..7
        };

        format!("{}{}", file_char, rank_char)
    }
    #[inline(always)]
    pub const fn get_mask(&self) -> Bitboard {
        ((1 as u64) << (self.0 as u64)) as Bitboard
    }

    #[inline(always)]
    pub fn get_squares_from_bitboard(mut bitboard: Bitboard) -> Vec<Square> {
        let mut squares = Vec::new();
        while bitboard != 0 {
            let idx = bitboard.trailing_zeros() as u8;
            squares.push(Square::new(idx));
            bitboard &= bitboard - 1;
        }
        squares
    }
}
