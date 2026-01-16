use crate::fields::{FieldElement, PrimeField};
use crate::utils::{modinv, pow_mod_u64};
use num_bigint::BigUint;
use num_traits::{One, Zero};

const PI_0_STR: &str = "1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";
const PI_1_STR: &str = "8214808651328230664709384460955058223172535940812848111745028410270193852110555964462294895493038196";

#[derive(Clone, Debug)]
pub enum AnemoiMds<F: FieldElement> {
    Identity,
    M2 { b: F },
    M3 { b: F },
    M4 { b: F },
    Matrix { mat: Vec<Vec<F>> },
}

impl<F: FieldElement> AnemoiMds<F> {
    pub fn apply(&self, input: &mut [F]) {
        match self {
            AnemoiMds::Identity => {}
            AnemoiMds::M2 { b } => {
                let mut x0 = input[0].clone();
                let mut b_x1 = b.clone();
                b_x1.mul_assign(&input[1]);
                x0.add_assign(&b_x1);
                let mut x1 = input[1].clone();
                let mut b_x0 = b.clone();
                b_x0.mul_assign(&x0);
                x1.add_assign(&b_x0);
                input[0] = x0;
                input[1] = x1;
            }
            AnemoiMds::M3 { b } => {
                let mut t = input[0].clone();
                let mut b_x2 = b.clone();
                b_x2.mul_assign(&input[2]);
                t.add_assign(&b_x2);
                let mut x2 = input[2].clone();
                x2.add_assign(&input[1]);
                let mut b_x0 = b.clone();
                b_x0.mul_assign(&input[0]);
                x2.add_assign(&b_x0);
                let mut x0 = t.clone();
                x0.add_assign(&x2);
                let mut x1 = input[1].clone();
                x1.add_assign(&t);
                input[0] = x0;
                input[1] = x1;
                input[2] = x2;
            }
            AnemoiMds::M4 { b } => {
                let mut x0 = input[0].clone();
                let mut x1 = input[1].clone();
                let mut x2 = input[2].clone();
                let mut x3 = input[3].clone();

                x0.add_assign(&x1);
                x2.add_assign(&x3);
                let mut b_x0 = b.clone();
                b_x0.mul_assign(&x0);
                x3.add_assign(&b_x0);
                x1.add_assign(&x2);
                let mut x1_mul = b.clone();
                x1_mul.mul_assign(&x1);
                x1 = x1_mul;
                x0.add_assign(&x1);
                let mut b_x3 = b.clone();
                b_x3.mul_assign(&x3);
                x2.add_assign(&b_x3);
                x1.add_assign(&x2);
                x3.add_assign(&x0);

                input[0] = x0;
                input[1] = x1;
                input[2] = x2;
                input[3] = x3;
            }
            AnemoiMds::Matrix { mat } => {
                let width = mat.len();
                let mut out = vec![F::zero(); width];
                for row in 0..width {
                    for (col, inp) in input.iter().enumerate() {
                        let mut tmp = mat[row][col].clone();
                        tmp.mul_assign(inp);
                        out[row].add_assign(&tmp);
                    }
                }
                input.clone_from_slice(&out);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct AnemoiParams<F: FieldElement> {
    pub(crate) n_cols: usize,
    pub(crate) width: usize,
    pub(crate) rounds: usize,
    pub(crate) alpha: u64,
    pub(crate) alpha_inv: BigUint,
    pub(crate) beta: F,
    pub(crate) delta: F,
    pub(crate) quad: u64,
    pub(crate) mds: AnemoiMds<F>,
    pub(crate) round_constants_c: Vec<Vec<F>>,
    pub(crate) round_constants_d: Vec<Vec<F>>,
}

impl<F: PrimeField> AnemoiParams<F> {
    pub fn from_spec(n_cols: usize, security_level: u64) -> Self {
        let modulus = F::modulus();
        let alpha = choose_alpha(&modulus);
        let alpha_inv = modinv(&BigUint::from(alpha), &(modulus.clone() - BigUint::one()));
        let generator = F::generator();
        let delta_big = modinv(&generator, &modulus);
        let beta = F::from_biguint(&generator);
        let delta = F::from_biguint(&delta_big);
        let rounds = get_n_rounds(security_level, n_cols, alpha);
        let (round_constants_c, round_constants_d) = generate_round_constants::<F>(
            n_cols,
            rounds,
            alpha,
            &generator,
            &delta_big,
            &modulus,
        );
        let mds = generate_mds::<F>(n_cols, &generator, &modulus);

        AnemoiParams {
            n_cols,
            width: n_cols * 2,
            rounds,
            alpha,
            alpha_inv,
            beta,
            delta,
            quad: 2,
            mds,
            round_constants_c,
            round_constants_d,
        }
    }
}

fn choose_alpha(modulus: &BigUint) -> u64 {
    let modulus_minus_one = modulus - BigUint::one();
    for alpha in [3u64, 5, 7, 9, 11] {
        if gcd_biguint(&BigUint::from(alpha), &modulus_minus_one) == BigUint::one() {
            return alpha;
        }
    }
    panic!("no supported alpha found for Anemoi")
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

fn get_n_rounds(security_level: u64, n_cols: usize, alpha: u64) -> usize {
    let kappa = match alpha {
        3 => 1,
        5 => 2,
        7 => 4,
        9 => 7,
        11 => 9,
        _ => panic!("unsupported alpha for Anemoi round selection"),
    };

    let mut rounds = 0usize;
    let mut complexity = BigUint::zero();
    let target = BigUint::one() << security_level;

    while complexity < target {
        rounds += 1;
        let n = 4 * n_cols * rounds + kappa;
        let k = 2 * n_cols * rounds;
        let binom = binomial_biguint(n, k);
        complexity = &binom * &binom;
    }

    rounds += 2;
    rounds += std::cmp::min(5, n_cols + 1);
    std::cmp::max(8, rounds)
}

fn binomial_biguint(n: usize, k: usize) -> BigUint {
    let k = std::cmp::min(k, n - k);
    let mut result = BigUint::one();
    for i in 0..k {
        result *= BigUint::from(n - i);
        result /= BigUint::from(i + 1usize);
    }
    result
}

fn generate_round_constants<F: PrimeField>(
    n_cols: usize,
    rounds: usize,
    alpha: u64,
    generator: &BigUint,
    delta: &BigUint,
    modulus: &BigUint,
) -> (Vec<Vec<F>>, Vec<Vec<F>>) {
    let pi0 = BigUint::parse_bytes(PI_0_STR.as_bytes(), 10)
        .expect("valid PI_0")
        % modulus;
    let pi1 = BigUint::parse_bytes(PI_1_STR.as_bytes(), 10)
        .expect("valid PI_1")
        % modulus;

    let mut pi1_pows = Vec::with_capacity(n_cols);
    let mut pi1_sq = Vec::with_capacity(n_cols);
    let mut pi1_pow = BigUint::one();
    for _ in 0..n_cols {
        pi1_sq.push((&pi1_pow * &pi1_pow) % modulus);
        pi1_pows.push(pi1_pow.clone());
        pi1_pow = (&pi1_pow * &pi1) % modulus;
    }

    let mut c = Vec::with_capacity(rounds);
    let mut d = Vec::with_capacity(rounds);

    let mut pi0_pow = BigUint::one();
    for _round in 0..rounds {
        let pi0_sq = (&pi0_pow * &pi0_pow) % modulus;
        let mut c_row = Vec::with_capacity(n_cols);
        let mut d_row = Vec::with_capacity(n_cols);
        for i in 0..n_cols {
            let sum = (&pi0_pow + &pi1_pows[i]) % modulus;
            let pow_alpha = pow_mod_u64(&sum, alpha, modulus);
            let c_val = (generator * &pi0_sq + &pow_alpha) % modulus;
            let d_val = ((generator * &pi1_sq[i] + &pow_alpha) + delta) % modulus;
            c_row.push(F::from_biguint(&c_val));
            d_row.push(F::from_biguint(&d_val));
        }
        c.push(c_row);
        d.push(d_row);
        pi0_pow = (&pi0_pow * &pi0) % modulus;
    }

    (c, d)
}

fn generate_mds<F: PrimeField>(
    n_cols: usize,
    generator: &BigUint,
    modulus: &BigUint,
) -> AnemoiMds<F> {
    if n_cols == 1 {
        return AnemoiMds::Identity;
    }
    if n_cols > 4 {
        panic!("anemoi mds generation only implemented for n_cols <= 4");
    }

    let mut b = BigUint::one();
    loop {
        b = (&b * generator) % modulus;
        let mat = build_mds_matrix(n_cols, &b, modulus);
        if is_mds(&mat, modulus) {
            let b_field = F::from_biguint(&b);
            return match n_cols {
                2 => AnemoiMds::M2 { b: b_field },
                3 => AnemoiMds::M3 { b: b_field },
                4 => AnemoiMds::M4 { b: b_field },
                _ => unreachable!(),
            };
        }
    }
}

fn build_mds_matrix(n_cols: usize, b: &BigUint, modulus: &BigUint) -> Vec<Vec<BigUint>> {
    let mut cols = Vec::with_capacity(n_cols);
    for i in 0..n_cols {
        let mut vec = vec![BigUint::zero(); n_cols];
        vec[i] = BigUint::one();
        let out = match n_cols {
            2 => m2_apply(&vec, b, modulus),
            3 => m3_apply(&vec, b, modulus),
            4 => m4_apply(&vec, b, modulus),
            _ => unreachable!(),
        };
        cols.push(out);
    }

    let mut mat = vec![vec![BigUint::zero(); n_cols]; n_cols];
    for col in 0..n_cols {
        for row in 0..n_cols {
            mat[row][col] = cols[col][row].clone();
        }
    }
    mat
}

fn m2_apply(input: &[BigUint], b: &BigUint, modulus: &BigUint) -> Vec<BigUint> {
    let mut x0 = input[0].clone();
    let b_x1 = mul_mod(b, &input[1], modulus);
    x0 = add_mod(&x0, &b_x1, modulus);
    let mut x1 = input[1].clone();
    let b_x0 = mul_mod(b, &x0, modulus);
    x1 = add_mod(&x1, &b_x0, modulus);
    vec![x0, x1]
}

fn m3_apply(input: &[BigUint], b: &BigUint, modulus: &BigUint) -> Vec<BigUint> {
    let mut t = input[0].clone();
    let b_x2 = mul_mod(b, &input[2], modulus);
    t = add_mod(&t, &b_x2, modulus);
    let mut x2 = add_mod(&input[2], &input[1], modulus);
    let b_x0 = mul_mod(b, &input[0], modulus);
    x2 = add_mod(&x2, &b_x0, modulus);
    let x0 = add_mod(&t, &x2, modulus);
    let x1 = add_mod(&input[1], &t, modulus);
    vec![x0, x1, x2]
}

fn m4_apply(input: &[BigUint], b: &BigUint, modulus: &BigUint) -> Vec<BigUint> {
    let mut x0 = add_mod(&input[0], &input[1], modulus);
    let mut x2 = add_mod(&input[2], &input[3], modulus);
    let b_x0 = mul_mod(b, &x0, modulus);
    let mut x3 = add_mod(&input[3], &b_x0, modulus);
    let mut x1 = add_mod(&input[1], &x2, modulus);
    x1 = mul_mod(b, &x1, modulus);
    x0 = add_mod(&x0, &x1, modulus);
    let b_x3 = mul_mod(b, &x3, modulus);
    x2 = add_mod(&x2, &b_x3, modulus);
    x1 = add_mod(&x1, &x2, modulus);
    x3 = add_mod(&x3, &x0, modulus);
    vec![x0, x1, x2, x3]
}

fn is_mds(matrix: &[Vec<BigUint>], modulus: &BigUint) -> bool {
    let n = matrix.len();
    if n == 0 || matrix.iter().any(|row| row.len() != n) {
        return false;
    }

    for row in matrix {
        for val in row {
            if val.is_zero() {
                return false;
            }
        }
    }

    for k in 2..=n {
        let rows = combinations(n, k);
        let cols = combinations(n, k);
        for row_idx in &rows {
            for col_idx in &cols {
                let mut sub = vec![vec![BigUint::zero(); k]; k];
                for (i, &r) in row_idx.iter().enumerate() {
                    for (j, &c) in col_idx.iter().enumerate() {
                        sub[i][j] = matrix[r][c].clone();
                    }
                }
                if det_mod(&sub, modulus).is_zero() {
                    return false;
                }
            }
        }
    }
    true
}

fn det_mod(matrix: &[Vec<BigUint>], modulus: &BigUint) -> BigUint {
    let n = matrix.len();
    if n == 0 {
        return BigUint::one();
    }

    let mut mat = matrix.to_vec();
    let mut det = BigUint::one();
    let mut swapped = false;

    for i in 0..n {
        let mut pivot = i;
        while pivot < n && mat[pivot][i].is_zero() {
            pivot += 1;
        }
        if pivot == n {
            return BigUint::zero();
        }
        if pivot != i {
            mat.swap(i, pivot);
            swapped = !swapped;
        }

        let pivot_val = mat[i][i].clone() % modulus;
        if pivot_val.is_zero() {
            return BigUint::zero();
        }
        det = (&det * &pivot_val) % modulus;
        let inv = modinv(&pivot_val, modulus);

        for j in i..n {
            mat[i][j] = mul_mod(&mat[i][j], &inv, modulus);
        }

        for r in (i + 1)..n {
            if mat[r][i].is_zero() {
                continue;
            }
            let factor = mat[r][i].clone();
            for c in i..n {
                let scaled = mul_mod(&mat[i][c], &factor, modulus);
                mat[r][c] = sub_mod(&mat[r][c], &scaled, modulus);
            }
        }
    }

    if swapped && !det.is_zero() {
        det = modulus - det;
    }
    det % modulus
}

fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    let mut current = Vec::with_capacity(k);
    combinations_inner(n, k, 0, &mut current, &mut out);
    out
}

fn combinations_inner(
    n: usize,
    k: usize,
    start: usize,
    current: &mut Vec<usize>,
    out: &mut Vec<Vec<usize>>,
) {
    if current.len() == k {
        out.push(current.clone());
        return;
    }
    let remaining = k - current.len();
    for i in start..=n - remaining {
        current.push(i);
        combinations_inner(n, k, i + 1, current, out);
        current.pop();
    }
}

fn add_mod(a: &BigUint, b: &BigUint, modulus: &BigUint) -> BigUint {
    let mut res = a + b;
    if res >= *modulus {
        res -= modulus;
    }
    res
}

fn sub_mod(a: &BigUint, b: &BigUint, modulus: &BigUint) -> BigUint {
    let mut res = if a >= b {
        a - b
    } else {
        (a + modulus) - b
    };
    if res >= *modulus {
        res -= modulus;
    }
    res
}

fn mul_mod(a: &BigUint, b: &BigUint, modulus: &BigUint) -> BigUint {
    (a * b) % modulus
}
