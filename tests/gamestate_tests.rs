#[cfg(test)]
mod tests {
    const BLACK_SIDE_OFFSET: u8 = 2;
    mod square_tests {
        use rusty_chess_engine::gamestate::{board::*, constants::*};

        #[test]
        fn test_new() {
            let sq = Square::new(0);
            assert_eq!(sq.get_index(), 0);

            let sq = Square::new(63);
            assert_eq!(sq.get_index(), 63);
        }

        #[test]
        #[should_panic(expected = "Attempted to create square with index more than max of 63")]
        fn test_new_panic() {
            Square::new(64);
        }

        #[test]
        fn test_new_from_file_rank() {
            assert_eq!(Square::new_from_file_rank(0, 0).unwrap().get_index(), 0);
            assert_eq!(Square::new_from_file_rank(7, 7).unwrap().get_index(), 63);
            assert!(Square::new_from_file_rank(8, 0).is_none());
            assert!(Square::new_from_file_rank(0, 8).is_none());
        }

        #[test]
        fn test_new_from_algebraic_notation() {
            assert_eq!(Square::new_from_algebraic_notation("a1").unwrap().get_index(), 0);
            assert_eq!(Square::new_from_algebraic_notation("h8").unwrap().get_index(), 63);
            assert_eq!(Square::new_from_algebraic_notation("e4").unwrap().to_algebraic_notation(), "e4");
            assert!(Square::new_from_algebraic_notation("i9").is_none());
            assert!(Square::new_from_algebraic_notation("a0").is_none());
        }

        #[test]
        fn test_get_file_rank() {
            let sq = Square::new_from_file_rank(3, 5).unwrap();
            let (file, rank) = sq.get_file_rank();
            assert_eq!(file, 3);
            assert_eq!(rank, 5);
        }

        #[test]
        fn test_to_algebraic_notation() {
            let sq = Square::new_from_file_rank(0, 0).unwrap();
            assert_eq!(sq.to_algebraic_notation(), "a1");
            let sq = Square::new_from_file_rank(7, 7).unwrap();
            assert_eq!(sq.to_algebraic_notation(), "h8");
        }

        #[test]
        fn test_get_mask() {
            let sq = Square::new(0);
            assert_eq!(sq.get_mask(), 1u64);

            let sq = Square::new(1);
            assert_eq!(sq.get_mask(), 1u64 << 1);

            let sq = Square::new(63);
            assert_eq!(sq.get_mask(), 1u64 << 63);
        }

        #[test]
        fn test_get_squares_from_bitboard() {
            let bitboard = (1u64 << 0) | (1u64 << 5) | (1u64 << 63);
            let squares = Square::get_squares_from_bitboard(bitboard);
            let indices: Vec<u8> = squares.iter().map(|s| s.get_index() as u8).collect();
            assert_eq!(indices, vec![0, 5, 63]);
        }
    }


    mod board_tests {
        use rusty_chess_engine::gamestate::{board::*, constants::*};
        
        #[test]
        fn test_place_piece_and_get_bitboard() {
            let mut board = Board::default();
            let sq = Square::new_from_algebraic_notation("e4").unwrap();

            board.add_piece(sq, PieceType::Pawn, Side::White);

            let bitboard = board.get_bitboard_of(PieceType::Pawn, Side::White);
            assert_eq!(bitboard, sq.get_mask());

            assert_eq!(board.occupancy[Side::White as usize], sq.get_mask());
            assert!(board.is_square_occupied(sq));
        }

        #[test]
        fn test_get_squares_of() {
            let mut board = Board::default();
            let sq1 = Square::new_from_algebraic_notation("a2").unwrap();
            let sq2 = Square::new_from_algebraic_notation("b2").unwrap();

            board.add_piece(sq1, PieceType::Pawn, Side::White);
            board.add_piece(sq2, PieceType::Pawn, Side::White);

            let squares = board.get_squares_of(PieceType::Pawn, Side::White);
            let indices: Vec<usize> = squares.iter().map(|s| s.get_index()).collect();

            assert!(indices.contains(&sq1.get_index()));
            assert!(indices.contains(&sq2.get_index()));
            assert_eq!(indices.len(), 2);
        }

        #[test]
        fn test_get_piece_at_square() {
            let mut board = Board::default();
            let sq = Square::new_from_algebraic_notation("d5").unwrap();

            assert_eq!(board.get_piece_at_square(sq), None);

            board.add_piece(sq, PieceType::Knight, Side::Black);

            let piece = board.get_piece_at_square(sq);
            assert!(piece.is_some());

            let (pt, side) = piece.unwrap();
            assert_eq!(pt, PieceType::Knight);
            assert_eq!(side, Side::Black);
        }

        #[test]
        fn test_is_square_occupied() {
            let mut board = Board::default();
            let sq_empty = Square::new_from_algebraic_notation("a1").unwrap();
            let sq_occupied = Square::new_from_algebraic_notation("h8").unwrap();

            assert!(!board.is_square_occupied(sq_empty));

            board.add_piece(sq_occupied, PieceType::King, Side::White);
            assert!(board.is_square_occupied(sq_occupied));
        }

        #[test]
        fn test_piece_index_consistency() {
            for side in &[Side::White, Side::Black] {
                for pt_u8 in 0..PIECE_TYPES_NUM {
                    let pt = PieceType::from_u8(pt_u8 as u8);
                    let index = Board::piece_index(pt, *side);

                    if *side == Side::White {
                        assert!(index < PIECE_TYPES_NUM);
                    } else {
                        assert!(index >= PIECE_TYPES_NUM);
                    }
                }
            }
        }
    #[test]
    fn test_move_piece() {
        let mut board = Board::default();
        let from = Square::new_from_algebraic_notation("e2").unwrap();
        let to   = Square::new_from_algebraic_notation("e4").unwrap();

        // Place a white pawn on e2
        board.add_piece(from, PieceType::Pawn, Side::White);
        assert!(board.is_square_occupied(from));
        assert!(!board.is_square_occupied(to));

        // Move it from e2 to e4
        board.move_piece(from, to, PieceType::Pawn, Side::White);

        // After the move:
        // - e2 should be empty
        assert!(!board.is_square_occupied(from));
        assert_eq!(board.get_piece_at_square(from), None);

        // - e4 should have the pawn
        assert!(board.is_square_occupied(to));
        let piece_on_to = board.get_piece_at_square(to).unwrap();
        assert_eq!(piece_on_to, (PieceType::Pawn, Side::White));

        // - bitboards must reflect the move:
        let pawn_bb = board.get_bitboard_of(PieceType::Pawn, Side::White);
        assert_eq!(pawn_bb, to.get_mask());

        // - occupancy for White should match the pawn mask on e4
        assert_eq!(board.occupancy[Side::White as usize], to.get_mask());
    }






    }
}

    
