use super::xhash::XHashParams;
use crate::utils::build_cauchy_mds;
use crate::fields::babybear::BabyBear;
use crate::fields::bls12_381::Bls12_381;
use crate::fields::bn254::Bn254;
use crate::fields::goldilocks::Goldilocks;
use crate::fields::koalabear::KoalaBear;
use crate::fields::mersenne31::Mersenne31;
use crate::fields::{biguint_from_limbs_le, PrimeField};
use lazy_static::lazy_static;
use num_traits::Zero;
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::Shake128;
use std::sync::Arc;

fn generate_xhash<F: PrimeField>(t: usize, label: &str) -> Arc<XHashParams<F>> {
    let modulus = F::modulus();
    let modulus_minus_one = &modulus - num_bigint::BigUint::from(1u64);

    // Choose d: smallest odd >= 3 coprime to p-1.
    let d = {
        let mut d = 3u64;
        loop {
            if gcd_u64_biguint(d, &modulus_minus_one) == num_bigint::BigUint::from(1u64) {
                break d;
            }
            d += 2;
        }
    };
    let d_inv = compute_d_inv(d, &modulus_minus_one);

    // Rounds: ceil(2*log_d(p)) + 2*t + 2 security margin.
    let log_d = {
        let mut exp = 0usize;
        let mut power = num_bigint::BigUint::from(1u64);
        let d_big = num_bigint::BigUint::from(d);
        while power < modulus {
            power *= &d_big;
            exp += 1;
        }
        exp
    };
    let rounds = (2 * log_d + 2 * t + 4).max(6);

    let byte_len = ((modulus.bits() + 7) / 8) as usize;

    let mut shake = Shake128::default();
    shake.update(label.as_bytes());
    let mut mod_bytes = modulus.to_bytes_le();
    mod_bytes.resize(byte_len, 0);
    shake.update(&mod_bytes);
    let mut reader = shake.finalize_xof();

    // Build Cauchy MDS matrix.
    let mds = build_cauchy_mds::<F>(t);

    // Round constants: rounds × t.
    let round_constants: Vec<Vec<F>> = (0..rounds)
        .map(|_| (0..t).map(|_| read_field(&mut reader, &modulus, byte_len)).collect())
        .collect();

    Arc::new(XHashParams::new(t, d, d_inv, rounds, &mds, &round_constants))
}

fn gcd_u64_biguint(a: u64, b: &num_bigint::BigUint) -> num_bigint::BigUint {
    let mut a_val = num_bigint::BigUint::from(a);
    let mut b_val = b.clone();
    while !b_val.is_zero() {
        let r = &a_val % &b_val;
        a_val = b_val;
        b_val = r;
    }
    a_val
}

fn compute_d_inv(d: u64, modulus_minus_one: &num_bigint::BigUint) -> [u64; 4] {
    use num_bigint::BigInt;
    use num_traits::{One, Signed, Zero};
    let d_big = num_bigint::BigUint::from(d);
    let mut t = BigInt::zero();
    let mut new_t = BigInt::one();
    let mut r = BigInt::from(modulus_minus_one.clone());
    let mut new_r = BigInt::from(d_big);
    while !new_r.is_zero() {
        let quotient = &r / &new_r;
        let next_t = &t - &quotient * &new_t;
        t = new_t;
        new_t = next_t;
        let next_r = &r - &quotient * &new_r;
        r = new_r;
        new_r = next_r;
    }
    if t.is_negative() {
        t += BigInt::from(modulus_minus_one.clone());
    }
    let inv: num_bigint::BigUint = t.try_into().expect("non-negative");
    let mut out = [0u64; 4];
    let limbs = inv.to_u64_digits();
    for (i, limb) in limbs.iter().enumerate().take(4) {
        out[i] = *limb;
    }
    out
}

fn read_field<F: PrimeField>(
    reader: &mut dyn XofReader,
    modulus: &num_bigint::BigUint,
    byte_len: usize,
) -> F {
    let bits = modulus.bits() as usize;
    let mod_bits = bits % 8;
    let mask = if mod_bits == 0 { 0xFF } else { (1u8 << mod_bits) - 1 };
    let mut buf = vec![0u8; byte_len];
    loop {
        reader.read(&mut buf);
        if mod_bits != 0 {
            let last = buf.len() - 1;
            buf[last] &= mask;
        }
        let val = num_bigint::BigUint::from_bytes_le(&buf);
        if &val < modulus {
            return F::from_biguint(&val);
        }
    }
}

lazy_static! {
    // -- 256-bit fields: ~512 bit state --
    pub static ref XHASH_BN254_2_PARAMS: Arc<XHashParams<Bn254>> =
        generate_xhash::<Bn254>(2, "XHash-BN254-2");
    pub static ref XHASH_BLS12_381_2_PARAMS: Arc<XHashParams<Bls12_381>> =
        generate_xhash::<Bls12_381>(2, "XHash-BLS12_381-2");

    // -- 256-bit fields: ~768 bit state --
    pub static ref XHASH_BN254_3_PARAMS: Arc<XHashParams<Bn254>> =
        generate_xhash::<Bn254>(3, "XHash-BN254-3");
    pub static ref XHASH_BLS12_381_3_PARAMS: Arc<XHashParams<Bls12_381>> =
        generate_xhash::<Bls12_381>(3, "XHash-BLS12_381-3");

    // -- 64-bit field --
    pub static ref XHASH_GOLDILOCKS_8_PARAMS: Arc<XHashParams<Goldilocks>> =
        generate_xhash::<Goldilocks>(8, "XHash-Goldilocks-8");
    pub static ref XHASH_GOLDILOCKS_12_PARAMS: Arc<XHashParams<Goldilocks>> =
        generate_xhash::<Goldilocks>(12, "XHash-Goldilocks-12");

    // -- 31-bit fields --
    pub static ref XHASH_MERSENNE31_16_PARAMS: Arc<XHashParams<Mersenne31>> =
        generate_xhash::<Mersenne31>(16, "XHash-Mersenne31-16");
    pub static ref XHASH_MERSENNE31_24_PARAMS: Arc<XHashParams<Mersenne31>> =
        generate_xhash::<Mersenne31>(24, "XHash-Mersenne31-24");
    pub static ref XHASH_BABYBEAR_16_PARAMS: Arc<XHashParams<BabyBear>> =
        generate_xhash::<BabyBear>(16, "XHash-BabyBear-16");
    pub static ref XHASH_BABYBEAR_24_PARAMS: Arc<XHashParams<BabyBear>> =
        generate_xhash::<BabyBear>(24, "XHash-BabyBear-24");
    pub static ref XHASH_KOALABEAR_16_PARAMS: Arc<XHashParams<KoalaBear>> =
        generate_xhash::<KoalaBear>(16, "XHash-KoalaBear-16");
    pub static ref XHASH_KOALABEAR_24_PARAMS: Arc<XHashParams<KoalaBear>> =
        generate_xhash::<KoalaBear>(24, "XHash-KoalaBear-24");
}
