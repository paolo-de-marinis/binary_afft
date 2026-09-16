pub fn bit_zero(x: u64) -> u8 {
    (x&1) as u8
}

pub fn binary_string(x: u64) -> String {
    format!("{x:064b}")
}

pub fn get_bit(x: u64, i: u32) -> bool {
    assert!(i<64);
    ((x >> i) & 1) != 0
}