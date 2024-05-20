use crate::{gamestate::board::Side, FenError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastlingSide {
    None,
    Kingside,
    Queenside,
    Both
}
impl CastlingSide {
    pub fn set(&mut self, castling_mask) {

    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights {
    pub white: CastlingSide,
    pub black: CastlingSide,
}

impl CastlingRights {
    pub fn new(white_sides: CastlingSide, black_sides: CastlingSide) -> Self {
        Self {
            white: white_sides,
            black: black_sides,
        }
    }
    pub fn disable(&mut self, side: Side) {
        match side {
            Side::White => self.white = CastlingSide::None,
            Side::Black => self.black = CastlingSide::None,
        }
    }

    pub fn set_for_side(&mut self, side: Side, rights: CastlingSide) -> Result<(), FenError> {
        match side {
            Side::White => self.white = (self.white as u8 | rights as u8),
            Side::Black => self.black = (self.black as u8 | rights as u8),
        }
        Ok(())
    }
    
}