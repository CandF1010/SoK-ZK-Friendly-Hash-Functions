use crate::fields::{FieldElement, PrimeField};
use crate::utils::modinv;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::Shake256;

#[derive(Clone, Debug)]
pub struct GriffinParams<F: FieldElement> {
    pub(crate) round_constants: Vec<Vec<F>>,
    pub(crate) t: usize,
    pub(crate) d: u64,
    pub(crate) d_inv: BigUint,
    pub(crate) rounds: usize,
    pub(crate) alpha_beta: Vec<[F; 2]>,
    #[allow(dead_code)]
    pub(crate) mat: Vec<Vec<F>>,
}

impl<F: PrimeField> GriffinParams<F> {
    pub fn from_spec(t: usize, security_level: u64) -> Self {
        let modulus = F::modulus();
        let capacity = compute_capacity(security_level, &modulus);
        Self::from_spec_with_capacity(t, capacity, security_level)
    }

    pub fn from_spec_with_capacity(t: usize, capacity: usize, security_level: u64) -> Self {
        let modulus = F::modulus();
        let max_sec = ((modulus.bits() as f64) * (t as f64) / 3.0).floor() as u64;
        assert!(security_level <= max_sec.min(256));

        let d = choose_d(&modulus);
        let rounds = compute_rounds(t, d, security_level);
        build_params::<F>(t, d, rounds, capacity, security_level, &modulus)
    }

    pub fn new(t: usize, d: u64, rounds: usize, capacity: usize, security_level: u64) -> Self {
        let modulus = F::modulus();
        build_params::<F>(t, d, rounds, capacity, security_level, &modulus)
    }

    pub fn get_t(&self) -> usize {
        self.t
    }

    pub fn get_rounds(&self) -> usize {
        self.rounds
    }
}

fn build_params<F: PrimeField>(
    t: usize,
    d: u64,
    rounds: usize,
    capacity: usize,
    security_level: u64,
    modulus: &BigUint,
) -> GriffinParams<F> {
    assert!(t == 3 || t % 4 == 0);
    assert!(rounds >= 1);
    assert!(capacity < t);

    let d_inv = modinv(&BigUint::from(d), &(modulus.clone() - BigUint::one()));
    let (alpha_beta, round_constants) =
        instantiate_constants::<F>(t, rounds, capacity, security_level, modulus);
    let mat = instantiate_matrix::<F>(t);

    GriffinParams {
        round_constants,
        t,
        d,
        d_inv,
        rounds,
        alpha_beta,
        mat,
    }
}

fn compute_capacity(security_level: u64, modulus: &BigUint) -> usize {
    let log2_p = modulus.bits() as f64;
    ((2.0 * security_level as f64) / log2_p).ceil() as usize
}

