use binary_afft::bits::*;

#[test]
fn word_bits_round_trip() {
    let words: [u64; 5] = [
        0,
        1,
        13,
        1u64 << 63,
        u64::MAX,
    ];

    for x in words {
        assert_eq!(from_bits(to_bits(x)), x);
    }
}

#[test]
fn bits_word_round_trip() {
    // All coordinates are false.
    let all_false: [bool; 64] = [false; 64];

    // All coordinates are true.
    let all_true: [bool; 64] = [true; 64];

    // Exactly one coordinate is true.
    let mut single_true: [bool; 64] = [false; 64];
    single_true[37] = true;

    // Alternating coordinates: false, true, false, true, ...
    let mut alternating: [bool; 64] = [false; 64];

    for i in 0..64 {
        alternating[i] = i % 2 == 1;
    }

    let bit_arrays: [[bool; 64]; 4] = [
        all_false,
        all_true,
        single_true,
        alternating,
    ];

    for bits in bit_arrays {
        assert_eq!(to_bits(from_bits(bits)), bits);
    }
}

#[test]
fn thirteen_has_expected_coordinates() {
    let bits = to_bits(13);

    for i in 0..64 {
        let expected = i == 0 || i == 2 || i == 3;

        assert_eq!(bits[i], expected);
    }
}

#[test]
fn get_bit_reads_the_same_coordinate_as_to_bits() {
    let words: [u64; 4] = [
        13,
        0xAAAAAAAAAAAAAAAA,
        0x5555555555555555,
        0x123456789ABCDEF0,
    ];

    for x in words {
        for i in 0..64 {
            assert_eq!(get_bit(x, i), to_bits(x)[i]);
        }
    }
}

#[test]
fn set_bit_changes_only_one_coordinate() {
    let x: u64 = 13;
    let index: usize = 5;
    let value: bool = true;

    let before = to_bits(x);
    let after = to_bits(set_bit(x, index, value));
    for i in 0..64 {
        if i==index {
            assert_ne!(before[i], after[i]);
        } else {
            assert_eq!(before[i], after[i]);
        }
    }
}    

#[test]
fn toggle_bit_is_an_involution() {
    let x: u64 = 0x123456789ABCDEF0;

    let indices: [usize; 6] = [
        0,
        1,
        17,
        32,
        48,
        63,
    ];

    for i in indices {
        assert_eq!(toggle_bit(toggle_bit(x, i), i), x);
    }
}

#[test]
#[should_panic]
fn bit_index_out_of_range_panics() {
    get_bit(13, 64);
}

#[test]
fn xor_matches_coordinate_addition() {
    let x: u64 = 13;
    let y: u64 = 101;

    let x_bits = to_bits(x);
    let y_bits = to_bits(y);

    let mut sum_bits: [bool; 64] = [false; 64];

    for i in 0..64 {
        sum_bits[i] = x_bits[i] ^ y_bits[i];
    }

    let coordinate_sum = from_bits(sum_bits);

    assert_eq!(coordinate_sum, x ^ y);
}