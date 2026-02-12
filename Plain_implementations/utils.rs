use crate::fields::FieldElement;
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Signed, Zero};

pub(crate) fn modinv(value: &BigUint, modulus: &BigUint) -> BigUint {
    let mut t = BigInt::zero();
    let mut new_t = BigInt::one();
    let mut r = BigInt::from(modulus.clone());
    let mut new_r = BigInt::from(value.clone());

    while !new_r.is_zero() {
        let quotient = &r / &new_r;
        let next_t = &t - &quotient * &new_t;
        t = new_t;
        new_t = next_t;
        let next_r = &r - &quotient * &new_r;
        r = new_r;
        new_r = next_r;
    }

    if r != BigInt::one() {
        panic!("value is not invertible modulo modulus");
    }

    if t.is_negative() {
        t += BigInt::from(modulus.clone());
    }

    t.try_into().expect("modular inverse must be non-negative")
}

pub(crate) fn pow_biguint<F: FieldElement>(base: &F, exp: &BigUint) -> F {
    let mut result = F::one();
    let mut base_power = base.clone();
    let mut e = exp.clone();

    while !e.is_zero() {
        if (&e & BigUint::one()) == BigUint::one() {
            result.mul_assign(&base_power);
        }
        e >>= 1;
        if !e.is_zero() {
            base_power.square();
        }
    }

    result
}
