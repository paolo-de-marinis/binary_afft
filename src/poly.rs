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

pub fn clmul128(a: u128, b: u128) -> (u128,u128) {
    let mut a0 = a as u64;
    let mut a1 = (a >> 64) as u64;
    let mut b0 = b as u64;
    let mut b1 = (b >> 64) as u64;
    let mut lo: u128 = 0;
    let mut hi: u128 = 0;
    // Karatsuba products
    let mut p0 = clmul(a0, b0);
    let mut p2 = clmul(a1, b1);
    // let mut p1 = clmul(a0,b1)^clmul(a1,b0);
    let p1 = clmul(a0 ^ a1, b0 ^ b1) ^ p0 ^ p2;
    
    lo = p0 ^ (p1 << 64);
    hi = p2 ^ (p1 >> 64);
    (lo,hi)
}