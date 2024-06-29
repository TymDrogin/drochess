pub mod board;
pub mod castling_rights;
use self::{
    board::{Board, Side, Square, PieceType, Bitboard},
    castling_rights::CastlingRights,
};

// This constants are related to moves and their encoding
// Check https://www.chessprogramming.org/Encoding_Moves for details
const MOVE_TO_OFFSET: u16 = 6;
const MOVE_FLAGS_OFFSET: u16 = 12;

const PROMOTION_FLAG_MASK: u16 = 0b1000;
const CAPTURE_FLAG_MASK: u16 =   0b0100;
const SPECIAL1_FLAG_MASK: u16 =  0b0010;
const SPECIAL2_FLAG_MASK: u16 =  0b0001;

const INDEX_MASK: u16 = 0b111111;
const FLAGS_MASK: u16 = 0b1111;


#[derive(Debug, Clone, PartialEq)]
pub struct Gamestate {
    pub board: Board,
    pub side_to_move: Side,
    pub castling_rights: CastlingRights,
    pub en_passant: u8,
    pub half_move_clock: u8,
    pub full_move_count: u8,

    
}
impl Gamestate {
    pub fn apply_move(&mut self, mov: Move) {
        todo!()
    }
    pub fn undo_move(&mut self, mov: Move) {
        todo!()
    }
    
}


// Structure of the moves is 4 flag bits, 6 bits for the index of square to move, and 6 bits for index of square to move to
// ****  ******  ****** - Total of 16 bits 
// flags toIndex fromIndex

pub struct Move(u16);
impl Move {
    #[inline(always)]
    fn encode(from: Square, to: Square, flags: MoveFlags) -> Move {
        Self(
            (from.get_index() as u16) 
            | (to.get_index() as u16) << MOVE_TO_OFFSET
            | (flags as u16) << MOVE_FLAGS_OFFSET
        )
    }
    #[inline(always)]
    fn decode(&self) -> (MoveFlags, Square, Square) { // Flags, Square
        (self.get_flags(), self.get_from_square(), self.get_to_square())
    }

    #[inline(always)]
    fn get_from_square(&self) -> Square {
        Square::new((self.0 & 0b111111) as u8)
    }
    #[inline(always)]
    fn get_to_square(&self) -> Square {
        Square::new(((self.0 >> MOVE_TO_OFFSET) & 0b111111) as u8)
    }
    #[inline(always)]
    fn get_flags(&self) -> MoveFlags {
        MoveFlags::from_u8(((self.0 >> MOVE_FLAGS_OFFSET) & 0b001111) as u8)
    }

    #[inline(always)]
    fn is_capture(&self) -> bool {
        let flags = self.get_flags() as u16;
        (flags & CAPTURE_FLAG_MASK) != 0
    }
    #[inline(always)]
    fn is_promotion(&self) -> bool {
        let flags = self.get_flags() as u16;
        (flags & PROMOTION_FLAG_MASK) != 0
    }
    #[inline(always)]
    fn is_promo_capture(&self) -> bool {
        let flags = self.get_flags() as u16;
        (flags & PROMOTION_FLAG_MASK != 0) && (flags & CAPTURE_FLAG_MASK != 0)
    }
    #[inline(always)]
    fn is_castle(&self) -> bool {
        let flags = self.get_flags() as u16;
        (flags & PROMOTION_FLAG_MASK == 0) && (flags & SPECIAL1_FLAG_MASK != 0)
    }

}


#[derive(Debug)]
pub enum MoveFlags {
    QuietMove =          0b0000, // 0
    DoublePawnPush =     0b0001, // 1
    KingCastle =         0b0010, // 2
    QueenCastle =        0b0011, // 3    
    Capture =            0b0100, // 4 
    EpCapture =          0b0101, // 5
    // All promotions
    KnightPromotion =    0b1000, // 8
    BishopPromotion =    0b1001, // 9
    RookPromotion =      0b1010, // 10
    QueenPromotion =     0b1011, // 11
    KnightPromoCapture = 0b1100, // 12
    BishopPromoCapture = 0b1101, // 13
    RookPromoCapture =   0b1110, // 14
    QueenPromoCapture =  0b1111, // 15
}
impl MoveFlags {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0b0000 => MoveFlags::QuietMove,
            0b0001 => MoveFlags::DoublePawnPush,
            0b0010 => MoveFlags::KingCastle,
            0b0011 => MoveFlags::QueenCastle,
            0b0100 => MoveFlags::Capture,
            0b0101 => MoveFlags::EpCapture,
            0b1000 => MoveFlags::KnightPromotion,
            0b1001 => MoveFlags::BishopPromotion,
            0b1010 => MoveFlags::RookPromotion,
            0b1011 => MoveFlags::QueenPromotion,
            0b1100 => MoveFlags::KnightPromoCapture,
            0b1101 => MoveFlags::BishopPromoCapture,
            0b1110 => MoveFlags::RookPromoCapture,
            0b1111 => MoveFlags::QueenPromoCapture,
            _ => unreachable!("Invalid move flag: {:#b}", value),
        }
    }
}
