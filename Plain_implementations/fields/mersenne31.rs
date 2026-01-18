use super::{FieldElement, PrimeField, PrimeFieldExt, PrimeFieldWords};
use core::fmt;
use num_bigint::BigUint;
use num_traits::ToPrimitive;

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Mersenne31(pub(crate) u32);

impl Mersenne31 {
    pub const MODULUS: u32 = 0x7FFF_FFFF;

    #[inline]
    pub const fn from_u32(val: u32) -> Self {
        Self(reduce_u32(val))
    }

    #[inline]
    pub const fn to_u32(&self) -> u32 {
        self.0
    }
}

impl fmt::Debug for Mersenne31 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.to_u32(), f)
    }
}

impl FieldElement for Mersenne31 {
    fn zero() -> Self {
        Self(0)
    }

    fn one() -> Self {
        Self(1)
    }

    fn from_u64(val: u64) -> Self {
        Self(reduce_u64(val))
    }

    fn add_assign(&mut self, other: &Self) {
        let sum = self.0 + other.0;
        self.0 = reduce_u32(sum);
    }

    fn sub_assign(&mut self, other: &Self) {
        if self.0 >= other.0 {
            self.0 -= other.0;
        } else {
            self.0 = self.0 + Self::MODULUS - other.0;
        }
    }

    fn mul_assign(&mut self, other: &Self) {
        let prod = (self.0 as u64) * (other.0 as u64);
        self.0 = reduce_u64(prod);
    }
}

impl PrimeField for Mersenne31 {
    fn modulus() -> BigUint {
        BigUint::from(Mersenne31::MODULUS as u64)
    }

    fn from_biguint(value: &BigUint) -> Self {
        let modulus = Self::modulus();
        let reduced = value % &modulus;
        let v = reduced.to_u64().expect("Mersenne31 fits into u64");
        Self::from_u64(v)
    }

    fn generator() -> BigUint {
        BigUint::from(7u32)
    }
}

impl PrimeFieldExt for Mersenne31 {
    fn to_biguint(&self) -> BigUint {
        BigUint::from(self.to_u32() as u64)
    }
}

impl PrimeFieldWords for Mersenne31 {
    fn to_words_le(&self) -> [u64; 4] {
        [self.to_u32() as u64, 0, 0, 0]
    }
}

#[inline]
const fn reduce_u32(val: u32) -> u32 {
    let p = Mersenne31::MODULUS;
    let mut res = (val & p) + (val >> 31);
    if res >= p {
        res -= p;
    }
    res
}

#[inline]
fn reduce_u64(val: u64) -> u32 {
    let p = Mersenne31::MODULUS as u64;
    let mut res = (val & p) + (val >> 31);
    res = (res & p) + (res >> 31);
    if res >= p {
        res -= p;
    }
    res as u32
}
