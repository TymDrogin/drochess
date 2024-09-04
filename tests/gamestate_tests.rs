#[cfg(test)]
mod tests {
    use rusty_chess_engine::gamestate::{board::*, castling_rights::*};

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
    fn test_get_squares_from_bitboard() {
        let bitboard: u64 = 0b10101010; // Example bitboard with bits 1, 3, 5, 7 set
        let squares = Square::get_squares_from_bitboard(bitboard);
        let expected_squares: Vec<Square> = vec![
            Square::new(1),
            Square::new(3),
            Square::new(5),
            Square::new(7),
        ];
        assert_eq!(squares, expected_squares);

        let bitboard: u64 = 0b1000000000000000000000000000000000000000000000000000000000000000; // Only the 63rd bit is set
        let squares = Square::get_squares_from_bitboard(bitboard);
        let expected_squares: Vec<Square> = vec![Square::new(63)];
        assert_eq!(squares, expected_squares);

        let bitboard: u64 = 0; // No bits set
        let squares = Square::get_squares_from_bitboard(bitboard);
        let expected_squares: Vec<Square> = vec![];
        assert_eq!(squares, expected_squares);
    }

    #[test]
    fn test_board_place_piece_at_square() {
        let mut board = Board::default();

        let square = Square::new_from_file_rank(3, 4).unwrap();
        board.place_piece_at_square(square, PieceType::Knight, Side::White);
        assert_eq!(
            board.white_pieces[PieceType::Knight as usize],
            square.get_mask()
        );

        let square = Square::new_from_file_rank(7, 7).unwrap();
        board.place_piece_at_square(square, PieceType::Queen, Side::Black);
        assert_eq!(
            board.black_pieces[PieceType::Queen as usize],
            square.get_mask()
        );
    }

    #[test]
    fn test_board_remove_piece_at_square() {
        let mut board = Board::default();

        // Place a white knight on the board at (3, 4)
        let square_knight = Square::new_from_file_rank(3, 4).unwrap();
        board.place_piece_at_square(square_knight, PieceType::Knight, Side::White);
        assert_eq!(
            board.white_pieces[PieceType::Knight as usize],
            square_knight.get_mask() as Bitboard
        );

        // Place a black queen on the board at (7, 7)
        let square_queen = Square::new_from_file_rank(7, 7).unwrap();
        board.place_piece_at_square(square_queen, PieceType::Queen, Side::Black);
        assert_eq!(
            board.black_pieces[PieceType::Queen as usize],
            square_queen.get_mask() as Bitboard
        );

        // Remove the white knight on the board at (3, 4)
        board.remove_piece_at_square(square_knight, PieceType::Knight, Side::White);
        assert_eq!(
            board.white_pieces[PieceType::Knight as usize],
            0 as Bitboard
        );

        // Remove the black queen on the board at (3, 4)
        board.remove_piece_at_square(square_queen, PieceType::Queen, Side::Black);
        assert_eq!(
            board.black_pieces[PieceType::Queen as usize],
            0 as Bitboard
        );
    }

    #[test]
    fn test_board_clear_square() {
        let mut board = Board::default();

        // Place a white knight on the board at (3, 4)
        let square_knight = Square::new_from_file_rank(3, 4).unwrap();
        board.place_piece_at_square(square_knight, PieceType::Knight, Side::White);
        assert_eq!(
            board.white_pieces[PieceType::Knight as usize],
            square_knight.get_mask()
        );

        // Place a black queen on the board at (7, 7)
        let square_queen = Square::new_from_file_rank(7, 7).unwrap();
        board.place_piece_at_square(square_queen, PieceType::Queen, Side::Black);
        assert_eq!(
            board.black_pieces[PieceType::Queen as usize],
            square_queen.get_mask()
        );

        // Clear the white knight
        board.clear_square(square_knight);
        assert_eq!(
            board.white_pieces[PieceType::Knight as usize],
            0
        );

        // Clear the black queen
        board.clear_square(square_queen);
        assert_eq!(
            board.black_pieces[PieceType::Queen as usize],
            0
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
