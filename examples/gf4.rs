use binary_afft::bits::*;

pub fn gf4_add(left: u8, right: u8) -> u8 {
    assert!(left < 4);
    assert!(right < 4);
    left ^ right
}

pub fn gf4_mul(left: u8, right: u8) -> u8 {
    assert!(left < 4);
    assert!(right < 4);
    let a = get_bit(left as u64, 0) as u8;
    let b = get_bit(left as u64, 1) as u8;
    let c = get_bit(right as u64, 0) as u8;
    let d = get_bit(right as u64, 1) as u8;
    (a&c^b&d)|((a&d^b&c^b&d) <<1)
}

fn main() {
    let alpha: u8 = 2;
    assert_eq!(gf4_add(alpha, 1), 3);
    assert_eq!(gf4_mul(alpha, alpha), 3);
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
fn gf4_generator_satisfies_defining_relation() {
    let u: u8 = 2;
    assert_eq!(gf4_mul(u, u), 3);
}

#[test]
fn gf4_addition_is_coordinatewise_xor(){
    for u in 0..4 {
        for v in 0..4 {
            assert_eq!(gf4_add(u, v), u^v);
        }
    }
}

#[test]
fn gf4_multiplication_stays_in_the_field() {
    for u in 0..4 {
        for v in 0..4 {
            assert!(gf4_mul(u, v) < 4);
        }
    }
    assert_ne!(gf4_mul(2,2),2&2);
}

#[test]
fn gf4_field_laws_hold() {
    for x in 0..4 {
        assert_eq!(gf4_mul(x, 1),x);
        assert_eq!(gf4_add(x,0),x);
        for y in 0..4 {
            assert_eq!(gf4_mul(x, y), gf4_mul(y, x));
            assert_eq!(gf4_add(x, y), gf4_add(y, x));
            for z in 0..4 {
                assert_eq!(gf4_mul(x, gf4_add(y, z)), gf4_add(gf4_mul(x, y), gf4_mul(x, z)));
                assert_eq!(gf4_mul(x, gf4_mul(y, z)), gf4_mul(gf4_mul(x, y), z));
                assert_eq!(gf4_add(x, gf4_add(y, z)), gf4_add(gf4_add(x, y),z)); 
            }
            
        }
    }

    let mut has_mul_inverse = false;
    let mut has_sum_inverse = false;
    for x in 1..4 {
        for y in 1..4 {
            if gf4_mul(x, y) == 1 {
                has_mul_inverse = true;
            }
            if gf4_add(x, y) == 0 {
                has_sum_inverse = true;
            }
            if has_mul_inverse && has_sum_inverse == true {
                break;
            }
        }
        assert_eq!(has_mul_inverse && has_sum_inverse,true);
    }    
}

#[test]
#[should_panic]
fn gf4_add_rejects_noncanonical_masks() {
    gf4_add(3, 10);
}

#[test]
#[should_panic]
fn gf4_mul_rejects_noncanonical_masks() {
    gf4_mul(3, 10);
}
}
