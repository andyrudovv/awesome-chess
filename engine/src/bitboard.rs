
/*

    56 57 58 59 60 61 62 63   (Rank 8: A8 - H8)
    48 49 50 51 52 53 54 55   (Rank 7: A7 - H7)
    40 41 42 43 44 45 46 47   (Rank 6: A6 - H6)
    32 33 34 35 36 37 38 39   (Rank 5: A5 - H5)
    24 25 26 27 28 29 30 31   (Rank 4: A4 - H4)
    16 17 18 19 20 21 22 23   (Rank 3: A3 - H3)
     8  9 10 11 12 13 14 15   (Rank 2: A2 - H2)
     0  1  2  3  4  5  6  7   (Rank 1: A1 - H1)

 */


/// A `Bitboard` represents a chessboard using a 64-bit integer.
/// Each bit corresponds to a square on the chessboard:
/// Bit 0 represents A1, Bit 63 represents H8.

use std::ops::{Not, BitAnd, BitOr, BitOrAssign, BitAndAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    /// Creates a new, empty bitboard (all bits set to 0).
    pub fn new() -> Self {
        Bitboard(0)
    }

    pub fn count_bits(&self) -> u32 {
        self.0.count_ones()
    }

    /// Sets a bit at the specified square (marks the square as occupied).
    pub fn set_bit(&mut self, square: u8) {
        self.0 |= 1 << square;
    }

    /// Clears a bit at the specified square (marks the square as empty).
    pub fn clear_bit(&mut self, square: u8) {
        self.0 &= !(1 << square);
    }

    /// Checks if a bit at the specified square is set (occupied).
    pub fn is_bit_set(&self, square: u8) -> bool {
        (self.0 & (1 << square)) != 0
    }

    /// Shifts the bitboard left by `n` bits.
    pub fn shift_left(&self, n: u8) -> Bitboard {
        Bitboard(self.0 << n)
    }

    /// Shifts the bitboard right by `n` bits.
    pub fn shift_right(&self, n: u8) -> Bitboard {
        Bitboard(self.0 >> n)
    }
}

/// Implement the `Not` trait for the `Bitboard` struct to invert all bits.
impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0) // Invert all bits in the underlying u64
    }
}

/// Implement the `BitAnd` trait for the `Bitboard` struct.
impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0) // Perform a bitwise AND on the two bitboards
    }
}

/// Implement the `BitAndAssign` trait for the `Bitboard` struct.
impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0; // Perform a bitwise AND assignment on the bitboard
    }
}

/// Implement the `BitOr` trait for the `Bitboard` struct.
impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0) // Perform a bitwise OR on the two bitboards
    }
}

/// Implement the `BitOrAssign` trait for the `Bitboard` struct.
impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0; // Perform a bitwise OR assignment on the bitboard
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitand_operator() {
        let bb1 = Bitboard(0b00000001_00000001);
        let bb2 = Bitboard(0b00000001_00000000);
        let result = bb1 & bb2; // Bitwise AND
        assert_eq!(result.0, 0b00000001_00000000);
    }
}
