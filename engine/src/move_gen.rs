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
pub fn generate_knight_moves(knights: Bitboard) -> Bitboard {
    // Masks for wrapping around the board
    let not_file_a = Bitboard(0xFEFEFEFEFEFEFEFE); // Clear file A
    let not_file_h = Bitboard(0x7F7F7F7F7F7F7F7F); // Clear file H
    let not_file_ab = Bitboard(0xFCFCFCFCFCFCFCFC); // Clear files A and B
    let not_file_gh = Bitboard(0x3F3F3F3F3F3F3F3F); // Clear files G and H

    // Generate all knight moves
    let up_left = (knights.shift_left(17)) & not_file_h;
    let up_right = (knights.shift_left(15)) & not_file_a;
    let down_left = (knights.shift_right(17)) & not_file_h;
    let down_right = (knights.shift_right(15)) & not_file_a;
    let left_up = (knights.shift_left(10)) & not_file_gh;
    let left_down = (knights.shift_right(6)) & not_file_gh;
    let right_up = (knights.shift_left(6)) & not_file_ab;
    let right_down = (knights.shift_right(10)) & not_file_ab;

    // Combine all possible moves
    up_left | up_right | down_left | down_right | left_up | left_down | right_up | right_down
}

/// Example for sliding pieces (rooks, bishops, queens)
pub fn generate_rook_moves(rook: Bitboard, occupied: Bitboard) -> Bitboard {
    const DIRECTIONS: [i8; 4] = [8, -8, 1, -1]; // Up, down, right, left
    let mut moves = Bitboard::new();

    for &direction in DIRECTIONS.iter() {
        let mut temp = rook;
        while temp != Bitboard(0) {
            // Shift in the given direction
            temp = if direction > 0 {
                temp.shift_left(direction as u8)
            } else {
                temp.shift_right((-direction) as u8)
            };

            // Stop if moving out of bounds
            if temp == Bitboard(0) {
                break;
            }

            // Add this square to possible moves
            moves |= temp;

            // Stop sliding if an occupied square is encountered
            if temp & occupied != Bitboard(0) {
                break;
            }
        }
    }

    moves
}


pub fn generate_bishop_moves(bishops: Bitboard, occupied: Bitboard) -> Bitboard {
    const DIRECTIONS: [i8; 4] = [9, -9, 7, -7]; // Diagonal directions: NE, SW, NW, SE
    let mut moves = Bitboard::new();

    for &direction in DIRECTIONS.iter() {
        let mut temp = bishops;
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


pub fn generate_queen_moves(queen: Bitboard, occupied: Bitboard) -> Bitboard {
    generate_rook_moves(queen, occupied) | generate_bishop_moves(queen, occupied)
}


pub fn generate_black_pawn_moves(pawns: Bitboard, empty: Bitboard) -> Bitboard {
    // Single forward push (down)
    let single_push = pawns.shift_right(8) & empty;

    // Double forward push (from rank 7)
    let double_push = (single_push.shift_right(8)) & empty & Bitboard(0x00FF000000000000); // Only rank 7

    single_push | double_push
}


pub fn generate_king_moves(king: Bitboard) -> Bitboard {
    let not_file_a = Bitboard(0xFEFEFEFEFEFEFEFE); // Clear file A
    let not_file_h = Bitboard(0x7F7F7F7F7F7F7F7F); // Clear file H

    let mut moves = Bitboard::new();
    moves |= king.shift_left(8); // Up
    moves |= king.shift_right(8); // Down
    moves |= (king.shift_left(1) & not_file_h); // Right
    moves |= (king.shift_right(1) & not_file_a); // Left
    moves |= (king.shift_left(9) & not_file_h); // Up-right
    moves |= (king.shift_left(7) & not_file_a); // Up-left
    moves |= (king.shift_right(9) & not_file_a); // Down-left
    moves |= (king.shift_right(7) & not_file_h); // Down-right

    moves
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
    let knight_moves = generate_knight_moves(knights); // Knight logic is identical for both colors
    let rook_moves = generate_rook_moves(rooks, occupied);       // Rook logic is identical for both colors
    pawn_moves | knight_moves | rook_moves
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
    let knight_moves = generate_knight_moves(knights);
    let rook_moves = generate_rook_moves(rooks, occupied);

    pawn_moves | knight_moves | rook_moves
}


