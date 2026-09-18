use crate::bits::*;

pub fn gf4_add(left: u8, right: u8) -> u8 {
    assert!(left < 4);
    assert!(right < 4);
    left ^ right
}

pub fn gf4_mul(left: u8, right: u8) -> u8 {
    assert!(left < 4);
    assert!(right < 4);
    let a = get_bit(left as u64, 0) as u8;
    let b = get_bit(left as u64, 1) as u8;
    let c = get_bit(right as u64, 0) as u8;
    let d = get_bit(right as u64, 1) as u8;
    (a&c^b&d)|((a&d^b&c^b&d) <<1)
}