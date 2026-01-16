pub mod babybear;
pub mod bls12_381;
pub mod bn254;
pub mod felt252;
pub mod goldilocks;
pub mod koalabear;
mod montgomery_4;
mod montgomery_31;

use num_bigint::BigUint;

pub trait FieldElement: Clone + Default + PartialEq + Eq + std::fmt::Debug {
    fn zero() -> Self;
    fn one() -> Self;
    fn from_u64(val: u64) -> Self;

    fn add_assign(&mut self, other: &Self);
    fn sub_assign(&mut self, other: &Self);
    fn mul_assign(&mut self, other: &Self);

    fn square(&mut self) {
        let tmp = self.clone();
        self.mul_assign(&tmp);
    }

    fn double(&mut self) {
        let tmp = self.clone();
        self.add_assign(&tmp);
    }

    fn negate(&self) -> Self {
        let mut out = Self::zero();
        out.sub_assign(self);
        out
    }

    fn pow_u64(&self, mut exp: u64) -> Self {
        let mut base = self.clone();
        let mut result = Self::one();
        while exp > 0 {
            if exp & 1 == 1 {
                result.mul_assign(&base);
            }
            exp >>= 1;
            if exp > 0 {
                base.square();
            }
        }
        result
    }
}

pub trait PrimeField: FieldElement {
    fn modulus() -> BigUint;
    fn from_biguint(value: &BigUint) -> Self;
    fn generator() -> BigUint;
}
