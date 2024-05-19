use crate::gamestate::{
    board::Board,
    defs::{CastlingRights, Side},
    Gamestate,
};
use core::str;
use thiserror::Error;

pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// This constants used to parse fen string
const FEN_NR_OF_PARTS: usize = 6;
const SPLITTER: char = '/';
const DASH: char = '-';
const SPACE: char = ' ';

// Constants for piece symbols
pub const WHITE_KING: char = 'K';
pub const WHITE_QUEEN: char = 'Q';
pub const WHITE_ROOK: char = 'R';
pub const WHITE_BISHOP: char = 'B';
pub const WHITE_KNIGHT: char = 'N';
pub const WHITE_PAWN: char = 'P';

pub const BLACK_KING: char = 'k';
pub const BLACK_QUEEN: char = 'q';
pub const BLACK_ROOK: char = 'r';
pub const BLACK_BISHOP: char = 'b';
pub const BLACK_KNIGHT: char = 'n';
pub const BLACK_PAWN: char = 'p';

// Side symbols
const WHITE_SIDE: char = 'w';
const BLACK_SIDE: char = 'b';

// Castling sides symbols 
const WHITE_KINGSIDE: char = 'K';
const WHITE_QUEENSIDE: char = 'Q';
const BLACK_KINGSIDE: char = 'k';
const BLACK_QUEENSIDE: char = 'q';

// This constants are used only for indexing thru the parts of fen
const BOARD_LAYOUT: usize = 0;
const STARTING_SIDE: usize = 1;
const CASTLING_RIGHTS: usize = 2;
const EN_PASSANT: usize = 3;
const HALF_MOVE_CLOCK: usize = 4;
const FULL_MOVE_COUNTER: usize = 5;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum FenError {
    #[error("FEN stiring given has invalid number of elements of `{0}`, expected 6")]
    InvalidNumOfElements(usize),

    #[error("Error in 1 part of FEN: Invalid board layout, colors or symbols")]
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
        let separated_fen: Vec<&str> = self.0.split(SPACE).collect();
        if separated_fen.len() != FEN_NR_OF_PARTS {
            return Err(FenError::InvalidNumOfElements(separated_fen.len()));
        }

        let board = Self::get_board(separated_fen[BOARD_LAYOUT])?;
        let side_to_move = Self::get_side_to_move(separated_fen[STARTING_SIDE])?;
        let castling_rights = Self::get_castling_rights(separated_fen[CASTLING_RIGHTS])?;
        let en_passant = Self::get_en_passant(separated_fen[EN_PASSANT])?;
        let half_move_clock = Self::get_half_move_clock(separated_fen[HALF_MOVE_CLOCK])?;
        let full_move_count = Self::get_full_move_count(separated_fen[FULL_MOVE_COUNTER])?;

        Ok(Gamestate {
            board,
            side_to_move,
            castling_rights,
            en_passant,
            half_move_clock,
            full_move_count,
        })
    }

    fn get_board(s: &str) -> Result<Board, FenError> {
        let mut board: Board;

        let piece_layout: Vec<&str> = s.split(SPLITTER).collect();
        todo!()
    }
    fn get_side_to_move(s: &str) -> Result<Side, FenError> {
        if s.len() != 1 {
            return Err(FenError::StartingSide);
        }
    
        match s.chars().next() {
            Some(WHITE_SIDE) => Ok(Side::White),
            Some(BLACK_SIDE) => Ok(Side::Black),
            _ => Err(FenError::StartingSide),
        }
    }
    // That is a lot of code as a consequence of me refusing to use flag bits and using ideomatic enums
    fn get_castling_rights(s: &str) -> Result<CastlingRights, FenError> {
        let castling_rights = CastlingRights;
        for c in s.chars() {
            let mut castling_rights = CastlingRights;
        
            for c in s.chars() {
                match c {
                    WHITE_KINGSIDE => match castling_rights.white {
                        CastlingSide::None => castling_rights.white = CastlingSide::Kingside,
                        CastlingSide::Queenside => castling_rights.white = CastlingSide::Both,
                        _ => Err(CastlingRights)
                    },
                    WHITE_QUEENSIDE => match castling_rights.white {
                        CastlingSide::None => castling_rights.white = CastlingSide::Queenside,
                        CastlingSide::Kingside => castling_rights.white = CastlingSide::Both,
                        _ => Err(CastlingRights)
                    },
                    BLACK_KINGSIDE => match castling_rights.black {
                        CastlingSide::None => castling_rights.black = CastlingSide::Kingside,
                        CastlingSide::Queenside => castling_rights.black = CastlingSide::Both,
                        _ => Err(CastlingRights)
                    },
                    BLACK_QUEENSIDE => match castling_rights.black {
                        CastlingSide::None => castling_rights.black = CastlingSide::Queenside,
                        CastlingSide::Kingside => castling_rights.black = CastlingSide::Both,
                        _ => Err(CastlingRights)
                    },
                    _ => return Err(FenError::InvalidCharacter(c)),
                }
            }
            Ok(castling_rights)
        }
    }

    fn get_en_passant(s: &str) -> Result<Option<u8>, FenError> {
        todo!()
    }
    fn get_half_move_clock(s: &str) -> Result<usize, FenError> {
        match s.parse::<usize>() {
            Ok(x) => Ok(x),
            Err(_) => Err(FenError::HalfMoveClock),
        }
    }
    fn get_full_move_count(s: &str) -> Result<usize, FenError> {
        match s.parse::<usize>() {
            Ok(x) => Ok(x),
            Err(_) => Err(FenError::FullMoveCounte),
        }
    }
}
