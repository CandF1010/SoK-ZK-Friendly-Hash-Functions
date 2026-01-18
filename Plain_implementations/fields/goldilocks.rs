use super::{FieldElement, PrimeField, PrimeFieldExt, PrimeFieldWords};
use core::fmt;
use num_bigint::BigUint;
use num_traits::ToPrimitive;

const EPSILON: u64 = (1 << 32) - 1;

#[derive(Clone, Copy, Default)]
pub struct Goldilocks(pub u64);

impl Goldilocks {
    pub const MODULUS: u64 = 0xFFFF_FFFF_0000_0001;

    pub fn from_hex(s: &str) -> Option<Self> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        u64::from_str_radix(s, 16).ok().map(Self::from_u64)
    }

    #[inline]
    fn to_canonical_u64(self) -> u64 {
        let mut c = self.0;
        if c >= Self::MODULUS {
            c -= Self::MODULUS;
        }
        c
    }

    pub fn to_u64(&self) -> u64 {
        self.to_canonical_u64()
    }
}

impl PartialEq for Goldilocks {
    fn eq(&self, other: &Self) -> bool {
        self.to_canonical_u64() == other.to_canonical_u64()
    }
}

impl Eq for Goldilocks {}

impl fmt::Debug for Goldilocks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.to_canonical_u64(), f)
    }
}

impl FieldElement for Goldilocks {
    fn zero() -> Self {
        Self(0)
    }

    fn one() -> Self {
        Self(1)
    }

    fn from_u64(val: u64) -> Self {
        let mut v = val;
        if v >= Self::MODULUS {
            v -= Self::MODULUS;
        }
        Self(v)
    }

    fn add_assign(&mut self, other: &Self) {
        let (sum, over) = self.0.overflowing_add(other.0);
        let (mut sum, over) = sum.overflowing_add((over as u64) * EPSILON);
        if over {
            sum = sum.wrapping_add(EPSILON);
        }
        self.0 = sum;
    }

    fn sub_assign(&mut self, other: &Self) {
        let (diff, under) = self.0.overflowing_sub(other.0);
        let (mut diff, under) = diff.overflowing_sub((under as u64) * EPSILON);
        if under {
            diff = diff.wrapping_sub(EPSILON);
        }
        self.0 = diff;
    }

    fn mul_assign(&mut self, other: &Self) {
        let prod = (self.0 as u128) * (other.0 as u128);
        self.0 = reduce128(prod).0;
    }
}

impl PrimeField for Goldilocks {
    fn modulus() -> BigUint {
        BigUint::from(Goldilocks::MODULUS)
    }

    fn from_biguint(value: &BigUint) -> Self {
        let modulus = Self::modulus();
        let reduced = value % &modulus;
        let v = reduced.to_u64().expect("Goldilocks fits into u64");
        Self::from_u64(v)
    }

    fn generator() -> BigUint {
        BigUint::from(7u32)
    }
}

impl PrimeFieldExt for Goldilocks {
    fn to_biguint(&self) -> BigUint {
        BigUint::from(self.to_u64())
    }
}

impl PrimeFieldWords for Goldilocks {
    fn to_words_le(&self) -> [u64; 4] {
        [self.to_u64(), 0, 0, 0]
    }
}

#[inline(always)]
const fn add_no_canonicalize_trashing_input(x: u64, y: u64) -> u64 {
    let (res_wrapped, carry) = x.overflowing_add(y);
    res_wrapped + EPSILON * (carry as u64)
}

#[inline]
fn reduce128(x: u128) -> Goldilocks {
    let (x_lo, x_hi) = split(x);
    let x_hi_hi = x_hi >> 32;
    let x_hi_lo = x_hi & EPSILON;

    let (mut t0, borrow) = x_lo.overflowing_sub(x_hi_hi);
    if borrow {
        t0 = t0.wrapping_sub(EPSILON);
    }
    let t1 = x_hi_lo * EPSILON;
    let t2 = add_no_canonicalize_trashing_input(t0, t1);
    Goldilocks(t2)
}

#[inline]
const fn split(x: u128) -> (u64, u64) {
    (x as u64, (x >> 64) as u64)
}
