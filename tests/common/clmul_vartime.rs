use binary_afft::bits::*;

pub fn clmul_vartime(a: u64, b: u64) -> u128 {
    let mut result: u128 = 0;
    for i in 0..64 {
        if get_bit(b, i as usize) {
            result ^= (a as u128) << i;
        }
    }
    result
}