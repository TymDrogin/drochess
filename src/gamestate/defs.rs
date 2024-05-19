use crate::gamestate::board::Side;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastlingSide {
    None,
    Kingside,
    Queenside,
    Both
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

    // NOTE: Do not pass None to cs
    pub fn is_allowed(&self, side_to_mode:Side, cs:CastlingSide) -> bool {
        match side_to_mode {
            Side::White => match cs {
                CastlingSide::Kingside => matches!(self.white, CastlingSide::Kingside | CastlingSide::Both),
                CastlingSide::Queenside => matches!(self.white, CastlingSide::Queenside | CastlingSide::Both),
                CastlingSide::Both => self.white == CastlingSide::Both,
                CastlingSide::None => false, // Castling None does not make sense to be allowed
            },
            Side::Black => match cs {
                CastlingSide::Kingside => matches!(self.black, CastlingSide::Kingside | CastlingSide::Both),
                CastlingSide::Queenside => matches!(self.black, CastlingSide::Queenside | CastlingSide::Both),
                CastlingSide::Both => self.black == CastlingSide::Both,
                CastlingSide::None => false, // Castling None does not make sense to be allowed
            },
        }
    }
}