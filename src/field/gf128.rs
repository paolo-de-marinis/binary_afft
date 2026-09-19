use crate::field::portable::{mul_p128,square_p128};
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gf128(u128);
// `self: Self` is deliberate: it keeps the receiver type visible in every signature.
#[allow(clippy::needless_arbitrary_self_type)]
impl Gf128 {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const GENERATOR: Self = Self(2);
    pub fn from_u128(x: u128) -> Self {
        Self(x)
    }
    pub fn to_u128(self: Self) -> u128 {
        self.0
    }
    pub fn square(self: Self) -> Self {
        Self(square_p128(self.0))
    }
    pub fn repeated_square(self: Self, exp: usize) -> Self {
        let mut result = self;
        for _ in 0..exp {
            result = result.square();
        }
        result
    }
    
    pub fn inverse(self: Self) -> Self {
        assert!(self!=Self::ZERO, "Element ZERO of Gf128 has no multiplicative inverse");
        let mut uaux = self; // initialize to u1 = self^(2^1-1) = self
        let mut result = self;
        let v = [1, 2, 3, 6, 7, 8, 15, 30, 60, 120, 127];
        // Use Itoh-Tsujii sequence
        for i in 1..5 {
            let index = v[i]-v[i-1];
            if index == 1 {
                result = result.repeated_square(index)*uaux;
            }
            else {
                result = result.repeated_square(index)*result;
            }
        }

        // Save u7 before building u8.
        uaux = result;

        // Build u8 = u7^2 * u1, with u1 = self.
        result = result.square()*self;
        
        for i in 6..v.len(){
            let index = v[i]-v[i-1];
            if index == 7 {
                result = result.repeated_square(index)*uaux;
            }
            else {
                result = result.repeated_square(index)*result;
            }
        }
        result.square()
    }
}    

impl Mul for Gf128 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(mul_p128(self.0, other.0))
    }
}
// In characteristic two, addition of coordinates is XOR.
#[allow(clippy::suspicious_arithmetic_impl)]
impl Add for Gf128 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0^other.0)
    }
}
// In characteristic two, -b = b, so subtracting is adding.
#[allow(clippy::suspicious_arithmetic_impl)]
impl Sub for Gf128 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0^other.0)
    }
}
impl Neg for Gf128 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(self.0)
    }
}