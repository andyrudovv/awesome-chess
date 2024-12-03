

#[cfg(test)]
mod tests {
    use engine::bitboard::Bitboard;
    use engine::move_gen::*;

    // Helper function to create a Bitboard from a 64-bit integer for testing
    fn bitboard_from_u64(val: u64) -> Bitboard {
        Bitboard(val)
    }

    // Helper function to generate expected moves for pawns (only checking the forward pushes)
    fn generate_expected_pawn_moves(pawn_pos: u64, empty_pos: u64) -> u64 {
        let pawns = bitboard_from_u64(pawn_pos);
        let empty = bitboard_from_u64(empty_pos);
        generate_white_pawn_moves(pawns, empty).0
    }

    // Test for white pawn moves
    #[test]
    fn test_generate_white_pawn_moves() {
        // Test white pawn's single and double forward push from initial positions
        assert_eq!(generate_expected_pawn_moves(0x000000000000FF00, 0xFFFFFFFFFFFFFFFF), 0x0000000000FF0000);
        assert_eq!(generate_expected_pawn_moves(0x0000000000FF0000, 0xFFFFFFFFFFFFFFFF), 0x0000000000000000);
    }

    // Test for knight moves (generating possible jumps for a knight)
    #[test]
    fn test_generate_knight_moves() {
        // Place knight on E4 (position: 0x0000001000000000) and generate possible knight moves
        let knight_pos = bitboard_from_u64(0x0000001000000000);
        let moves = generate_knight_moves(knight_pos);
        assert_eq!(moves.0, 0x0000000022000000); // Example of expected result for this position
    }

    // Test for rook moves (generating possible sliding moves from an initial position)
    #[test]
    fn test_generate_rook_moves() {
        // Place rook on E4 (0x0000001000000000) and generate possible rook moves
        let rook_pos = bitboard_from_u64(0x0000001000000000);
        let occupied = bitboard_from_u64(0x000000000000FF00); // Simulate some occupied squares (blocking)
        let moves = generate_rook_moves(rook_pos, occupied);
        assert_eq!(moves.0, 0x0000000800000000); // Example expected result for rook moves
    }

    // Test for bishop moves (generating diagonal moves)
    #[test]
    fn test_generate_bishop_moves() {
        // Place bishop on E4 (0x0000001000000000) and generate possible bishop moves
        let bishop_pos = bitboard_from_u64(0x0000001000000000);
        let occupied = bitboard_from_u64(0x000000000000FF00); // Simulate some occupied squares
        let moves = generate_bishop_moves(bishop_pos, occupied);
        assert_eq!(moves.0, 0x0000000200000000); // Example expected result for bishop moves
    }

    // Test for queen moves (combining rook and bishop moves)
    #[test]
    fn test_generate_queen_moves() {
        // Place queen on E4 (0x0000001000000000) and generate possible queen moves
        let queen_pos = bitboard_from_u64(0x0000001000000000);
        let occupied = bitboard_from_u64(0x000000000000FF00); // Simulate some occupied squares
        let moves = generate_queen_moves(queen_pos, occupied);
        assert_eq!(moves.0, 0x0000000A00000000); // Example expected result for queen moves
    }

    // Test for black pawn moves
    #[test]
    fn test_generate_black_pawn_moves() {
        // Test black pawn single and double forward push from initial positions
        assert_eq!(generate_expected_pawn_moves(0x00000000000000FF, 0xFFFFFFFFFFFFFFFF), 0x00000000000000FF);
    }

    // Test for king moves
    #[test]
    fn test_generate_king_moves() {
        // Place king on E4 (0x0000001000000000) and generate possible king moves
        let king_pos = bitboard_from_u64(0x0000001000000000);
        let moves = generate_king_moves(king_pos);
        assert_eq!(moves.0, 0x0000001C00000000); // Example expected result for king moves
    }

    // Test for combined white moves
    #[test]
    fn test_generate_white_moves() {
        let pawns = bitboard_from_u64(0x000000000000FF00);
        let knights = bitboard_from_u64(0x0000001000000000);
        let rooks = bitboard_from_u64(0x0000000800000000);
        let occupied = bitboard_from_u64(0x000000000000FF00);
        let empty = bitboard_from_u64(0xFFFFFFFFFFFFFFFF);
        let moves = generate_white_moves(pawns, knights, rooks, occupied, empty);
        assert_eq!(moves.0, 0x0000001E00000000); // Example combined result for all white pieces
    }

    // Test for combined black moves
    #[test]
    fn test_generate_black_moves() {
        let pawns = bitboard_from_u64(0x00000000000000FF);
        let knights = bitboard_from_u64(0x0000001000000000);
        let rooks = bitboard_from_u64(0x0000000800000000);
        let occupied = bitboard_from_u64(0x000000000000FF00);
        let empty = bitboard_from_u64(0xFFFFFFFFFFFFFFFF);
        let moves = generate_black_moves(pawns, knights, rooks, occupied, empty);
        assert_eq!(moves.0, 0x0000001E00000000); // Example combined result for all black pieces
    }
}
