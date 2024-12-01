use crate::bitboard::Bitboard;
use crate::board::ChessBoard;
use crate::move_gen::{generate_white_moves, generate_black_moves};

/// A structure to hold the chessboard state.


/// Ensure the king is not in check after a move.
pub fn is_king_safe(board: &ChessBoard, color: bool, move_mask: Bitboard) -> bool {
    let king = if color {
        board.white_king & !move_mask
    } else {
        board.black_king & !move_mask
    };

    if color {
        !is_attacked_by_black(board, king)
    } else {
        !is_attacked_by_white(board, king)
    }
}

/// Check if a square is attacked by black pieces.
pub fn is_attacked_by_black(board: &ChessBoard, square: Bitboard) -> bool {
    let black_moves = generate_black_moves(
        board.black_pawns,
        board.black_knights,
        board.black_rooks,
        board.occupied,
        board.empty
    );
    (square & black_moves) != Bitboard(0)
}

/// Check if a square is attacked by white pieces.
pub fn is_attacked_by_white(board: &ChessBoard, square: Bitboard) -> bool {
    let white_moves = generate_white_moves(
        board.white_pawns,
        board.white_knights,
        board.white_rooks,
        board.occupied,
        board.empty
    );
    (square & white_moves) != Bitboard(0)
}

/// Filter pseudo-legal moves into fully legal moves.
pub fn filter_legal_moves(board: &ChessBoard, color: bool, moves: Bitboard) -> Bitboard {
    let mut legal_moves = Bitboard::new();
    let mut temp = moves;

    while temp != Bitboard(0) {
        let next_move = Bitboard(1 << temp.0.trailing_zeros());
        if is_king_safe(board, color, next_move) {
            legal_moves |= next_move;
        }
        temp &= !next_move;
    }

    legal_moves
}
