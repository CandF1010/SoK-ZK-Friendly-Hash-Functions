use crate::fields::{FieldElement, PrimeField};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Signed, Zero};
use sha3::digest::XofReader;

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

/// Sample a uniformly random field element from a SHAKE128 XOF stream.
///
/// Uses masked-byte rejection sampling: reads `ceil(modulus_bits/8)` bytes,
/// masks the top bits of the last byte, and rejects values ≥ modulus.
/// This is the standard method used by zkfriendlyhashzoo.
pub fn read_field_from_shake<F: PrimeField>(reader: &mut dyn XofReader) -> F {
    let modulus = F::modulus();
    let bits = modulus.bits() as usize;
    let byte_len = (bits + 7) / 8;
    let mod_bits = bits % 8;
    let mask = if mod_bits == 0 { 0xFFu8 } else { (1u8 << mod_bits) - 1 };
    let last = byte_len.saturating_sub(1);
    let mut buf = vec![0u8; byte_len];

    loop {
        reader.read(&mut buf);
        if mod_bits != 0 {
            buf[last] &= mask;
        }
        let val = BigUint::from_bytes_le(&buf);
        if val < modulus {
            return F::from_biguint(&val);
        }
    }
}
