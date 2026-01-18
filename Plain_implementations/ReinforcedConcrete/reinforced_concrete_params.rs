use crate::fields::PrimeFieldExt;
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake128,
};

const RC_T: usize = 3;

#[derive(Clone, Debug)]
pub struct ReinforcedConcreteParams<F: PrimeFieldExt> {
    pub(crate) round_constants: Vec<[F; RC_T]>,
    pub(crate) alpha: [F; 2],
    pub(crate) beta: [F; 2],
    pub(crate) si: Vec<u16>,
    pub(crate) p_prime: u16,
    pub(crate) sbox: Vec<u16>,
    pub(crate) d: u64,
    pub(crate) modulus: BigUint,
}

impl<F: PrimeFieldExt> ReinforcedConcreteParams<F> {
    pub const PRE_ROUNDS: usize = 3;
    pub const POST_ROUNDS: usize = 3;
    pub const TOTAL_ROUNDS: usize = Self::PRE_ROUNDS + Self::POST_ROUNDS + 1;
    pub const T: usize = RC_T;
    pub const INIT_SHAKE: &'static str = "ReinforcedConcrete";

    pub fn new_fixed(d: u64, si: &[u16], sbox: &[u16], alpha_beta: [u64; 4]) -> Self {
        let modulus = F::modulus();
        let alpha = [F::from_u64(alpha_beta[0]), F::from_u64(alpha_beta[1])];
        let beta = [F::from_u64(alpha_beta[2]), F::from_u64(alpha_beta[3])];
        let round_constants = Self::instantiate_rc(&modulus);
        let p_prime = sbox.len() as u16;

        ReinforcedConcreteParams {
            round_constants,
            alpha,
            beta,
            si: si.to_vec(),
            p_prime,
            sbox: sbox.to_vec(),
            d,
            modulus,
        }
    }

    pub fn new_auto() -> Self {
        let modulus = F::modulus();
        let d = Self::select_d(&modulus);
        let alpha_beta = Self::select_alpha_beta(&modulus);
        let alpha = [F::from_u64(alpha_beta[0]), F::from_u64(alpha_beta[1])];
        let beta = [F::from_u64(alpha_beta[2]), F::from_u64(alpha_beta[3])];

        let si = Self::choose_uniform_radix(&modulus);
        let v = Self::decompose_biguint(&(modulus.clone() - BigUint::one()), &si);
        let min_v = *v.iter().min().expect("non-empty decomposition");
        let p_prime = Self::prev_prime(min_v).expect("valid prime for S-box");
        let sbox = Self::generate_sbox(p_prime);
        let round_constants = Self::instantiate_rc(&modulus);

        ReinforcedConcreteParams {
            round_constants,
            alpha,
            beta,
            si,
            p_prime,
            sbox,
            d,
            modulus,
        }
    }

    pub fn get_t(&self) -> usize {
        Self::T
    }

    pub fn get_rounds(&self) -> usize {
        Self::TOTAL_ROUNDS
    }

    fn instantiate_rc(modulus: &BigUint) -> Vec<[F; RC_T]> {
        let mut shake = Self::init_shake(modulus);
        (0..=Self::TOTAL_ROUNDS)
            .map(|_| {
                let mut rc: [F; RC_T] = core::array::from_fn(|_| F::zero());
                for el in rc.iter_mut() {
                    *el = Self::field_element_from_shake(&mut shake, modulus);
                }
                rc
            })
            .collect()
    }

    fn init_shake(modulus: &BigUint) -> impl XofReader {
        let mut shake = Shake128::default();
        shake.update(Self::INIT_SHAKE.as_bytes());
        for limb in modulus.to_u64_digits() {
            shake.update(&u64::to_le_bytes(limb));
        }
        shake.finalize_xof()
    }

    fn field_element_from_shake(reader: &mut dyn XofReader, modulus: &BigUint) -> F {
        let bytes = ((modulus.bits() + 7) / 8) as usize;
        let mut buf = vec![0u8; bytes];
        loop {
            reader.read(&mut buf);
            let candidate = BigUint::from_bytes_le(&buf);
            if &candidate < modulus {
                return F::from_biguint(&candidate);
            }
        }
    }

    fn select_d(modulus: &BigUint) -> u64 {
        let p_minus_one = modulus - BigUint::one();
        if Self::gcd_biguint(&BigUint::from(5u32), &p_minus_one) == BigUint::one() {
            return 5;
        }
        let mut d = 3u64;
        loop {
            if Self::gcd_biguint(&BigUint::from(d), &p_minus_one) == BigUint::one() {
                return d;
            }
            d += 2;
        }
    }

    fn select_alpha_beta(modulus: &BigUint) -> [u64; 4] {
        let bls_mod = BigUint::parse_bytes(
            b"73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001",
            16,
        )
        .expect("valid BLS12-381 modulus");
        let bn_mod = BigUint::parse_bytes(
            b"30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001",
            16,
        )
        .expect("valid BN254 modulus");
        if modulus == &bls_mod || modulus == &bn_mod {
            return [1, 3, 2, 4];
        }

        let mut found = Vec::with_capacity(2);
        for alpha in 1u64..64 {
            for beta in 1u64..64 {
                if !Self::is_quadratic_residue(alpha, beta, modulus) {
                    found.push((alpha, beta));
                    if found.len() == 2 {
                        return [found[0].0, found[1].0, found[0].1, found[1].1];
                    }
                }
            }
        }

        panic!("could not find suitable alpha/beta parameters");
    }

