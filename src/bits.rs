pub fn bit_zero(x: u64) -> u8 {
    (x&1) as u8
}

pub fn binary_string(x: u64) -> String {
    format!("{x:064b}")
}