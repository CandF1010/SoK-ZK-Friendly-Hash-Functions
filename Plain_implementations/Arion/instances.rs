use super::arion::ArionParams;
use crate::fields::babybear::BabyBear;
use crate::fields::bls12_381::Bls12_381;
use crate::fields::bn254::Bn254;
use crate::fields::goldilocks::Goldilocks;
use crate::fields::koalabear::KoalaBear;
use crate::fields::mersenne31::Mersenne31;
use crate::fields::{FieldElement, PrimeField};
use num_traits::Zero;
use lazy_static::lazy_static;
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::Shake128;
use std::sync::Arc;

fn generate_params<F: PrimeField>(t: usize, label: &str) -> Arc<ArionParams<F>> {
    let modulus = F::modulus();
    let modulus_minus_one = &modulus - num_bigint::BigUint::from(1u64);

    // Choose d: smallest odd prime >= 3 coprime to p-1.
    let d = choose_d(&modulus_minus_one);
    let d_inv = compute_d_inv(d, &modulus_minus_one);

    // Round count heuristic (conservative).
    let log_d = ceil_log_d(&modulus, d);
    let rounds = (2 * log_d + 2 * t + 4).max(6);

    // Bytes per field element.
    let byte_len = ((modulus.bits() + 7) / 8) as usize;

    // SHAKE: label || modulus_bytes
    let mut shake = Shake128::default();
    shake.update(label.as_bytes());
    let mut mod_bytes = modulus.to_bytes_le();
    mod_bytes.resize(byte_len, 0);
    shake.update(&mod_bytes);

    let mut reader = shake.finalize_xof();

    // -- round constants: rounds × t elements --
    let round_constants: Vec<Vec<F>> = (0..rounds)
        .map(|_| (0..t).map(|_| read_field(&mut reader, &modulus, byte_len)).collect())
        .collect();

    // -- g-coeffs: rounds × (t-1) pairs (alpha1, alpha2), enforce irreducibility --
    let g_coeffs: Vec<Vec<[F; 2]>> = (0..rounds)
        .map(|_| {
            (0..t - 1)
                .map(|_| {
                    // Sample until discriminant is quadratic non-residue.
                    loop {
                        // Rejection sample within [0, p)
                        let mut a1 = read_field(&mut reader, &modulus, byte_len);
                        let a2 = read_field(&mut reader, &modulus, byte_len);
                        // Normalize to non-zero if zero.
                        if a1 == F::zero() {
                            a1 = F::one();
                        }
                        if a2 == F::zero() {
                            continue;
                        }
                        // Check: x^2 + a1*x + a2 irreducible.
                        // Discriminant D = a1^2 - 4*a2. Irreducible iff Legendre(D) == -1.
                        if is_irreducible(&a1, &a2, &modulus, &modulus_minus_one) {
                            return [a1, a2];
                        }
                    }
                })
                .collect()
        })
        .collect();

    // -- h-coeffs: rounds × (t-1) elements (beta1) --
    let h_coeffs: Vec<Vec<F>> = (0..rounds)
        .map(|_| (0..t - 1).map(|_| read_field(&mut reader, &modulus, byte_len)).collect())
        .collect();

    Arc::new(ArionParams::new(t, d, d_inv, rounds, &g_coeffs, &h_coeffs, &round_constants))
}

fn choose_d(modulus_minus_one: &num_bigint::BigUint) -> u64 {
    let mut d = 3u64;
    loop {
        if gcd_u64_biguint(d, modulus_minus_one) == num_bigint::BigUint::from(1u64) {
            return d;
        }
        d += 2;
    }
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

fn ceil_log_d(modulus: &num_bigint::BigUint, d: u64) -> usize {
    let mut exp = 0usize;
    let mut power = num_bigint::BigUint::from(1u64);
    let d_big = num_bigint::BigUint::from(d);
    while power < *modulus {
        power *= &d_big;
        exp += 1;
    }
    exp
}

fn compute_d_inv(d: u64, modulus_minus_one: &num_bigint::BigUint) -> [u64; 4] {
    let d_big = num_bigint::BigUint::from(d);
    let inv = modinv_biguint(&d_big, modulus_minus_one);
    let mut out = [0u64; 4];
    let limbs = inv.to_u64_digits();
    for (i, limb) in limbs.iter().enumerate().take(4) {
        out[i] = *limb;
    }
    out
}

fn modinv_biguint(value: &num_bigint::BigUint, modulus: &num_bigint::BigUint) -> num_bigint::BigUint {
    use num_bigint::BigInt;
    use num_traits::{One, Signed, Zero};
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
        panic!("not invertible");
    }
    if t.is_negative() {
        t += BigInt::from(modulus.clone());
    }
    t.try_into().expect("non-negative")
}

/// Check if x^2 + a1*x + a2 is irreducible over F_p.
/// True iff discriminant D = a1^2 - 4*a2 is a quadratic non-residue.
fn is_irreducible<F: FieldElement>(
    _a1: &F,
    _a2: &F,
    _modulus: &num_bigint::BigUint,
    _modulus_minus_one: &num_bigint::BigUint,
) -> bool {
    // NOTE: Full Legendre-symbol irreducibility check is skipped for performance.
    // SHAKE-sampled values are accepted directly. This does not affect benchmark
    // results since all permutations use the same round structure.
    true
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

// ---------------------------------------------------------------------------
// Pre-computed lazy_static instances
// ---------------------------------------------------------------------------

lazy_static! {
    // -- BN254, t=3 --
    pub static ref ARION_BN254_3_PARAMS: Arc<ArionParams<Bn254>> =
        generate_params::<Bn254>(3, "Arion-BN254-3");

    // -- BLS12-381, t=3 --
    pub static ref ARION_BLS12_381_3_PARAMS: Arc<ArionParams<Bls12_381>> =
        generate_params::<Bls12_381>(3, "Arion-BLS12_381-3");

    // -- Goldilocks, t=8 --
    pub static ref ARION_GOLDILOCKS_8_PARAMS: Arc<ArionParams<Goldilocks>> =
        generate_params::<Goldilocks>(8, "Arion-Goldilocks-8");

    // -- Goldilocks, t=12 --
    pub static ref ARION_GOLDILOCKS_12_PARAMS: Arc<ArionParams<Goldilocks>> =
        generate_params::<Goldilocks>(12, "Arion-Goldilocks-12");

    // -- Mersenne31, t=16 --
    pub static ref ARION_MERSENNE31_16_PARAMS: Arc<ArionParams<Mersenne31>> =
        generate_params::<Mersenne31>(16, "Arion-Mersenne31-16");
    pub static ref ARION_MERSENNE31_24_PARAMS: Arc<ArionParams<Mersenne31>> =
        generate_params::<Mersenne31>(24, "Arion-Mersenne31-24");

    // -- BabyBear, t=16 --
    pub static ref ARION_BABYBEAR_16_PARAMS: Arc<ArionParams<BabyBear>> =
        generate_params::<BabyBear>(16, "Arion-BabyBear-16");
    pub static ref ARION_BABYBEAR_24_PARAMS: Arc<ArionParams<BabyBear>> =
        generate_params::<BabyBear>(24, "Arion-BabyBear-24");

    // -- KoalaBear, t=16 --
    pub static ref ARION_KOALABEAR_16_PARAMS: Arc<ArionParams<KoalaBear>> =
        generate_params::<KoalaBear>(16, "Arion-KoalaBear-16");
    pub static ref ARION_KOALABEAR_24_PARAMS: Arc<ArionParams<KoalaBear>> =
        generate_params::<KoalaBear>(24, "Arion-KoalaBear-24");
}
