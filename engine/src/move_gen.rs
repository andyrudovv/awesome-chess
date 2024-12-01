use crate::bitboard::Bitboard;



/// Generate all pseudo-legal moves for white pawns
pub fn generate_white_pawn_moves(pawns: Bitboard, empty: Bitboard) -> Bitboard {
    // Single forward push
    let single_push = pawns.shift_left(8) & empty;

    // Double forward push
    let double_push = (single_push.shift_left(8)) & empty & Bitboard(0x0000FF00); // Only rank 3

    single_push | double_push
}

/// Generate knight moves from a given position
pub fn generate_knight_moves(knights: Bitboard, occupied: Bitboard) -> Bitboard {
    let left = knights.shift_left(17) & !Bitboard(0x8080808080808080); // Avoid wrapping on files
    let right = knights.shift_left(15) & !Bitboard(0x0101010101010101);
    left | right // Combine possible moves
}

/// Example for sliding pieces (rooks, bishops, queens)
pub fn generate_rook_moves(rook: Bitboard, occupied: Bitboard) -> Bitboard {
    const DIRECTIONS: [i8; 4] = [8, -8, 1, -1]; // Up, down, right, left
    let mut moves = Bitboard::new();

    for &direction in DIRECTIONS.iter() {
        let mut temp = rook;
        while temp != Bitboard(0) {
            temp = if direction > 0 {
                temp.shift_left(direction as u8)
            } else {
                temp.shift_right((-direction) as u8)
            };

            // Stop if we encounter an occupied square
            if temp & occupied != Bitboard(0) {
                break;
            }

            moves |= temp;
        }
    }

    moves
}

/// Generate all pseudo-legal moves for white pieces
pub fn generate_white_moves(
    pawns: Bitboard,
    knights: Bitboard,
    rooks: Bitboard,
    occupied: Bitboard,
    empty: Bitboard,
) -> Bitboard {
    let pawn_moves = generate_white_pawn_moves(pawns, empty);
    let knight_moves = generate_knight_moves(knights, occupied);
    let rook_moves = generate_rook_moves(rooks, occupied);
    pawn_moves | knight_moves | rook_moves
}

pub fn generate_black_pawn_moves(pawns: Bitboard, empty: Bitboard) -> Bitboard {
    // Single forward push (down)
    let single_push = pawns.shift_right(8) & empty;

    // Double forward push (from rank 7)
    let double_push = (single_push.shift_right(8)) & empty & Bitboard(0x00FF000000000000); // Only rank 7

    single_push | double_push
}

/// Generate all pseudo-legal moves for black pieces
pub fn generate_black_moves(
    pawns: Bitboard,
    knights: Bitboard,
    rooks: Bitboard,
    occupied: Bitboard,
    empty: Bitboard,
) -> Bitboard {
    let pawn_moves = generate_black_pawn_moves(pawns, empty);
    let knight_moves = generate_knight_moves(knights, occupied); // Knight logic is identical for both colors
    let rook_moves = generate_rook_moves(rooks, occupied);       // Rook logic is identical for both colors
    pawn_moves | knight_moves | rook_moves
}