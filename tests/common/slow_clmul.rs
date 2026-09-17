pub fn slow_clmul(a: u128, b: u128) -> (u128,u128) {
    let mut v = [false; 256];
    for i in 0..128 {
        for j in 0..128 {
            v[i+j]^=(((a >> i) & 1) & ((b >> j) & 1))!=0;
        }
    }
    let mut result1: u128 = 0;
    for i in 0..128 {
        if v[i] {
            result1 ^= (1 as u128) << i;
        }
    }
    let mut result2: u128 = 0;
    for i in 0..128 {
        if v[i+128] {
            result2 ^= (1 as u128) << i;
        }
    }
    (result1,result2)
}