#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_new() {
        let square = Square::new(3, 4);

        assert_eq!(square.get_index(), 35); // 4 << 3 + 3 = 32 + 3 = 35

        let square = Square::new(7, 7);
        assert_eq!(square.get_index(), 63); // 7 << 3 + 7 = 56 + 7 = 63
    }

    #[test]
    #[should_panic(expected = "Attempted to create square with file: 8 or rank: 8 vith values more then 7")]
    fn test_square_new_invalid() {
        Square::new(8, 8);
    }

    #[test]
    fn test_square_new_from_algebraic_notation() {
        let square = Square::new_from_algebraic_notation("e4");
        assert_eq!(square.get_index(), 28); // 3 << 3 + 4 = 24 + 4 = 28

        let square = Square::new_from_algebraic_notation("h8");
        assert_eq!(square.get_index(), 63); // 7 << 3 + 7 = 56 + 7 = 63
    }

    #[test]
    fn test_square_get_file_rank() {
        let square = Square(35); // 35 = 4 << 3 + 3 -> (3, 4)
        let (file, rank) = square.get_file_rank();
        assert_eq!((file, rank), (3, 4));

        let square = Square(63); // 63 = 7 << 3 + 7 -> (7, 7)
        let (file, rank) = square.get_file_rank();
        assert_eq!((file, rank), (7, 7));
    }

    #[test]
    fn test_board_set_square() {
        let mut board = Board::new();

        let square = Square::new(3, 4);
        board.set_square(square, PieceType::Knight, Side::White);
        println!("{}", square.get_index());
        assert_eq!(board.white_pieces[PieceType::Knight as usize], 1 << square.get_index() as Bitboard);

        let square = Square::new(7, 7);
        println!("{}", square.get_index());
        board.set_square(square, PieceType::Queen, Side::Black);
        assert_eq!(board.black_pieces[PieceType::Queen as usize], 1 << square.get_index() as Bitboard);
    }
}
