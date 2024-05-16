use gamestate::Gamestate:: {
    Gamestate,
    Board,
};

use core::str;

pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

const FEN_NR_OF_PARTS: usize = 6;
const LIST_OF_PIECES: &str = "kqrbnpKQRBNP";
const WHITE_OR_BLACK: &str = "wb";
const SPLITTER: char = '/';
const DASH: char = '-';
const EM_DASH: char = '–';
const SPACE: char = ' ';



#[derive(Error, Debug)]
pub enum FenError {
    #[error("FEN stiring given has invalid lenght of `{0}`, expected 4-6")]
    InvalidLength(usize),

    #[error("Error in 1 part of FEN: Pieces or squares")]
    Part1,
    #[error("Error in 2 part of FEN:  Colors")]
    Part2,
    #[error("Error in 3 part of FEN:  Castling rights")]
    Part3,
    #[error("Error in 4 part of FEN:  En passant field")]
    Part4,
    #[error("Error in 5 part of FEN: : Half-move clock")]
    Part5,
    #[error("Error in 6 part of FEN:  Full-move number")]
    Part6,
} 
pub type FenResult = Result<Gamestate, FenError>;

// Fen struct is used to independently implement fen logic,
// Instead of having it be a part of Gamestate
pub struct Fen(&str);
impl Fen {
    pub fn process(&self) -> FenResult {
        let separated_fen: Vec<&str> = self.0.split_whitespaces().collect();
        if(separated_fen.len() != FEN_NR_OF_PARTS) {
            FenError::InvalidLength(separated_fen.len());
        }
        let mut board: Board = {0};
    }


}
