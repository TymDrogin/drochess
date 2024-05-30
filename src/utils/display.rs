use crate::gamestate::{
    board::*,
    castling_rights::{
        CastlingRights,
        CastlingSide,
    },
    Gamestate
};
use std::fmt::{self, Display};

const PRINT_METADATA: bool = true;

pub const WHITE_PAWN: char = '♙';
pub const WHITE_KNIGHT: char = '♘';
pub const WHITE_BISHOP: char = '♗';
pub const WHITE_ROOK: char = '♖';
pub const WHITE_QUEEN: char = '♕';
pub const WHITE_KING: char = '♔';
pub const BLACK_PAWN: char = '♟';
pub const BLACK_KNIGHT: char = '♞';
pub const BLACK_BISHOP: char = '♝';
pub const BLACK_ROOK: char = '♜';
pub const BLACK_QUEEN: char = '♛';
pub const BLACK_KING: char = '♚';

pub const EMPTY: char = '.';

impl Display for Gamestate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print board
        writeln!(f, "a b c d e f g h")?;
        for rank in (0..BOARD_SIDE_LENGTH).rev() {
            for file in 0..BOARD_SIDE_LENGTH {
                let square: Square;

                // Depending on the current player to move it will "turn" the board
                // using reversed indexing
                match self.side_to_move {
                    Side::White => {square = Square::new_from_file_rank(7 - file, 7 - rank).unwrap()},
                    Side::Black => {square = Square::new_from_file_rank(file, rank).unwrap()},
                }

                let piece_char = match self.board.get_piece_at_square(square) {
                    None => EMPTY,
                    Some((piece_type, side)) => match (piece_type, side) {
                        (PieceType::Pawn, Side::White) => WHITE_PAWN,
                        (PieceType::Knight, Side::White) => WHITE_KNIGHT,
                        (PieceType::Bishop, Side::White) => WHITE_BISHOP,
                        (PieceType::Rook, Side::White) => WHITE_ROOK,
                        (PieceType::Queen, Side::White) => WHITE_QUEEN,
                        (PieceType::King, Side::White) => WHITE_KING,

                        (PieceType::Pawn, Side::Black) => BLACK_PAWN,
                        (PieceType::Knight, Side::Black) => BLACK_KNIGHT,
                        (PieceType::Bishop, Side::Black) => BLACK_BISHOP,
                        (PieceType::Rook, Side::Black) => BLACK_ROOK,
                        (PieceType::Queen, Side::Black) => BLACK_QUEEN,
                        (PieceType::King, Side::Black) => BLACK_KING,
                    },
                };
                write!(f, "{} ", piece_char)?;
            }
            writeln!(f, "{}", rank + 1)?;
        }
        if PRINT_METADATA {
            let white_rights: CastlingSide = self.castling_rights.get_for_side(Side::White);
            let black_rights: CastlingSide = self.castling_rights.get_for_side(Side::Black);
        
            writeln!(f)?;
            
            // Print White castling rights
            write!(f, "White castling rights: ")?;
            match white_rights {
                CastlingSide::None => writeln!(f, "None")?,
                CastlingSide::Kingside => writeln!(f, "Kingside")?,
                CastlingSide::Queenside => writeln!(f, "Queenside")?,
                CastlingSide::Both => writeln!(f, "Kingside and Queenside")?,
            
            }

            // Print Black castling rights
            write!(f, "Black castling rights: ")?;
            match black_rights {
                CastlingSide::None => writeln!(f, "None")?,
                CastlingSide::Kingside => writeln!(f, "Kingside")?,
                CastlingSide::Queenside => writeln!(f, "Queenside")?,
                CastlingSide::Both => writeln!(f, "Kingside and Queenside")?,
            }

            // Print clocks
            write!(f, "Half move clock: {0}", self.half_move_clock)?;
            writeln!(f)?;
            write!(f, "Full move counter: {0}", self.full_move_count)?;
            writeln!(f)?;
        }
        Ok(())
    }
}
