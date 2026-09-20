use binary_afft::field::Gf128;
use stdrandom::random_u128;
use subtle::{Choice,ConditionallySelectable,ConstantTimeEq};
#[test]
fn gf128_public_api_works() {
    // Construction and reading are inverse.
    let a = Gf128::from_u128(0b1010);
    assert_eq!(a.to_u128(), 0b1010);

    // The three constants carry the assigned words.
    assert_eq!(Gf128::ZERO.to_u128(), 0);
    assert_eq!(Gf128::ONE.to_u128(), 1);
    assert_eq!(Gf128::GENERATOR.to_u128(), 0b10);

    // Addition, subtraction and opposite.
    let b = Gf128::from_u128(0b1100);
    assert_eq!((a + b).to_u128(), 0b0110);
    assert_eq!((a - b).to_u128(), 0b0110);
    assert_eq!(-a, a);

    // The product reaches the backend.
    assert_eq!(Gf128::ONE * Gf128::GENERATOR, Gf128::GENERATOR);
}

#[test]
fn square_matches_multiplication_by_itself() {
    for _ in 0..1000 {
        let a = Gf128::from_u128(random_u128());
        assert_eq!(a.square(), a*a);
    }
}
#[test]
fn multiplication_by_inverse_is_identity() {
    for _ in 0..1000 {
        let a = Gf128::from_u128(random_u128());
        if a.to_u128() == 0 {
            continue;
        }
        assert_eq!(Gf128::ONE, a.inverse()*a);
    }
}

#[test]
#[should_panic]
fn inverse_of_zero_panics() {
    Gf128::ZERO.inverse();
}

#[test]
fn constant_time_equality_works() {
    let a = Gf128::from_u128(42);
    let b = Gf128::from_u128(42);
    let c = Gf128::from_u128(43);

    assert!(bool::from(a.ct_eq(&b)));
    assert!(!bool::from(a.ct_eq(&c)));
}

#[test]
fn conditional_selection_works() {
    let a = Gf128::from_u128(42);
    let b = Gf128::from_u128(43);

    assert_eq!(Gf128::conditional_select(&a,&b,Choice::from(0)),a);

    assert_eq!(Gf128::conditional_select(&a,&b,Choice::from(1)),b);
}
