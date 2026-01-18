use crate::fields::{FieldElement, PrimeField};
use crate::utils::pow_biguint;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::VecDeque;

const FIELD_ID: u8 = 1;
const SBOX_ID_NEPTUNE: u8 = 2;
const EXTERNAL_ROUNDS_FIRST: usize = 4;
const EXTERNAL_ROUNDS_LAST: usize = 2;
const EXTERNAL_ROUNDS_TOTAL: usize = 6;

#[derive(Clone, Debug)]
pub struct NeptuneParams<F: FieldElement> {
    pub(crate) t: usize,
    pub(crate) t_prime: usize,
    pub(crate) d: u64,
    pub(crate) rounds_e_first: usize,
    pub(crate) rounds_e_last: usize,
    #[allow(dead_code)]
    pub(crate) rounds_e: usize,
    pub(crate) rounds_i: usize,
    #[allow(dead_code)]
    pub(crate) rounds: usize,
    pub(crate) alpha: F,
    pub(crate) gamma: F,
    pub(crate) mds_even: Vec<Vec<F>>,
    pub(crate) mds_odd: Vec<Vec<F>>,
    pub(crate) mat_internal_diag_m_1: Vec<F>,
    #[allow(dead_code)]
    pub(crate) mat_internal: Vec<Vec<F>>,
    pub(crate) round_constants: Vec<Vec<F>>,
}

impl<F: PrimeField> NeptuneParams<F> {
    pub fn from_spec(t: usize, security_level: u64) -> Self {
        assert!(t >= 2 && t % 2 == 0);
        let t_prime = t / 2;

        let modulus = F::modulus();
        let d = choose_d(&modulus);
        let rounds_i = compute_internal_rounds(t, d, security_level, &modulus);
        let rounds_e = EXTERNAL_ROUNDS_TOTAL;
        let rounds = rounds_e + rounds_i;

        let (mds_even, mds_odd) = generate_mds_pair::<F>(t_prime, &modulus);
        let mat_internal_diag_m_1 = generate_internal_diag::<F>(t, &modulus);
        let mat_internal = build_internal_matrix(&mat_internal_diag_m_1);

        let round_constants = generate_round_constants::<F>(t, rounds_e, rounds_i, &modulus);

        NeptuneParams {
            t,
            t_prime,
            d,
            rounds_e_first: EXTERNAL_ROUNDS_FIRST,
            rounds_e_last: EXTERNAL_ROUNDS_LAST,
            rounds_e,
            rounds_i,
            rounds,
            alpha: F::one(),
            gamma: F::one(),
            mds_even,
            mds_odd,
            mat_internal_diag_m_1,
            mat_internal,
            round_constants,
        }
    }
}

