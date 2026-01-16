use crate::fields::{FieldElement, PrimeField};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::Shake128;

#[derive(Clone, Debug)]
pub struct GmimcErfParams<F: FieldElement> {
    pub(crate) t: usize,
    pub(crate) d: u64,
    pub(crate) rounds: usize,
    pub(crate) round_constants: Vec<F>,
}

impl<F: PrimeField> GmimcErfParams<F> {
    pub const INIT_SHAKE: &'static str = "GMiMC";

    pub fn from_spec(t: usize) -> Self {
        let modulus = F::modulus();
        let d = choose_d(&modulus);
        let rounds = rounds_for_params(t, d, &modulus);
        let round_constants = generate_round_constants::<F>(rounds, &modulus);

        GmimcErfParams {
            t,
            d,
            rounds,
            round_constants,
        }
    }
}

fn choose_d(modulus: &BigUint) -> u64 {
    let modulus_minus_one = modulus - BigUint::one();
    let mut d = 3u64;
    loop {
        if gcd_biguint(&BigUint::from(d), &modulus_minus_one) == BigUint::one() {
            return d;
        }
        d += 2;
    }
}

fn rounds_for_params(t: usize, d: u64, modulus: &BigUint) -> usize {
    let rounds_linear = 2 + 2 * (t + t * t);
    let log_d = ceil_log_d(modulus, d);
    let rounds_algebraic = 2 * log_d + 2 * t;
    std::cmp::max(rounds_linear, rounds_algebraic)
}

fn ceil_log_d(modulus: &BigUint, d: u64) -> usize {
    let mut exp = 0usize;
    let mut power = BigUint::one();
    let d_big = BigUint::from(d);
    while power < *modulus {
        power *= &d_big;
        exp += 1;
    }
    exp
}

fn generate_round_constants<F: PrimeField>(rounds: usize, modulus: &BigUint) -> Vec<F> {
    let mut shake = Shake128::default();
    shake.update(GmimcErfParams::<F>::INIT_SHAKE.as_bytes());

    let byte_len = ((modulus.bits() + 7) / 8) as usize;
    let mut mod_bytes = modulus.to_bytes_le();
    mod_bytes.resize(byte_len, 0);
    shake.update(&mod_bytes);

    let mut reader = shake.finalize_xof();
    (0..rounds)
        .map(|_| field_element_from_shake::<F>(&mut reader, modulus))
        .collect()
}

fn field_element_from_shake<F: PrimeField>(reader: &mut dyn XofReader, modulus: &BigUint) -> F {
    let bits = modulus.bits() as usize;
    let bytes = (bits + 7) / 8;
    let mod_bits = bits % 8;
    let mask = if mod_bits == 0 { 0xFF } else { (1u8 << mod_bits) - 1 };

    loop {
        let mut buf = vec![0u8; bytes];
        reader.read(&mut buf);
        if mod_bits != 0 {
            let last = buf.len() - 1;
            buf[last] &= mask;
        }
        let val = BigUint::from_bytes_le(&buf);
        if val < *modulus {
            return F::from_biguint(&val);
        }
    }
}

fn gcd_biguint(a: &BigUint, b: &BigUint) -> BigUint {
    let mut a_val = a.clone();
    let mut b_val = b.clone();
    while !b_val.is_zero() {
        let r = &a_val % &b_val;
        a_val = b_val;
        b_val = r;
    }
    a_val
}
