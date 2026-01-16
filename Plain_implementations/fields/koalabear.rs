use super::montgomery_31::{Monty31Params, MontyField31};
use super::{FieldElement, PrimeField};
use core::fmt;
use num_bigint::BigUint;
use num_traits::ToPrimitive;

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct KoalaBear(pub(crate) MontyField31<KoalaBearParams>);

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct KoalaBearParams;

impl Monty31Params for KoalaBearParams {
    const PRIME: u32 = 0x7f00_0001;
    const MONTY_MU: u32 = 0x8100_0001;
}

impl KoalaBear {
    pub const MODULUS: u32 = KoalaBearParams::PRIME;

    pub fn from_u32(val: u32) -> Self {
        Self(MontyField31::new(val))
    }

    pub fn to_u32(&self) -> u32 {
        self.0.to_u32()
    }
}

impl fmt::Debug for KoalaBear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.to_u32(), f)
    }
}

impl FieldElement for KoalaBear {
    fn zero() -> Self {
        Self(MontyField31::new_monty(0))
    }

    fn one() -> Self {
        Self(MontyField31::new(1))
    }

    fn from_u64(val: u64) -> Self {
        Self(MontyField31::from_u64(val))
    }

    fn add_assign(&mut self, other: &Self) {
        self.0.add_assign(&other.0);
    }

    fn sub_assign(&mut self, other: &Self) {
        self.0.sub_assign(&other.0);
    }

    fn mul_assign(&mut self, other: &Self) {
        self.0.mul_assign(&other.0);
    }
}

impl PrimeField for KoalaBear {
    fn modulus() -> BigUint {
        BigUint::from(KoalaBearParams::PRIME as u64)
    }

    fn from_biguint(value: &BigUint) -> Self {
        let modulus = Self::modulus();
        let reduced = value % &modulus;
        let v = reduced.to_u64().expect("KoalaBear fits into u64");
        Self::from_u32(v as u32)
    }

    fn generator() -> BigUint {
        BigUint::from(3u32)
    }
}
