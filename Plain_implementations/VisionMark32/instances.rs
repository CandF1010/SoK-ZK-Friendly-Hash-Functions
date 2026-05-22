use super::vision_mark32::VisionMark32Params;
use crate::utils::build_cauchy_mds;
use crate::fields::babybear::BabyBear;
use crate::fields::bls12_381::Bls12_381;
use crate::fields::bn254::Bn254;
use crate::fields::goldilocks::Goldilocks;
use crate::fields::koalabear::KoalaBear;
use crate::fields::mersenne31::Mersenne31;
use crate::fields::{PrimeField};
use lazy_static::lazy_static;
use num_traits::Zero;
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::Shake128;
use std::sync::Arc;

fn generate_vision<F: PrimeField>(t: usize, label: &str) -> Arc<VisionMark32Params<F>> {
    assert_eq!(t % 2, 0);
    let modulus = F::modulus();
    let modulus_minus_one = &modulus - num_bigint::BigUint::from(1u64);

    // Choose alpha: smallest odd >= 3 coprime to p-1.
    let alpha = {
        let mut a = 3u64;
        loop {
            if gcd_u64_biguint(a, &modulus_minus_one) == num_bigint::BigUint::from(1u64) {
                break a;
            }
            a += 2;
        }
    };
    let alpha_inv = compute_d_inv(alpha, &modulus_minus_one);

    // Half-rounds = 2 * full_rounds.
    let log_alpha = {
        let mut exp = 0usize;
        let mut power = num_bigint::BigUint::from(1u64);
        let a_big = num_bigint::BigUint::from(alpha);
        while power < modulus {
            power *= &a_big;
            exp += 1;
        }
        exp
    };
    let full_rounds = (2 * log_alpha + t / 2 + 4).max(4);
    let half_rounds = 2 * full_rounds;

    let byte_len = ((modulus.bits() + 7) / 8) as usize;

    let mut shake = Shake128::default();
    shake.update(label.as_bytes());
    let mut mod_bytes = modulus.to_bytes_le();
    mod_bytes.resize(byte_len, 0);
    shake.update(&mod_bytes);
    let mut reader = shake.finalize_xof();

    // Cauchy MDS matrix.
    let mds = build_cauchy_mds::<F>(t);

    // Round constants: half_rounds × t.
    let round_constants: Vec<Vec<F>> = (0..half_rounds)
        .map(|_| (0..t).map(|_| read_field(&mut reader, &modulus, byte_len)).collect())
        .collect();

    Arc::new(VisionMark32Params::new(t, alpha, alpha_inv, half_rounds, &mds, &round_constants))
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
    // -- Standard sizes for fair comparison (~512 bit state) --
    // Note: Vision requires even t (flystel works on pairs).
    // For 256-bit fields, t=2 gives ~512 bits; t=3 is not supported.
    pub static ref VISION_BN254_2_PARAMS: Arc<VisionMark32Params<Bn254>> =
        generate_vision::<Bn254>(2, "Vision-BN254-2");
    pub static ref VISION_BLS12_381_2_PARAMS: Arc<VisionMark32Params<Bls12_381>> =
        generate_vision::<Bls12_381>(2, "Vision-BLS12_381-2");

    // -- 64-bit field (~512 and ~768 bit state) --
    pub static ref VISION_GOLDILOCKS_8_PARAMS: Arc<VisionMark32Params<Goldilocks>> =
        generate_vision::<Goldilocks>(8, "Vision-Goldilocks-8");
    pub static ref VISION_GOLDILOCKS_12_PARAMS: Arc<VisionMark32Params<Goldilocks>> =
        generate_vision::<Goldilocks>(12, "Vision-Goldilocks-12");

    // -- 31-bit fields: standard sizes --
    pub static ref VISION_MERSENNE31_16_PARAMS: Arc<VisionMark32Params<Mersenne31>> =
        generate_vision::<Mersenne31>(16, "Vision-Mersenne31-16");
    pub static ref VISION_MERSENNE31_24_PARAMS: Arc<VisionMark32Params<Mersenne31>> =
        generate_vision::<Mersenne31>(24, "Vision-Mersenne31-24");
    pub static ref VISION_BABYBEAR_16_PARAMS: Arc<VisionMark32Params<BabyBear>> =
        generate_vision::<BabyBear>(16, "Vision-BabyBear-16");
    pub static ref VISION_BABYBEAR_24_PARAMS: Arc<VisionMark32Params<BabyBear>> =
        generate_vision::<BabyBear>(24, "Vision-BabyBear-24");
    pub static ref VISION_KOALABEAR_16_PARAMS: Arc<VisionMark32Params<KoalaBear>> =
        generate_vision::<KoalaBear>(16, "Vision-KoalaBear-16");
    pub static ref VISION_KOALABEAR_24_PARAMS: Arc<VisionMark32Params<KoalaBear>> =
        generate_vision::<KoalaBear>(24, "Vision-KoalaBear-24");

    // -- Special: t=32 "mark32" size (only over 31-bit fields, for completeness) --
    pub static ref VISION_MARK32_MERSENNE31_PARAMS: Arc<VisionMark32Params<Mersenne31>> =
        generate_vision::<Mersenne31>(32, "VisionMark32-Mersenne31");
    pub static ref VISION_MARK32_BABYBEAR_PARAMS: Arc<VisionMark32Params<BabyBear>> =
        generate_vision::<BabyBear>(32, "VisionMark32-BabyBear");
    pub static ref VISION_MARK32_KOALABEAR_PARAMS: Arc<VisionMark32Params<KoalaBear>> =
        generate_vision::<KoalaBear>(32, "VisionMark32-KoalaBear");
}
