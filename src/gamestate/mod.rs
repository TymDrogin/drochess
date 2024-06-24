pub mod board;
pub mod castling_rights;


const MOVE_TO_OFFSET: u16 = 6;
const MOVE_FLAG_OFFSET: u16 = 12;

use self::{
    board::{Board, Side, Square, PieceType, Bitboard},
    castling_rights::CastlingRights,
};
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

#[derive(Debug)]
pub enum MoveFlag {
    QuietMove =          0b0000, // 0
    DoublePawnPush =     0b0001, // 1
    KingCastle =         0b0010, // 2
    QueenCastle =        0b0011, // 3
    Capture =            0b0100, // 4
    EpCapture =          0b0101, // 5
    KnightPromotion =    0b1000, // 8
    BishopPromotion =    0b1001, // 9
    RookPromotion =      0b1010, // 10
    QueenPromotion =     0b1011, // 11
    KnightPromoCapture = 0b1100, // 12
    BishopPromoCapture = 0b1101, // 13
    RookPromoCapture =   0b1110, // 14
    QueenPromoCapture =  0b1111, // 15
}
impl MoveFlag {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0b0000 => MoveFlag::QuietMove,
            0b0001 => MoveFlag::DoublePawnPush,
            0b0010 => MoveFlag::KingCastle,
            0b0011 => MoveFlag::QueenCastle,
            0b0100 => MoveFlag::Capture,
            0b0101 => MoveFlag::EpCapture,
            0b1000 => MoveFlag::KnightPromotion,
            0b1001 => MoveFlag::BishopPromotion,
            0b1010 => MoveFlag::RookPromotion,
            0b1011 => MoveFlag::QueenPromotion,
            0b1100 => MoveFlag::KnightPromoCapture,
            0b1101 => MoveFlag::BishopPromoCapture,
            0b1110 => MoveFlag::RookPromoCapture,
            0b1111 => MoveFlag::QueenPromoCapture,
            _ => unreachable!("Invalid move flag: {:#b}", value),
        }
    }
}
pub struct Move(u16);
impl Move {
    fn encode(from: Square, to: Square, flag: MoveFlag) -> Move {
        Self(
            (from.get_index() as u16) 
            | (to.get_index() as u16) << MOVE_TO_OFFSET
            | (flag as u16) << MOVE_FLAG_OFFSET
        )
    }
    fn decode(&self) -> (MoveFlag, Square, Square) { // Flags, Square
        let from_index = (self.0 & 0b111111) as u8;
        let to_index = ((self.0 >> MOVE_TO_OFFSET) & 0b111111) as u8;
        let flag = ((self.0 >> MOVE_FLAG_OFFSET) & 0b001111) as u8;

        (MoveFlag::from_u8(flag), Square::new(from_index), Square::new(to_index))
    }
    fn get_from_index() {
        todo!()
    }
    fn get_to_index() {

    }
    fn get_flags() -> MoveFlag {
        todo!()
    }
}

