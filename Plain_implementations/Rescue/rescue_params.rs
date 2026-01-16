use crate::fields::{FieldElement, PrimeField};
use crate::utils::{modinv, pow_biguint};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::Shake256;

#[derive(Clone, Debug)]
pub struct RescueParams<F: FieldElement> {
    pub(crate) t: usize,
    pub(crate) alpha: u64,
    pub(crate) alphainv: BigUint,
    pub(crate) rounds: usize,
    pub(crate) mds: Vec<Vec<F>>,
    pub(crate) round_constants: Vec<F>,
}

impl<F: FieldElement> RescueParams<F> {
    pub fn new(
        t: usize,
        alpha: u64,
        alphainv: BigUint,
        rounds: usize,
        mds: Vec<Vec<F>>,
        round_constants: Vec<F>,
    ) -> Self {
        let expected = 2 * t * rounds;
        assert_eq!(round_constants.len(), expected);
        RescueParams {
            t,
            alpha,
            alphainv,
            rounds,
            mds,
            round_constants,
        }
    }
}

impl<F: PrimeField> RescueParams<F> {
    pub fn from_spec(t: usize, capacity: usize, security_level: usize) -> Self {
        assert!(capacity < t, "capacity must be less than state width");

        let modulus = F::modulus();
        let (alpha, alphainv) = compute_alphas(&modulus);
        let rounds = compute_rounds(t, capacity, security_level, alpha);
        let mds = generate_mds_matrix::<F>(t, &modulus);
        let round_constants = generate_round_constants::<F>(
            t,
            capacity,
            security_level,
            rounds,
            &modulus,
        );

        RescueParams::new(t, alpha, alphainv, rounds, mds, round_constants)
    }
}

fn compute_alphas(modulus: &BigUint) -> (u64, BigUint) {
    let p_minus_one = modulus - BigUint::one();
    let mut alpha = 3u64;
    loop {
        let a = BigUint::from(alpha);
        if gcd_biguint(&a, &p_minus_one) == BigUint::one() {
            let alphainv = modinv(&a, &p_minus_one);
            return (alpha, alphainv);
        }
        alpha += 1;
    }
}

fn compute_rounds(t: usize, capacity: usize, security_level: usize, alpha: u64) -> usize {
    let rate = t - capacity;
    let target = BigUint::one() << security_level;
    let mut l1 = None;

    for n in 1..25usize {
        let dcon = ((alpha - 1) as usize * t * (n - 1)) / 2 + 2;
        let v = t * (n - 1) + rate;
        let bin = binomial(v + dcon, v);
        let mut bin_sq = bin.clone();
        bin_sq *= &bin;
        if bin_sq > target {
            l1 = Some(n);
            break;
        }
    }

    let l1 = l1.expect("failed to derive Rescue-Prime round count");
    let base = if l1 < 5 { 5 } else { l1 };
    (3 * base + 1) / 2
}

fn generate_mds_matrix<F: PrimeField>(t: usize, modulus: &BigUint) -> Vec<Vec<F>> {
    let g = F::from_biguint(&F::generator());
    let mut matrix = vec![vec![F::zero(); 2 * t]; t];

    for i in 0..t {
        for j in 0..2 * t {
            let exp = (i * j) as u64;
            matrix[i][j] = g.pow_u64(exp);
        }
    }

    let modulus_minus_two = modulus - BigUint::from(2u32);
    rref_in_place::<F>(&mut matrix, &modulus_minus_two);

    let mut mds = vec![vec![F::zero(); t]; t];
    for i in 0..t {
        for j in 0..t {
            mds[j][i] = matrix[i][t + j].clone();
        }
    }
    mds
}

fn rref_in_place<F: PrimeField>(matrix: &mut [Vec<F>], modulus_minus_two: &BigUint) {
    let rows = matrix.len();
    if rows == 0 {
        return;
    }
    let cols = matrix[0].len();
    let mut lead = 0usize;

    for r in 0..rows {
        if lead >= cols {
            break;
        }
        let mut i = r;
        while i < rows && matrix[i][lead] == F::zero() {
            i += 1;
        }
        if i == rows {
            lead += 1;
            if lead >= cols {
                break;
            }
            continue;
        }

        if i != r {
            matrix.swap(i, r);
        }

        let inv = pow_biguint(&matrix[r][lead], modulus_minus_two);
        for c in 0..cols {
            matrix[r][c].mul_assign(&inv);
        }

        for row in 0..rows {
            if row == r {
                continue;
            }
            let factor = matrix[row][lead].clone();
            if factor == F::zero() {
                continue;
            }
            for c in 0..cols {
                let mut tmp = matrix[r][c].clone();
                tmp.mul_assign(&factor);
                matrix[row][c].sub_assign(&tmp);
            }
        }

        lead += 1;
    }
}

fn generate_round_constants<F: PrimeField>(
    t: usize,
    capacity: usize,
    security_level: usize,
    rounds: usize,
    modulus: &BigUint,
) -> Vec<F> {
    let bitlen = modulus.bits() as usize;
    let bytes_per_int = (bitlen + 7) / 8 + 1;
    let num_constants = 2 * t * rounds;
    let num_bytes = bytes_per_int * num_constants;

    let seed = format!(
        "Rescue-XLIX({},{},{},{})",
        modulus, t, capacity, security_level
    );

    let mut shake = Shake256::default();
    shake.update(seed.as_bytes());
    let mut reader = shake.finalize_xof();

    let mut bytes = vec![0u8; num_bytes];
    reader.read(&mut bytes);

    let mut constants = Vec::with_capacity(num_constants);
    for i in 0..num_constants {
        let start = i * bytes_per_int;
        let end = start + bytes_per_int;
        let chunk = &bytes[start..end];
        let integer = BigUint::from_bytes_le(chunk);
        let reduced = integer % modulus;
        constants.push(F::from_biguint(&reduced));
    }

    constants
}

fn gcd_biguint(a: &BigUint, b: &BigUint) -> BigUint {
    let mut x = a.clone();
    let mut y = b.clone();
    while !y.is_zero() {
        let r = &x % &y;
        x = y;
        y = r;
    }
    x
}

fn binomial(n: usize, k: usize) -> BigUint {
    if k == 0 || k == n {
        return BigUint::one();
    }
    let k = if k > n - k { n - k } else { k };
    let mut result = BigUint::one();
    for i in 0..k {
        result *= BigUint::from((n - i) as u64);
        result /= BigUint::from((i + 1) as u64);
    }
    result
}
