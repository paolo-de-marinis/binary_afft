use binary_toolkit::polynomial::poly_degree;

#[test]
fn zero_polynomial_has_no_degree() {
    assert_eq!(poly_degree(0), None);
}

#[test]
fn non_zero_constant_polynomial_has_degree_0() {
    assert_eq!(poly_degree(1), Some(0));
}

#[test]
fn degree_tracks_the_highest_power() {
    let x=13;
    assert_eq!(poly_degree(x), Some(3));
    assert_eq!(poly_degree(1<<127), Some(127));
}

#[test]
fn degree_is_not_hamming_weight() {
    let x=1<<7|1;
    assert_eq!(poly_degree(x), Some(7));
}    