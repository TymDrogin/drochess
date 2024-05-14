mod defs;
mod fen;
//use defs::;
use board;

#[derive(Debug, Clone, PartialEqHow)]
pub struct Gamestate {
    board: Board,
    side_to_move: Side,    
    en_passant: Option<u8>,
    half_move_clock: u32,
    full_move_count: u32,
}

impl Gamestate {
    pub fn new() -> Self {
        board = Board;
        side_to_move = Side::None;
        en_passant = None;

    
    }
}


pub fn get_piece_squares(&self, piece_type: PieceType, side: Side) -> Bitboard {
    match side {
        Side::White => self.white_pieces[piece_type as usize],
        Side::Black => self.black_pieces[piece_type as usize],
        _ => panic!("Can't get piece bitmap -- no side selected")
    }
}