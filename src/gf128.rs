// Reduce a polynomial modulo p(X) = X^128 + X^7 + X^2 + X + 1.
pub fn reduce_p128(l: u128, h: u128) -> u128 {
    // First fold: keep the coefficients of h that remain below degree 128.
    let mut low = l ^ h ^ (h << 1) ^ (h << 2) ^ (h << 7);
    // Recover the coefficients that crossed degree 127 and reindex them
    // after factoring out X^128.
    let overflow = (h >> 127) ^ (h >> 126) ^ (h >> 121);
    // Second fold: replace the remaining X^128 factor using
    // X^128 = X^7 + X^2 + X + 1 modulo p(X).
    low = low ^ overflow ^ (overflow << 1) ^ (overflow << 2) ^ (overflow << 7);
    low
}