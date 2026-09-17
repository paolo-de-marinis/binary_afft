use crate::bits::*;

pub fn poly_degree(p: u128) -> Option<usize> {
    if p == 0 {
        None
    } else {
        Some((127-p.leading_zeros()) as usize)
    }
}

pub fn clmul(a: u64, b: u64) -> u128 {
    let mut result: u128 = 0;
    for i in 0..64 {
        if get_bit(b, i as usize) {
            result ^= (a as u128) << i;
        }
    }
    result
}