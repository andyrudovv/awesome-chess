use crate::bitboard::Bitboard;
use crate::board::ChessBoard;

/// Assign material values to pieces.
pub fn material_value(pawns: Bitboard, knights: Bitboard, rooks: Bitboard) -> i32 {
    let pawn_value = 100;
    let knight_value = 300;
    let rook_value = 500;

    (pawns.count_bits() * pawn_value
        + knights.count_bits() * knight_value
        + rooks.count_bits() * rook_value) as i32
}

/// Evaluate the board position.
pub fn evaluate_position(board: &ChessBoard) -> i32 {
    // Material scores
    let white_material = material_value(
        board.white_pawns,
        board.white_knights,
        board.white_rooks,
    );
    let black_material = material_value(
        board.black_pawns,
        board.black_knights,
        board.black_rooks,
    );

    // Positional bonuses (simplified example)
    let positional_bonus = board.white_pawns.count_bits() as i32 * 10
        - board.black_pawns.count_bits() as i32 * 10;

    // Total evaluation
    white_material - black_material + positional_bonus
}
