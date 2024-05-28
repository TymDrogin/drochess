use crate::gamestate::{board::*, Gamestate};
use std::fmt::{self, write, Display};

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
        writeln!(f, "a b c d e f g h")?;
        for rank in (0..BOARD_SIDE_LENGTH).rev() {
            for file in 0..BOARD_SIDE_LENGTH {
                let square: Square;

                // Depending on the current player to move it will "turn" the board to the player
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
        Ok(())
    }
}
