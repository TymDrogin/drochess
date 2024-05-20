use crate::{gamestate::board::Side, FenError};

const BLACK_SIDE_OFFSET: u8 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastlingSide {
    None      = 0, //00000000
    Kingside  = 1, //00000001
    Queenside = 2, //00000010
    Both      = 3  //00000011
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

// structure is as follows: 0000(garbage bits), 00(black bits), 00(white bits) -> 0000****  
pub struct CastlingRights(u8);
impl CastlingRights {
    pub fn new() -> Self {
        todo!()
    }
    pub fn set_for_side(&mut self, side: Side, rights: CastlingSide) -> Result<(), FenError> {
        match side {
            Side::White => self.0 |= rights as u8,
            Side::Black => self.0 |= (rights as u8) << BLACK_SIDE_OFFSET,
        }
        Ok(())
    }  
    // If castling occures rules should be completely disabled for the side that castled  
    pub fn disable_full_side(&mut self, side: Side) {
        match side {
            Side::White => {
                const MASK:u8 = !(CastlingSide::Both as u8);
                self.0 &= MASK;
            }
            Side::Black => {
                const MASK:u8 = !((CastlingSide::Both as u8) << BLACK_SIDE_OFFSET);
                self.0 &= MASK;
            }
        }
    }
    // In case one rook is moved ability to castle is lost only on one side
    pub fn disable_part_of_side(&mut self, side: Side, castling_side_to_disable: CastlingSide) {
        match side {
            Side::White => {
                let mask = !(castling_side_to_disable as u8); 
                self.0 &= mask;
            }
            Side::Black => {
                let mask = !((castling_side_to_disable as u8) << BLACK_SIDE_OFFSET); 
                self.0 &= mask;
            }
        }

    }
    pub fn disable_all(&mut self) {
        self.0 = 0;
    }
}