fn choose_d(modulus: &BigUint) -> u64 {
    let p_minus_one = modulus - BigUint::one();
    for d in [3u64, 5, 7, 11] {
        if gcd_biguint(&BigUint::from(d), &p_minus_one) == BigUint::one() {
            return d;
        }
    }
    panic!("no valid d in [3,5,7,11] for this field");
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

fn compute_internal_rounds(
    t: usize,
    d: u64,
    security_level: u64,
    modulus: &BigUint,
) -> usize {
    let log2_p = modulus.bits() as f64;
    let log2_d = (d as f64).log2();
    let min_sec = (security_level as f64).min(log2_p);
    let log_d_2 = 1.0 / log2_d;
    let log_d_t = (t as f64).log2() / log2_d;

    let inner = log_d_2 * (min_sec - 6.0) + 3.0 + (t as f64) + log_d_t;
    (1.125 * inner.ceil()).ceil() as usize
}

fn generate_round_constants<F: PrimeField>(
    t: usize,
    rounds_e: usize,
    rounds_i: usize,
    modulus: &BigUint,
) -> Vec<Vec<F>> {
    let rounds = rounds_e + rounds_i;
    let n_bits = modulus.bits() as usize;
    let seed = grain_seed_bits(n_bits as u64, t as u64, rounds_e as u64, rounds_i as u64);
    let mut lfsr = GrainLfsr::new(seed);
    lfsr.warmup(160);

    let num_constants = rounds * t;
    let mut flat = Vec::with_capacity(num_constants);
    for _ in 0..num_constants {
        let mut candidate = lfsr.random_bits(n_bits);
        while candidate >= *modulus {
            candidate = lfsr.random_bits(n_bits);
        }
        flat.push(F::from_biguint(&candidate));
    }

    flat.chunks(t).map(|chunk| chunk.to_vec()).collect()
}

fn grain_seed_bits(n_bits: u64, t: u64, rounds_e: u64, rounds_i: u64) -> [u8; 80] {
    let mut bits = Vec::with_capacity(80);
    // FIELD=1 (GF(p)) and SBOX=2 (Neptune).
    append_bits(&mut bits, 2, FIELD_ID as u128);
    append_bits(&mut bits, 4, SBOX_ID_NEPTUNE as u128);
    push_bits(&mut bits, n_bits, 12);
    push_bits(&mut bits, t, 12);
    push_bits(&mut bits, rounds_e, 10);
    push_bits(&mut bits, rounds_i, 10);
    bits.extend(std::iter::repeat(1u8).take(30));
    bits.try_into().expect("neptune grain seed is 80 bits")
}

fn push_bits(bits: &mut Vec<u8>, value: u64, width: usize) {
    for i in (0..width).rev() {
        bits.push(((value >> i) & 1) as u8);
    }
}

fn append_bits(bits: &mut Vec<u8>, width: usize, value: u128) {
    for i in (0..width).rev() {
        bits.push(((value >> i) & 1) as u8);
    }
}

struct GrainLfsr {
    state: VecDeque<u8>,
}

impl GrainLfsr {
    fn new(seed_bits: [u8; 80]) -> Self {
        Self {
            state: seed_bits.into_iter().collect(),
        }
    }

    fn warmup(&mut self, steps: usize) {
        for _ in 0..steps {
            self.next_bit();
        }
    }

    fn next_bit(&mut self) -> u8 {
        let new_bit = self.state[62]
            ^ self.state[51]
            ^ self.state[38]
            ^ self.state[23]
            ^ self.state[13]
            ^ self.state[0];
        self.state.pop_front();
        self.state.push_back(new_bit);
        new_bit
    }

    fn next_shrunk_bit(&mut self) -> u8 {
        loop {
            let first = self.next_bit();
            let second = self.next_bit();
            if first == 1 {
                return second;
            }
        }
    }

    fn random_bits(&mut self, n_bits: usize) -> BigUint {
        let mut out = BigUint::zero();
        for _ in 0..n_bits {
            out <<= 1u8;
            if self.next_shrunk_bit() == 1 {
                out += BigUint::one();
            }
        }
        out
    }
}

fn generate_mds_pair<F: PrimeField>(t: usize, modulus: &BigUint) -> (Vec<Vec<F>>, Vec<Vec<F>>) {
    let mut offset = 0u64;
    loop {
        let m1 = generate_mds_cauchy::<F>(t, offset, modulus);
        let m2 = generate_mds_cauchy::<F>(t, offset + 2 * t as u64 + 1, modulus);
        if let (Some(m1), Some(m2)) = (m1, m2) {
            if mds_pair_ok::<F>(&m1, &m2, modulus) {
                return (m1, m2);
            }
        }
        offset += 4 * t as u64 + 1;
    }
}

fn mds_pair_ok<F: PrimeField>(
    m1: &[Vec<F>],
    m2: &[Vec<F>],
    modulus: &BigUint,
) -> bool {
    if m1.len() != m2.len() {
        return false;
    }
    let t = m1.len();
    for i in 0..t {
        for j in 0..t {
            if m1[i][j] == m2[i][j] {
                return false;
            }
        }
    }
    !is_scalar_multiple::<F>(m1, m2, modulus)
}

fn is_scalar_multiple<F: PrimeField>(
    m1: &[Vec<F>],
    m2: &[Vec<F>],
    modulus: &BigUint,
) -> bool {
    let t = m1.len();
    if t == 0 {
        return true;
    }
    let modulus_minus_two = modulus - BigUint::from(2u32);
    let inv = pow_biguint(&m2[0][0], &modulus_minus_two);
    let mut scalar = m1[0][0].clone();
    scalar.mul_assign(&inv);

    for i in 0..t {
        for j in 0..t {
            let mut tmp = m2[i][j].clone();
            tmp.mul_assign(&scalar);
            if tmp != m1[i][j] {
                return false;
            }
        }
    }
    true
}

fn generate_mds_cauchy<F: PrimeField>(
    t: usize,
    offset: u64,
    modulus: &BigUint,
) -> Option<Vec<Vec<F>>> {
    let xs: Vec<F> = (0..t)
        .map(|i| F::from_u64(offset + i as u64))
        .collect();
    let ys: Vec<F> = (0..t)
        .map(|i| F::from_u64(offset + t as u64 + i as u64))
        .collect();
    if has_duplicates(&xs) || has_duplicates(&ys) {
        return None;
    }

    let modulus_minus_two = modulus - BigUint::from(2u32);
    let mut mds = vec![vec![F::zero(); t]; t];
    for i in 0..t {
        for j in 0..t {
            let mut denom = xs[i].clone();
            denom.add_assign(&ys[j]);
            if denom == F::zero() {
                return None;
            }
            mds[i][j] = pow_biguint(&denom, &modulus_minus_two);
        }
    }
    Some(mds)
}

fn has_duplicates<F: FieldElement>(values: &[F]) -> bool {
    for i in 0..values.len() {
        for j in (i + 1)..values.len() {
            if values[i] == values[j] {
                return true;
            }
        }
    }
    false
}

fn generate_internal_diag<F: PrimeField>(t: usize, modulus: &BigUint) -> Vec<F> {
    let gen = F::from_biguint(&F::generator());
    let mut offset = 1u64;
    loop {
        let mut diag = Vec::with_capacity(t);
        let mut cur = gen.pow_u64(offset);
        for _ in 0..t {
            diag.push(cur.clone());
            cur.mul_assign(&gen);
        }

        let mut diag_m_1 = Vec::with_capacity(t);
        for val in diag {
            let mut out = val;
            out.sub_assign(&F::one());
            diag_m_1.push(out);
        }

        let mat = build_internal_matrix(&diag_m_1);
        if is_invertible::<F>(&mat, modulus) {
            return diag_m_1;
        }
        offset += 1;
    }
}

fn build_internal_matrix<F: FieldElement>(diag_m_1: &[F]) -> Vec<Vec<F>> {
    let t = diag_m_1.len();
    let mut mat = vec![vec![F::one(); t]; t];
    for i in 0..t {
        mat[i][i].add_assign(&diag_m_1[i]);
    }
    mat
}

fn is_invertible<F: PrimeField>(matrix: &[Vec<F>], modulus: &BigUint) -> bool {
    let n = matrix.len();
    if n == 0 || matrix.iter().any(|row| row.len() != n) {
        return false;
    }

    let modulus_minus_two = modulus - BigUint::from(2u32);
    let mut mat = matrix.to_vec();

    for i in 0..n {
        let mut pivot = i;
        while pivot < n && mat[pivot][i] == F::zero() {
            pivot += 1;
        }
        if pivot == n {
            return false;
        }
        if pivot != i {
            mat.swap(i, pivot);
        }

        let inv = pow_biguint(&mat[i][i], &modulus_minus_two);
        for j in i..n {
            mat[i][j].mul_assign(&inv);
        }

        for r in 0..n {
            if r == i {
                continue;
            }
            let factor = mat[r][i].clone();
            if factor == F::zero() {
                continue;
            }
            for c in i..n {
                let mut tmp = mat[i][c].clone();
                tmp.mul_assign(&factor);
                mat[r][c].sub_assign(&tmp);
            }
        }
    }

    true
}
