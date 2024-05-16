#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}
// Instead of stantrart bitflags i decided to go with more ideomatic
// way of encapsulating states using enums. The good thing is in the future
// use of enums will pay of by forcing me or anybody else to cover all cases.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastlingSide {
    None,
    Kingside,
    Queenside,
    Both
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights {
    white: CastlingSide,
    black: CastlingSide,
}
impl CastlingRights {
    pub fn new() -> Self {
        Self {
            white: CastlingSide::Both,
            black: CastlingSide::Both,
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