use binary_toolkit::bits::*;

#[test]
fn test_constant_in_different_bases() {
    assert_eq!(13u64, 0b1101u64);
    assert_ne!(13u64, 0b1011u64);
}

#[test]
fn test_get_bit() {
    // 13 in binary is 1101
    assert_eq!(get_bit(13, 0), true);
    assert_eq!(get_bit(13, 1), false);
    assert_eq!(get_bit(13, 2), true);
    assert_eq!(get_bit(13, 3), true);
}

#[test]
fn test_set_bit() {
    // Setting a bit that is already set does not change the value
    assert_eq!(set_bit(13, 0, true), 13);
}

#[test]
fn test_toggle_bit() {
    // 13 (1101) with bit 0 toggled becomes 12 (1100)
    assert_eq!(toggle_bit(13, 0), 12);

    // Involution property: toggle_bit(toggle_bit(x, i), i) == x
    assert_eq!(toggle_bit(toggle_bit(13, 2), 2), 13);
}

#[test]
fn test_to_bits() {
    let x: u64 = 1;

    // Initialize all entries to false (0)
    let mut expected = [false; 64];

    // Set only the least significant bit
    expected[0] = true;

    assert_eq!(to_bits(x), expected);
}

#[test]
fn test_to_bits_13() {
    let mut expected = [false; 64];

    // 13 in binary is 1101.
    // The array stores bits by increasing position:
    // bits[i] is the bit with weight 2^i.
    // Therefore the first entries are [1, 0, 1, 1].
    expected[0] = true;
    expected[1] = false;
    expected[2] = true;
    expected[3] = true;

    assert_eq!(to_bits(13), expected);
}

#[test]
fn test_from_bits() {
    let x: u64 = 1;
    let mut expected: [bool; 64] = [false; 64];

    // Set only the least significant bit
    expected[0] = true;

    assert_eq!(from_bits(expected), x);
}

#[test]
fn binary_coordinates_and_words_are_inverse() {
    let x: u64 = 1;
    let mut expected: [bool; 64] = [false; 64];
    // Set only the least significant bit
    expected[0] = true;
    // Converting to bits and back must recover the original value
    assert_eq!(from_bits(to_bits(x)), x);
    // Converting to words and back must recover the original value
    assert_eq!(to_bits(from_bits(expected)), expected);
}