fn choose_d(modulus: &BigUint) -> u64 {
    let p_minus_one = modulus - BigUint::one();
    let mut d = 3u64;
    loop {
        if gcd_biguint(&BigUint::from(d), &p_minus_one) == BigUint::one() {
            return d;
        }
        d += 1;
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

fn compute_rounds(t: usize, d: u64, security_level: u64) -> usize {
    let target = BigUint::one() << ((security_level / 2) as usize);
    let mut rgb = 1usize;

    for candidate in 1..25 {
        let left_n = BigUint::from(candidate * (d as usize + t) + 1);
        let left_k = 1 + t * candidate;
        let left = binomial_biguint(&left_n, left_k);

        let right_n = BigUint::from(d).pow(candidate as u32) + BigUint::from(1 + candidate);
        let right_k = 1 + candidate;
        let right = binomial_biguint(&right_n, right_k);

        let min_lr = if left < right { left } else { right };
        if min_lr >= target {
            rgb = candidate;
            break;
        }
    }

    let base = std::cmp::max(6usize, 1 + rgb);
    (6 * base + 4) / 5
}

fn binomial_biguint(n: &BigUint, k: usize) -> BigUint {
    if k == 0 {
        return BigUint::one();
    }

    let mut result = BigUint::one();
    for i in 0..k {
        let term = n - BigUint::from(i as u64);
        result *= &term;
        result /= BigUint::from((i + 1) as u64);
    }
    result
}

fn instantiate_constants<F: PrimeField>(
    t: usize,
    rounds: usize,
    capacity: usize,
    security_level: u64,
    modulus: &BigUint,
) -> (Vec<[F; 2]>, Vec<Vec<F>>) {
    let bytes_per_int = (modulus.bits() as usize + 7) / 8 + 1;
    let num_elems = t * (rounds - 1) + 2;
    let num_bytes = bytes_per_int * num_elems;

    let seed = format!(
        "Griffin({},{},{},{})",
        modulus, t, capacity, security_level
    );
    let mut shake = Shake256::default();
    shake.update(seed.as_bytes());
    let mut reader = shake.finalize_xof();

    let mut bytes = vec![0u8; num_bytes];
    reader.read(&mut bytes);

    let mut elements = Vec::with_capacity(num_elems);
    for i in 0..num_elems {
        let start = i * bytes_per_int;
        let end = start + bytes_per_int;
        let chunk = &bytes[start..end];
        let value = BigUint::from_bytes_le(chunk) % modulus;
        elements.push(F::from_biguint(&value));
    }

    let alpha2 = elements[0].clone();
    let beta2 = elements[1].clone();
    let mut alpha_beta = Vec::with_capacity(t.saturating_sub(2));
    alpha_beta.push([alpha2.clone(), beta2.clone()]);
    for i in 3..t {
        let mut alpha = alpha2.clone();
        alpha.mul_assign(&F::from_u64((i - 1) as u64));
        let mut beta = beta2.clone();
        beta.mul_assign(&F::from_u64(((i - 1) * (i - 1)) as u64));
        alpha_beta.push([alpha, beta]);
    }

    let round_flat = &elements[2..];
    let expected = t * (rounds - 1);
    assert_eq!(round_flat.len(), expected);
    let round_constants = round_flat.chunks(t).map(|chunk| chunk.to_vec()).collect();

    (alpha_beta, round_constants)
}

fn circ_mat<F: FieldElement>(row: &[F]) -> Vec<Vec<F>> {
    let t = row.len();
    let mut mat: Vec<Vec<F>> = Vec::with_capacity(t);
    let mut rot = row.to_owned();
    mat.push(rot.clone());
    for _ in 1..t {
        rot.rotate_right(1);
        mat.push(rot.clone());
    }
    mat
}

fn instantiate_matrix<F: FieldElement>(t: usize) -> Vec<Vec<F>> {
    if t == 3 {
        let row = vec![F::from_u64(2), F::from_u64(1), F::from_u64(1)];
        return circ_mat(&row);
    }

    let row1 = vec![
        F::from_u64(5),
        F::from_u64(7),
        F::from_u64(1),
        F::from_u64(3),
    ];
    let row2 = vec![
        F::from_u64(4),
        F::from_u64(6),
        F::from_u64(1),
        F::from_u64(1),
    ];
    let row3 = vec![
        F::from_u64(1),
        F::from_u64(3),
        F::from_u64(5),
        F::from_u64(7),
    ];
    let row4 = vec![
        F::from_u64(1),
        F::from_u64(1),
        F::from_u64(4),
        F::from_u64(6),
    ];
    let c_mat = vec![row1, row2, row3, row4];

    if t == 4 {
        return c_mat;
    }

    assert_eq!(t % 4, 0);
    let mut mat: Vec<Vec<F>> = vec![vec![F::zero(); t]; t];
    for row in 0..t {
        for col in 0..t {
            let row_mod = row % 4;
            let col_mod = col % 4;
            let mut val = c_mat[row_mod][col_mod].clone();
            if row / 4 == col / 4 {
                val.add_assign(&c_mat[row_mod][col_mod]);
            }
            mat[row][col] = val;
        }
    }
    mat
}
