pub fn poly_degree(p: u128) -> Option<usize> {
    if p == 0 {
        None
    } else {
        Some((127-p.leading_zeros()) as usize)
    }
}
