use crate::board::ChessBoard;

/// Parse a FEN string into a ChessBoard structure.
pub fn parse_fen(fen: &str) -> ChessBoard {
    let parts: Vec<&str> = fen.split_whitespace().collect();
    let board_state = parts[0];

    let mut board = ChessBoard::new();
    let mut square = 0;

    for c in board_state.chars() {
        match c {
            'P' => board.white_pawns.set_bit(square),
            'R' => board.white_rooks.set_bit(square),
            'N' => board.white_knights.set_bit(square),
            'K' => board.white_king.set_bit(square),
            'p' => board.black_pawns.set_bit(square),
            'r' => board.black_rooks.set_bit(square),
            'n' => board.black_knights.set_bit(square),
            'k' => board.black_king.set_bit(square),
            '/' => square -= 8, // Move to the next rank
            '1'..='8' => square += c.to_digit(10).unwrap() as u8,
            _ => (),
        }
        square += 1;
    }

    board
}

/// Convert a ChessBoard structure to a FEN string.
pub fn to_fen(board: &ChessBoard) -> String {
    let mut fen = String::new();
    for rank in (0..8).rev() {
        let mut empty = 0;
        for file in 0..8 {
            let square = rank * 8 + file;
            if board.white_pawns.is_bit_set(square) {
                if empty > 0 {
                    fen.push_str(&empty.to_string());
                    empty = 0;
                }
                fen.push('P');
            } else if board.black_pawns.is_bit_set(square) {
                if empty > 0 {
                    fen.push_str(&empty.to_string());
                    empty = 0;
                }
                fen.push('p');
            } else {
                empty += 1;
            }
        }
        if empty > 0 {
            fen.push_str(&empty.to_string());
        }
        if rank > 0 {
            fen.push('/');
        }
    }
    fen
}
