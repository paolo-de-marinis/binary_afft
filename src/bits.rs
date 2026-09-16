
pub fn get_bit(x: u64, i: usize) -> bool {
    assert!(i<64);
    ((x >> i) & 1) != 0
}

pub fn set_bit(x: u64, i: usize, value: bool) -> u64 {
    assert!(i<64);
    let mask: u64 = 1 << i;
    if value {
        x | mask
    } else {
        x & !mask
    }
}

pub fn toggle_bit(x: u64, i: usize) -> u64 {
    assert!(i<64);
    let mask: u64 = 1 << i;
    x ^ mask
}

pub fn to_bits(x: u64) -> [bool; 64] {
    let mut v = [false; 64];
    for i in 0..64 {
       v[i as usize] = get_bit(x, i);        
    }
    v
}

pub fn from_bits(bits: [bool; 64]) -> u64 {
    let mut x: u64 = 0;
    for i in 0..64 {
    x = set_bit(x,i,bits[i as usize]);
    }
    x
}