use crate::gamestate::board::Side;

pub const BLACK_SIDE_OFFSET: u8 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastlingSide {
    None = 0,      //00000000
    Kingside = 1,  //00000001
    Queenside = 2, //00000010
    Both = 3,      //00000011
}
impl CastlingSide {
    pub fn get_from_u8(u: u8) -> Option<CastlingSide> {
        match u {
            0 => Some(CastlingSide::None),
            1 => Some(CastlingSide::Kingside),
            2 => Some(CastlingSide::Queenside),
            3 => Some(CastlingSide::Both),
            _ => None
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

// structure is as follows: 0000(garbage bits), 00(black bits), 00(white bits) -> 0000****
pub struct CastlingRights(u8);
impl CastlingRights {
    pub fn new() -> Self {
        CastlingRights(0)
    }
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn set_for_side(&mut self, side: Side, rights: CastlingSide) {
        match side {
            Side::White => self.0 |= rights as u8,
            Side::Black => self.0 |= (rights as u8) << BLACK_SIDE_OFFSET,
        }
    }
    pub fn get_for_side(&self, side: Side) -> CastlingSide {
        const WHITE_MASK: u8 = 0b0000_00_11; 
        const BLACK_MASK: u8 = 0b0000_11_00;

        match side {
            Side::White => CastlingSide::get_from_u8(self.0 & WHITE_MASK).unwrap(),
            Side::Black => CastlingSide::get_from_u8(self.0 & BLACK_MASK >> BLACK_SIDE_OFFSET).unwrap(),
        }
    }
    // If castling occures rules should be completely disabled for the side that castled
    pub fn disable_full_side(&mut self, side: Side) {
        const WHITE_MASK: u8 = 0b0000_11_00; // Since first two bits are:   0 all white rights(castling side) will be set for 0 (None)
        const BLACK_MASK: u8 = 0b0000_00_11; // Since third and fourth are: 0 all black rights(castling side) will be set for 0 (None)
        match side {
            Side::White => self.0 &= WHITE_MASK,
            Side::Black => self.0 &= BLACK_MASK,
        }
    }
    // In case one rook is moved ability to castle is lost only on one side
    pub fn disable_part_of_side(&mut self, side: Side, castling_side_to_disable: CastlingSide) {
        let mask: u8;
        match side {
            Side::White => mask = !(castling_side_to_disable as u8),
            Side::Black => mask = !((castling_side_to_disable as u8) << BLACK_SIDE_OFFSET),
        }
        self.0 &= mask;
    }
    pub fn disable_all(&mut self) {
        self.0 = 0;
    }
}