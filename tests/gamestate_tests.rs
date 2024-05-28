#[cfg(test)]
mod tests {
    use rusty_chess_engine::gamestate::{board::*, defs::*};

    #[test]
    fn test_square_new_from_file_rank() {
        let square = Square::new_from_file_rank(3, 4).unwrap();

        assert_eq!(square.get_index(), 35); // 4 << 3 + 3 = 32 + 3 = 35

        let square = Square::new_from_file_rank(7, 7).unwrap();
        assert_eq!(square.get_index(), 63); // 7 << 3 + 7 = 56 + 7 = 63
    }

    #[test]
    fn test_square_new_invalid() {
        let square =  Square::new_from_file_rank(8, 8);
        assert_eq!(square, None);
    }

    #[test]
    fn test_square_new_from_algebraic_notation() {
        let square = Square::new_from_algebraic_notation("e4").unwrap();
        assert_eq!(square.get_index(), 28); // 3 << 3 + 4 = 24 + 4 = 28

        let square = Square::new_from_algebraic_notation("h8").unwrap();
        assert_eq!(square.get_index(), 63); // 7 << 3 + 7 = 56 + 7 = 63
    }

    #[test]
    fn test_square_get_file_rank() {
        let square = Square::new(35); // 35 = 4 << 3 + 3 -> (3, 4)
        let (file, rank) = square.get_file_rank();
        assert_eq!((file, rank), (3, 4));

        let square = Square::new(63); // 63 = 7 << 3 + 7 -> (7, 7)
        let (file, rank) = square.get_file_rank();
        assert_eq!((file, rank), (7, 7));
    }

    #[test]
    fn test_board_set_square() {
        let mut board = Board::new();

        let square = Square::new_from_file_rank(3, 4).unwrap();
        board.set_square(square, PieceType::Knight, Side::White);
        println!("{}", square.get_index());
        assert_eq!(
            board.white_pieces[PieceType::Knight as usize],
            1 << square.get_index() as Bitboard
        );

        let square = Square::new_from_file_rank(7, 7).unwrap();
        println!("{}", square.get_index());
        board.set_square(square, PieceType::Queen, Side::Black);
        assert_eq!(
            board.black_pieces[PieceType::Queen as usize],
            1 << square.get_index() as Bitboard
        );
    }

    #[test]
    fn test_castling_rights_new() {
        let rights = CastlingRights::new();
        assert_eq!(rights.get(), 0);
    }

    #[test]
    fn test_set_for_side() {
        let mut rights = CastlingRights::new();
        rights.set_for_side(Side::White, CastlingSide::Kingside);
        assert_eq!(rights.get(), CastlingSide::Kingside as u8);

        rights.set_for_side(Side::Black, CastlingSide::Queenside);
        assert_eq!(
            rights.get(),
            (CastlingSide::Kingside as u8) | ((CastlingSide::Queenside as u8) << BLACK_SIDE_OFFSET)
        );
    }

    #[test]
    fn test_disable_full_side() {
        let mut rights = CastlingRights::new();
        rights.set_for_side(Side::White, CastlingSide::Both);
        rights.set_for_side(Side::Black, CastlingSide::Both);
        rights.disable_full_side(Side::White);
        assert_eq!(
            rights.get(),
            (CastlingSide::Both as u8) << BLACK_SIDE_OFFSET
        );

        rights.disable_full_side(Side::Black);
        assert_eq!(rights.get(), 0);
    }

    #[test]
    fn test_disable_part_of_side() {
        let mut rights = CastlingRights::new();
        rights.set_for_side(Side::White, CastlingSide::Both);
        rights.set_for_side(Side::Black, CastlingSide::Both);

        rights.disable_part_of_side(Side::White, CastlingSide::Kingside);
        assert_eq!(
            rights.get(),
            (CastlingSide::Queenside as u8) | ((CastlingSide::Both as u8) << BLACK_SIDE_OFFSET)
        );

        rights.disable_part_of_side(Side::Black, CastlingSide::Queenside);
        assert_eq!(
            rights.get(),
            (CastlingSide::Queenside as u8) | (CastlingSide::Kingside as u8) << BLACK_SIDE_OFFSET
        );
    }
    #[test]
    fn test_disable_all() {
        let mut rights = CastlingRights::new();
        rights.set_for_side(Side::White, CastlingSide::Both);
        rights.set_for_side(Side::Black, CastlingSide::Both);
        rights.disable_all();
        assert_eq!(rights.get(), 0);
    }
}
