// `as u128` is deliberate: it marks the literal as a 128-bit word.
#[allow(clippy::unnecessary_cast)]
pub fn slow_reduce(lo: u128, hi: u128) -> u128 {
    let mut v = [false; 256];
    for i in 0..256 {
        if i < 128 {
            v[i] = ((lo >> i) & 1) != 0;
        } else {
            v[i] = ((hi >> (i-128)) & 1) != 0;
        }
    }
    for j in (128..256).rev() {
        if v[j] {
        v[j-128] ^= v[j];
        v[j-128+1] ^= v[j];
        v[j-128+2] ^= v[j];
        v[j-128+7] ^= v[j];
        v[j] = false;
        }
    }
    
    let mut result: u128 = 0;
    for i in 0..128 {
        if v[i] {
            result ^= (1 as u128) << i;
        }
    }
    result
}