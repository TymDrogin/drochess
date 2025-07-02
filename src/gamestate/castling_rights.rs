use crate::gamestate::board::Side;

const CASTLING_SHIFT: u8 = 2;
const CASTLING_SIDE_MASK: u8 = 0b0000_00_11; // 2 bits for each side, 2 bits for white and 2 bits for black

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastlingSide {
    None = 0,      //00000000
    Kingside = 1,  //00000001
    Queenside = 2, //00000010
    Both = 3,      //00000011
}
impl CastlingSide {
    #[inline(always)]
    pub fn from_u8(u: u8) -> CastlingSide {
        match u {
            0 => CastlingSide::None,
            1 => CastlingSide::Kingside,
            2 => CastlingSide::Queenside,
            3 => CastlingSide::Both,
            _ => unreachable!("Invalid castling side from u8 call"),
        }
    }
}

// The encoding is as follows: 0000(garbage bits), 00(black bits), 00(white bits) -> 0000_****
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights(u8);
impl CastlingRights {
    #[inline(always)]
    pub fn new() -> Self {
        CastlingRights(0)
    }

    #[inline(always)]
    pub fn as_u8(&self) -> u8 {
        self.0
    }

    #[inline(always)]
    pub fn set_rights(&mut self, side: Side, rights: CastlingSide) {
        self.0 |= (rights as u8) << Self::shift(side);
    }

    #[inline(always)]
    pub fn get_rights(&self, side: Side) -> CastlingSide {
        let mask = CASTLING_SIDE_MASK << Self::shift(side);

        CastlingSide::from_u8(self.0 & mask >> Self::shift(side))
    }

    #[inline(always)]
    fn shift(side: Side) -> u8 {
        CASTLING_SHIFT * side as u8
    }

    #[inline(always)]
    pub fn disable_side(&mut self, side: Side) {
        let mask = !(CASTLING_SIDE_MASK << Self::shift(side));

        self.0 &= mask;
    }

    #[inline(always)]
    pub fn disable_specific_right(&mut self, side: Side, castling_side_to_disable: CastlingSide) {
        let mask: u8 = !((castling_side_to_disable as u8) << Self::shift(side));

        self.0 &= mask;
    }

    #[inline(always)]
    pub fn clear_all(&mut self) {
        self.0 = 0;
    }
}
