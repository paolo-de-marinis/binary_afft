use stdrandom::random_u128;
use binary_toolkit::gf128::*;
mod common;
use common::*;

#[test]
fn reduction_matches_the_slow_reference() {
    for _ in 0..10000 {
        let l = random_u128();
        let h = random_u128();  
        assert_eq!(reduce_p128(l, h), slow_reduce(l, h));
    }
}    