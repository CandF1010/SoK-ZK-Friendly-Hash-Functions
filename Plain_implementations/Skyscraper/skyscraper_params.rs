use crate::fields::{FieldElement, PrimeField, PrimeFieldExt};
use crate::utils::modinv;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use sha2::{Digest, Sha256};

const ROUND_COUNT: usize = 18;
const CHUNK_BITS: usize = 8;
const RC_LABEL: &[u8] = b"Skyscraper-v2";

#[derive(Clone, Debug)]
pub struct ExtElem<F: FieldElement> {
    pub(crate) coeffs: Vec<F>,
}

impl<F: FieldElement> ExtElem<F> {
    pub fn zero(n: usize) -> Self {
        ExtElem {
            coeffs: vec![F::zero(); n],
        }
    }

    pub fn from_coeffs(coeffs: Vec<F>) -> Self {
        ExtElem { coeffs }
    }

    pub fn add_assign(&mut self, other: &Self) {
        for (a, b) in self.coeffs.iter_mut().zip(other.coeffs.iter()) {
            a.add_assign(b);
        }
    }

    pub fn sub_assign(&mut self, other: &Self) {
        for (a, b) in self.coeffs.iter_mut().zip(other.coeffs.iter()) {
            a.sub_assign(b);
        }
    }

    pub fn mul_scalar(&mut self, scalar: &F) {
        for coeff in self.coeffs.iter_mut() {
            coeff.mul_assign(scalar);
        }
    }

    pub fn mul_assign(&mut self, other: &Self, beta: &F) {
        let n = self.coeffs.len();
        assert_eq!(n, other.coeffs.len());
        if n == 1 {
            self.coeffs[0].mul_assign(&other.coeffs[0]);
            return;
        }

        let mut out = vec![F::zero(); n];
        for i in 0..n {
            for j in 0..n {
                let mut term = self.coeffs[i].clone();
                term.mul_assign(&other.coeffs[j]);
                let idx = i + j;
                if idx < n {
                    out[idx].add_assign(&term);
                } else {
                    let mut beta_term = beta.clone();
                    beta_term.mul_assign(&term);
                    out[idx - n].sub_assign(&beta_term);
                }
            }
        }
        self.coeffs = out;
    }

    pub fn square(&self, beta: &F) -> Self {
        let mut out = self.clone();
        let rhs = self.clone();
        out.mul_assign(&rhs, beta);
        out
    }
}

#[derive(Clone, Debug)]
pub struct SkyscraperBar {
    n: usize,
    s: usize,
    m: usize,
    rot: usize,
    modulus: BigUint,
}

impl SkyscraperBar {
    pub fn new(n: usize, modulus: &BigUint) -> Self {
        let bits = modulus.bits() as usize;
        let m = (bits + CHUNK_BITS - 1) / CHUNK_BITS;
        assert!(m >= 2 && m % 2 == 0, "Skyscraper requires even m");
        SkyscraperBar {
            n,
            s: CHUNK_BITS,
            m,
            rot: m / 2,
            modulus: modulus.clone(),
        }
    }

    pub fn apply<F: PrimeField + PrimeFieldExt>(&self, elem: &ExtElem<F>) -> ExtElem<F> {
        assert_eq!(elem.coeffs.len(), self.n);
        let mut chunks = Vec::with_capacity(self.n * self.m);
        for coeff in elem.coeffs.iter() {
            let z = coeff.to_biguint();
            let mut bytes = z.to_bytes_be();
            if bytes.len() > self.m {
                panic!("Skyscraper BAR: coefficient exceeds expected chunk count");
            }
            let mut padded = vec![0u8; self.m - bytes.len()];
            padded.append(&mut bytes);
            chunks.extend_from_slice(&padded);
        }

        chunks.rotate_left(self.rot);
        for byte in chunks.iter_mut() {
            *byte = bar_sbox(*byte);
        }

        let mut coeffs = Vec::with_capacity(self.n);
        for i in 0..self.n {
            let start = i * self.m;
            let end = start + self.m;
            let mut acc = BigUint::zero();
            for byte in chunks[start..end].iter() {
                acc <<= self.s;
                acc += BigUint::from(*byte);
            }
            if acc >= self.modulus {
                acc -= &self.modulus;
            }
            coeffs.push(F::from_biguint(&acc));
        }

        ExtElem::from_coeffs(coeffs)
    }
}

#[derive(Clone, Debug)]
pub struct SkyscraperParams<F: PrimeField> {
    pub(crate) n: usize,
    pub(crate) beta: u64,
    pub(crate) beta_f: F,
    pub(crate) rounds: usize,
    pub(crate) rcons: Vec<ExtElem<F>>,
    pub(crate) sigma_inv: F,
    pub(crate) montgomery: bool,
    pub(crate) bar: SkyscraperBar,
}

impl<F: PrimeField + PrimeFieldExt> SkyscraperParams<F> {
    pub fn new(n: usize, beta: u64) -> Self {
        let modulus = F::modulus();
        let bar = SkyscraperBar::new(n, &modulus);
        let rcons = generate_rcons::<F>(n, &modulus);
        let sigma_inv = compute_sigma_inv::<F>(&modulus);
        SkyscraperParams {
            n,
            beta,
            beta_f: F::from_u64(beta),
            rounds: ROUND_COUNT,
            rcons,
            sigma_inv,
            montgomery: true,
            bar,
        }
    }
}

fn compute_sigma_inv<F: PrimeField>(modulus: &BigUint) -> F {
    let bits = modulus.bits();
    let machine_bits = ((bits + 63) / 64) * 64;
    let sigma = (BigUint::one() << machine_bits) % modulus;
    let sigma_inv = modinv(&sigma, modulus);
    F::from_biguint(&sigma_inv)
}

fn generate_rcons<F: PrimeField>(n: usize, modulus: &BigUint) -> Vec<ExtElem<F>> {
    let mut rcons = Vec::with_capacity(ROUND_COUNT);
    rcons.push(ExtElem::zero(n));

    let mut label = [0u8; 28];
    label[..RC_LABEL.len()].copy_from_slice(RC_LABEL);

    for i in 0..(ROUND_COUNT - 2) {
        let mut coeffs = Vec::with_capacity(n);
        for j in 0..n {
            let idx = (i * n + j) as u32;
            let mut input = Vec::with_capacity(32);
            input.extend_from_slice(&idx.to_be_bytes());
            input.extend_from_slice(&label);
            let digest = Sha256::digest(&input);
            let value = BigUint::from_bytes_be(&digest) % modulus;
            coeffs.push(F::from_biguint(&value));
        }
        rcons.push(ExtElem::from_coeffs(coeffs));
    }

    rcons.push(ExtElem::zero(n));
    rcons
}

fn bar_sbox(v: u8) -> u8 {
    let t1 = (!v).rotate_left(1);
    let t2 = v.rotate_left(2);
    let t3 = v.rotate_left(3);
    let y = v ^ (t1 & t2 & t3);
    y.rotate_left(1)
}

pub fn is_square_round(round: usize) -> bool {
    !matches!(round, 6 | 7 | 10 | 11)
}