    fn is_quadratic_residue(alpha: u64, beta: u64, modulus: &BigUint) -> bool {
        let alpha_sq = BigUint::from(alpha) * BigUint::from(alpha);
        let four_beta = BigUint::from(4u32) * BigUint::from(beta);
        let mut value = if alpha_sq >= four_beta {
            alpha_sq - four_beta
        } else {
            modulus - (four_beta - alpha_sq) % modulus
        };
        value %= modulus;
        if value.is_zero() {
            return true;
        }
        let exp = (modulus - BigUint::one()) >> 1;
        value.modpow(&exp, modulus) == BigUint::one()
    }

    fn choose_uniform_radix(modulus: &BigUint) -> Vec<u16> {
        let p = modulus
            .to_u64()
            .expect("auto parameters only support <= 64-bit fields");
        let mut best = None;
        for n in 2usize..=8usize {
            for s in 256u64..=u16::MAX as u64 {
                if Self::pow_u128(s, n) <= p as u128 {
                    continue;
                }
                let (digits, ok) = Self::decompose_u64(p - 1, s, n);
                if !ok {
                    continue;
                }
                let min_v = digits.iter().copied().min().unwrap_or(0);
                if min_v < 2 {
                    continue;
                }
                if best
                    .map(|(best_min, _best_n, _best_s)| min_v > best_min)
                    .unwrap_or(true)
                {
                    best = Some((min_v, n, s));
                }
            }
        }

        let (_min_v, n, s) = best.expect("unable to select radix parameters");
        vec![s as u16; n]
    }

    fn decompose_u64(mut value: u64, base: u64, n: usize) -> (Vec<u16>, bool) {
        let mut digits = Vec::with_capacity(n);
        for _ in 0..n {
            digits.push((value % base) as u16);
            value /= base;
        }
        (digits, value == 0)
    }

    fn decompose_biguint(value: &BigUint, si: &[u16]) -> Vec<u16> {
        let mut x = value.clone();
        let mut res = vec![0u16; si.len()];
        for i in (1..si.len()).rev() {
            let s = BigUint::from(si[i] as u64);
            let digit = (&x % &s).to_u64().expect("digit fits in u64") as u16;
            res[i] = digit;
            x = (x - BigUint::from(digit as u64)) / s;
        }
        res[0] = x.to_u64().expect("digit fits in u64") as u16;
        res
    }

    fn prev_prime(value: u16) -> Option<u16> {
        for candidate in (2..=value).rev() {
            if Self::is_prime(candidate) {
                return Some(candidate);
            }
        }
        None
    }

    fn is_prime(value: u16) -> bool {
        if value < 2 {
            return false;
        }
        if value == 2 || value == 3 {
            return true;
        }
        if value % 2 == 0 {
            return false;
        }
        let mut d = 3u16;
        while (d as u32) * (d as u32) <= value as u32 {
            if value % d == 0 {
                return false;
            }
            d += 2;
        }
        true
    }

    fn generate_sbox(p_prime: u16) -> Vec<u16> {
        let p_prime_u64 = p_prime as u64;
        let d = Self::select_mersenne_exponent(p_prime_u64);
        let r = 2 * Self::ceil_log_d(p_prime_u64, d);
        let constants = Self::sample_mimc_constants(p_prime_u64, d, r);

        (0..p_prime_u64)
            .map(|mut x| {
                for c in constants.iter() {
                    x = Self::pow_mod_u64(x + c, d, p_prime_u64);
                }
                x as u16
            })
            .collect()
    }

    fn select_mersenne_exponent(p_prime: u64) -> u64 {
        const MERSENNE_PRIMES: [u64; 8] = [
            3,
            7,
            31,
            127,
            8191,
            131071,
            524287,
            2147483647,
        ];
        let p_minus_one = p_prime - 1;
        for d in MERSENNE_PRIMES {
            if Self::gcd_u64(d, p_minus_one) == 1 {
                return d;
            }
        }
        panic!("no suitable Mersenne exponent found");
    }

    fn ceil_log_d(value: u64, base: u64) -> usize {
        let mut power = 1u64;
        let mut count = 0usize;
        while power < value {
            power = power.saturating_mul(base);
            count += 1;
        }
        count
    }

    fn sample_mimc_constants(p_prime: u64, d: u64, r: usize) -> Vec<u64> {
        let mut shake = Shake128::default();
        shake.update(b"ReinforcedConcreteSBox");
        shake.update(&u64::to_le_bytes(p_prime));
        shake.update(&u64::to_le_bytes(d));
        let mut reader = shake.finalize_xof();
        let mut out = Vec::with_capacity(r);
        let mut buf = [0u8; 2];
        while out.len() < r {
            reader.read(&mut buf);
            let candidate = u16::from_le_bytes(buf) as u64;
            if candidate < p_prime {
                out.push(candidate);
            }
        }
        out
    }

    fn pow_mod_u64(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
        let mut result = 1u64;
        base %= modulus;
        while exp > 0 {
            if exp & 1 == 1 {
                result = (result as u128 * base as u128 % modulus as u128) as u64;
            }
            exp >>= 1;
            if exp > 0 {
                base = (base as u128 * base as u128 % modulus as u128) as u64;
            }
        }
        result
    }

    fn pow_u128(base: u64, exp: usize) -> u128 {
        let mut result = 1u128;
        let mut base_val = base as u128;
        let mut e = exp as u64;
        while e > 0 {
            if e & 1 == 1 {
                result = result.saturating_mul(base_val);
            }
            e >>= 1;
            if e > 0 {
                base_val = base_val.saturating_mul(base_val);
            }
        }
        result
    }

    fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let tmp = a % b;
            a = b;
            b = tmp;
        }
        a
    }

    fn gcd_biguint(a: &BigUint, b: &BigUint) -> BigUint {
        let mut a = a.clone();
        let mut b = b.clone();
        while !b.is_zero() {
            let tmp = &a % &b;
            a = b;
            b = tmp;
        }
        a
    }
}
