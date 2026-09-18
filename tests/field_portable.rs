use stdrandom::random_u128;
use binary_afft::field::portable::*;
#[path = "common/slow_clmul.rs"]
mod slow_clmul_ref;
#[path = "common/slow_reduce.rs"]
mod slow_reduce_ref;

use slow_clmul_ref::slow_clmul;
use slow_reduce_ref::slow_reduce;

#[test]
fn clmuls_match_reference() {
    for _ in 0..1000 {
        let a = random_u128() as u64;
        let b = random_u128() as u64;
        assert_eq!(clmul(a,b), slow_clmul(a as u128,b as u128).0);
        let a = random_u128();
        let b = random_u128();
        assert_eq!(clmul128(a, b), slow_clmul(a, b));
    }
}    

#[test]
fn reduction_matches_the_slow_reference() {
    for _ in 0..10000 {
        let l = random_u128();
        let h = random_u128();  
        assert_eq!(reduce_p128(l, h), slow_reduce(l, h));
    }
}    

#[test]
fn test_spread() {
    let a: u64 = 0b111111111;
    let b: u128 = 0b10101010101010101;
    let c = spread(a);
    assert_eq!(c, b);
}