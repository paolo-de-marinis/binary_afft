use crate::field::portable::{mul_p128,square_p128};
use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gf128(u128);
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
}
impl Mul for Gf128 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(mul_p128(self.0, other.0))
    }
}
impl Add for Gf128 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0^other.0)
    }
}
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