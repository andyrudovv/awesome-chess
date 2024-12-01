use engine::bitboard::Bitboard;

#[test]
fn test_set_and_clear_bit() {
    let mut bb = Bitboard::new();
    bb.set_bit(0);
    assert!(bb.is_bit_set(0));

    bb.clear_bit(0);
    assert!(!bb.is_bit_set(0));
}

#[test]
fn test_shift_operations() {
    let mut bb = Bitboard::new();
    bb.set_bit(0); // Set A1
    let shifted = bb.shift_left(8); // Move up to A2
    assert!(shifted.is_bit_set(8));
    assert!(!shifted.is_bit_set(0));
}
