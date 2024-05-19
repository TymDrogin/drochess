use crate::gamestate::{
    Gamestate,
    board::Board,
};

use thiserror::Error;
use core::str;

pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

const FEN_NR_OF_PARTS: usize = 6;
const LIST_OF_PIECES: &str = "kqrbnpKQRBNP";
const WHITE_OR_BLACK: &str = "wb";
const SPLITTER: char = '/';
const DASH: char = '-';
const SPACE: char = ' ';

#[derive(Error, Debug)]
pub enum FenError {
    #[error("FEN stiring given has invalid lenght of `{0}`, expected 4-6")]
    InvalidLength(usize),

    #[error("Error in 1 part of FEN: Pieces or squares")]
    PieceLayout,
    #[error("Error in 2 part of FEN:  Colors")]
    StartingSide,
    #[error("Error in 3 part of FEN:  Castling rights")]
    CastlingRights,
    #[error("Error in 4 part of FEN:  En passant field")]
    EnPassant,
    #[error("Error in 5 part of FEN: : Half-move clock")]
    HalfMoveClock,
    #[error("Error in 6 part of FEN:  Full-move number")]
    FullMoveCounte,
} 
pub type FenResult = Result<Gamestate, FenError>;

// Fen struct is used to independently implement fen logic,
// Instead of having it be a part of Gamestate
pub struct Fen(pub String);
impl Fen {
    pub fn process(&self) -> FenResult {
        let mut gamestate: Gamestate;

        let separated_fen: Vec<&str> = self.0.split_whitespace().collect();
        if separated_fen.len() != FEN_NR_OF_PARTS {
            return Err(FenError::InvalidLength(separated_fen.len()));
        }

        
        let piece_layout = separated_fen[0];




        todo!();
    }
}
