use crate::gamestate::{
    board::*,
    castling_rights::{CastlingRights, CastlingSide},
    constants::*,
    Gamestate,
    zobrist::*
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
const WHITE_KING: char = 'K';
const WHITE_QUEEN: char = 'Q';
const WHITE_ROOK: char = 'R';
const WHITE_BISHOP: char = 'B';
const WHITE_KNIGHT: char = 'N';
const WHITE_PAWN: char = 'P';

const BLACK_KING: char = 'k';
const BLACK_QUEEN: char = 'q';
const BLACK_ROOK: char = 'r';
const BLACK_BISHOP: char = 'b';
const BLACK_KNIGHT: char = 'n';
const BLACK_PAWN: char = 'p';

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

    #[error("Error in 1 part of FEN: `{0}` ")]
    PieceLayout(String),
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
// instead of having it be a part of Gamestate
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

        let mut game = Gamestate::new(
            board,
            side_to_move,
            castling_rights,
            en_passant,
            half_move_clock,
            full_move_count,
            0,
        );
        game.zobrist_key = Zobrist::hash(&game);
        Ok(game)
    }

    fn get_board(s: &str) -> Result<Board, FenError> {
        let fen_ranks: Vec<&str> = s.split(SPLITTER).collect();
        if fen_ranks.len() != 8 {
            return Err(FenError::PieceLayout(format!(
                "The number of board ranks is not equal to 8, ranks number = `{}`",
                fen_ranks.len()
            )));
        }

        let mut board = Board::default();
        for (rank_index, rank) in fen_ranks.iter().enumerate() {
            if rank_index >= BOARD_SIDE_LENGTH as usize {
                return Err(FenError::PieceLayout(format!(
                    "Rank index value is more than 8, rank index = `{}`",
                    rank_index
                )));
            }

            let mut file_index: u8 = 0;
            for piece in rank.chars() {
                if piece.is_digit(10) {
                    let empty_squares = piece.to_digit(10).unwrap() as u8;
                    if !(1..=8).contains(&empty_squares) {
                        return Err(FenError::PieceLayout(format!(
                            "Invalid number of empty squares: {}, at rank {}, file index {}",
                            empty_squares,
                            rank_index + 1,
                            file_index + 1
                        )));
                    }

                    file_index += empty_squares;
                    continue;
                }
                if file_index >= BOARD_SIDE_LENGTH as u8 {
                    return Err(FenError::PieceLayout(format!(
                        "File index value is more than 8, file index = `{}`",
                        file_index
                    )));
                }

                let square = Square::new_from_file_rank(file_index, 7 - rank_index as u8);

                match piece {
                    WHITE_KING => board.add_piece(square, PieceType::King, Side::White),
                    WHITE_QUEEN => {
                        board.add_piece(square, PieceType::Queen, Side::White)
                    }
                    WHITE_ROOK => board.add_piece(square, PieceType::Rook, Side::White),
                    WHITE_BISHOP => {
                        board.add_piece(square, PieceType::Bishop, Side::White)
                    }
                    WHITE_KNIGHT => {
                        board.add_piece(square, PieceType::Knight, Side::White)
                    }
                    WHITE_PAWN => board.add_piece(square, PieceType::Pawn, Side::White),

                    BLACK_KING => board.add_piece(square, PieceType::King, Side::Black),
                    BLACK_QUEEN => {
                        board.add_piece(square, PieceType::Queen, Side::Black)
                    }
                    BLACK_ROOK => board.add_piece(square, PieceType::Rook, Side::Black),
                    BLACK_BISHOP => {
                        board.add_piece(square, PieceType::Bishop, Side::Black)
                    }
                    BLACK_KNIGHT => {
                        board.add_piece(square, PieceType::Knight, Side::Black)
                    }
                    BLACK_PAWN => board.add_piece(square, PieceType::Pawn, Side::Black),

                    _ => {
                        return Err(FenError::PieceLayout(format!(
                            "Invalid symbol '{}' encountered",
                            piece
                        )))
                    }
                }
                // Increment index
                file_index += 1;
            }

            if file_index != BOARD_SIDE_LENGTH as u8 {
                return Err(FenError::PieceLayout(format!(
                    "By the end of the rank, file does not have exactly 8 squares, but {}",
                    file_index
                )));
            }
        }

        Ok(board)
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
    fn get_castling_rights(s: &str) -> Result<CastlingRights, FenError> {
        if s.len() != 4 {
            return Err(FenError::CastlingRights);
        }

        let mut cr = CastlingRights::new();
        for ch in s.chars() {
            match ch {
                WHITE_KINGSIDE => cr.set_rights(Side::White, CastlingSide::Kingside),
                WHITE_QUEENSIDE => cr.set_rights(Side::White, CastlingSide::Queenside),
                BLACK_KINGSIDE => cr.set_rights(Side::Black, CastlingSide::Kingside),
                BLACK_QUEENSIDE => cr.set_rights(Side::Black, CastlingSide::Queenside),
                _ => return Err(FenError::CastlingRights),
            }
        }
        Ok(cr)
    }
    fn get_en_passant(s: &str) -> Result<u8, FenError> {
        if s == "-" {
            return Ok(0 as u8);
        }
        if s.len() != 2 {
            return Err(FenError::EnPassant);
        }
        match Square::new_from_algebraic_notation(s) {
            Some(sq) => {
                let (_, rank) = sq.get_file_rank();
                let en_passant = 1 << rank;
                Ok(en_passant)
            }
            None => Err(FenError::EnPassant),
        }
    }
    fn get_half_move_clock(s: &str) -> Result<u8, FenError> {
        match s.parse::<u8>() {
            Ok(x) => Ok(x),
            Err(_) => Err(FenError::HalfMoveClock),
        }
    }
    fn get_full_move_count(s: &str) -> Result<u8, FenError> {
        match s.parse::<u8>() {
            Ok(x) => Ok(x),
            Err(_) => Err(FenError::FullMoveCounte),
        }
    }
}
