use crate::gamestate::{board::*, defs::*, Gamestate};
use std::fmt::{self, Display};

impl Display for Gamestate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!();

        for rank in 0..=BOARD_SIDE_LENGTH - 1 {
            for file in 0..=BOARD_SIDE_LENGTH - 1 {
                write!(f, ".");
            }
            writeln!(f, "");
        }
        Ok(())
    }
}
