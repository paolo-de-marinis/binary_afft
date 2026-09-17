use stdrandom::random_u128;
use binary_toolkit::gf128::*;
#[path = "common/slow_reduce.rs"]
mod slow_reduce_ref;

use slow_reduce_ref::slow_reduce;

#[test]
fn reduction_matches_the_slow_reference() {
    for _ in 0..10000 {
        let l = random_u128();
        let h = random_u128();  
        assert_eq!(reduce_p128(l, h), slow_reduce(l, h));
    }
}    