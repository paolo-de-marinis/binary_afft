use std::u128;

use crate::bits::*;

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
    let a0 = a as u64;
    let a1 = (a >> 64) as u64;
    let b0 = b as u64;
    let b1 = (b >> 64) as u64;
    // Karatsuba products
    let p0 = clmul(a0, b0);
    let p2 = clmul(a1, b1);
    // let mut p1 = clmul(a0,b1)^clmul(a1,b0);
    let p1 = clmul(a0 ^ a1, b0 ^ b1) ^ p0 ^ p2;
    
    let lo = p0 ^ (p1 << 64);
    let hi = p2 ^ (p1 >> 64);
    (lo,hi)
}

// Reduce a polynomial modulo p(X) = X^128 + X^7 + X^2 + X + 1.
pub fn reduce_p128(l: u128, h: u128) -> u128 {
    // First fold: keep the coefficients of h that remain below index 128 
    // and reduce them using X^128 = X^7 + X^2 + X + 1 modulo p(X).
    let mut low = l ^ h ^ (h << 1) ^ (h << 2) ^ (h << 7);
    // Recover the coefficients of hX , hX^2 and hX^7 
    // that crossed index 127 and reindex them to prepare them for reduction
    let overflow = (h >> 127) ^ (h >> 126) ^ (h >> 121);
    // Second fold: replace the remaining X^128 factor in the overflow
    // and reduce them using X^128 = X^7 + X^2 + X + 1 modulo p(X).
    low = low ^ overflow ^ (overflow << 1) ^ (overflow << 2) ^ (overflow << 7);
    low
}

pub fn mul_p128(a: u128, b: u128) -> u128 {
    let (lo, hi) = clmul128(a, b);
    reduce_p128(lo, hi)
}





const fn spread_mask(i: u32) -> u128 {
    u128::MAX/((1 << i) +1)
}
pub const M32: u128 = spread_mask(32);
pub const M16: u128 = spread_mask(16);
pub const M8: u128 = spread_mask(8);
pub const M4: u128 = spread_mask(4);
pub const M2: u128 = spread_mask(2);
pub const M1: u128 = spread_mask(1);

pub fn spread(a: u64) -> u128 {
    let mut result= a as u128;
    /* Inefficient spread
    for i in 0..64 {
        result ^= (get_bit(a, i) as u128)<<2*i;
    } 
    */
    for (i,m) in [(32,M32), (16,M16), (8,M8), (4,M4), (2,M2), (1,M1)]{
        result = (result | (result << i)) & m;
    }
    result
}





pub fn spread128(a: u128) -> (u128, u128) {
    let a0 = a as u64;
    let a1 = (a >> 64) as u64;
    // Karatsuba products
    let p0 = spread(a0);
    let p2 = spread(a1);
    
    let lo = p0;
    let hi = p2;
    (lo,hi)
}

pub fn square_p128(a: u128) -> u128 {
    let (lo, hi) = spread128(a);
    reduce_p128(lo, hi)
}