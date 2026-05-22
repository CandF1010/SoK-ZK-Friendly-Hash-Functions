use crate::fields::FieldElement;
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Signed, Zero};

/// Build a Cauchy MDS matrix of size t×t.
///
/// A Cauchy matrix C[i][j] = 1 / (x_i + y_j) where all x_i and y_j are
/// distinct and x_i + y_j ≠ 0.  Every square Cauchy matrix over a field
/// with distinct x_i, y_j is guaranteed to be MDS.
///
/// We use x_i = i+1, y_j = j+1 (both 1-indexed), so C[i][j] = 1/(i+j+2).
/// This is a standard choice also used in Poseidon parameter generation.
pub fn build_cauchy_mds<F: crate::fields::PrimeField>(t: usize) -> Vec<Vec<F>> {
    let modulus = F::modulus();
    let mut m = vec![vec![F::zero(); t]; t];
    for i in 0..t {
        for j in 0..t {
            // C[i][j] = 1 / (i + j + 2)    (i, j are 0-indexed, so +2)
            let denom = BigUint::from((i + j + 2) as u64);
            let inv = modinv(&denom, &modulus);
            m[i][j] = F::from_biguint(&inv);
        }
    }
    m
}

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
