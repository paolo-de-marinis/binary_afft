use stdrandom::random_u128;
use binary_toolkit::poly::*;
#[path = "common/slow_clmul.rs"]
mod slow_clmul_ref;

use slow_clmul_ref::slow_clmul;

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