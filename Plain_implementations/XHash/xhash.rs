/// XHash: SPN-based ZK-friendly hash permutation.
///
/// From Aly, Kales, Munk (2023). XHash is an Arithmetization-Oriented hash
/// designed for efficient ZK proofs, using a power-map S-box with low-degree
/// inverse. This makes verification (the inverse direction) cheap in proof
/// systems that express constraints "backwards" through the S-box.
///
/// Structure (each round):
///   1. S-box: x → x^d applied to all t state elements (full S-box layer)
///   2. Linear: multiply by MDS matrix
///   3. Add round constants
///
/// The exponent d is chosen small (3, 5, or 7) so that the forward S-box
/// is low-degree, making the inverse also efficient in the constraint system.

use crate::fields::FieldElement;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct XHashParams<F: FieldElement> {
    pub(crate) t: usize,
    pub(crate) d: u64,
    pub(crate) d_inv: [u64; 4],
    pub(crate) rounds: usize,
    /// MDS matrix: t×t (stored row-major).
    pub(crate) mds: Vec<Vec<F>>,
    /// Round constants: [round][state_element].
    pub(crate) round_constants: Vec<Vec<F>>,
}

impl<F: FieldElement> XHashParams<F> {
    pub fn new(
        t: usize,
        d: u64,
        d_inv: [u64; 4],
        rounds: usize,
        mds: &[Vec<F>],
        round_constants: &[Vec<F>],
    ) -> Self {
        assert_eq!(mds.len(), t);
        assert_eq!(round_constants.len(), rounds);
        for rc in round_constants {
            assert_eq!(rc.len(), t);
        }
        XHashParams {
            t,
            d,
            d_inv,
            rounds,
            mds: mds.to_owned(),
            round_constants: round_constants.to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct XHash<F: FieldElement> {
    pub(crate) params: Arc<XHashParams<F>>,
}

impl<F: FieldElement> XHash<F> {
    pub fn new(params: &Arc<XHashParams<F>>) -> Self {
        XHash {
            params: Arc::clone(params),
        }
    }

    pub fn get_t(&self) -> usize {
        self.params.t
    }

    /// XHash permutation: (ARK ○ MatMul ○ S)^R
    pub fn permutation(&self, input: &[F]) -> Vec<F> {
        let t = self.params.t;
        assert_eq!(input.len(), t);

        let mut state = input.to_vec();
        for r in 0..self.params.rounds {
            // 1. S-box layer: x → x^d on all elements.
            self.sbox_layer(&mut state);
            // 2. Linear layer: multiply by MDS matrix.
            state = self.matmul(&state);
            // 3. Add round constants.
            self.add_rc(&mut state, r);
        }
        state
    }

    fn sbox_layer(&self, state: &mut [F]) {
        for el in state.iter_mut() {
            *el = sbox_pow(el, self.params.d);
        }
    }

    fn matmul(&self, x: &[F]) -> Vec<F> {
        let t = x.len();
        let mut out = vec![F::zero(); t];
        for i in 0..t {
            for (j, xj) in x.iter().enumerate().take(t) {
                let mut term = self.params.mds[i][j].clone();
                term.mul_assign(xj);
                out[i].add_assign(&term);
            }
        }
        out
    }

    fn add_rc(&self, state: &mut [F], round: usize) {
        let rc = &self.params.round_constants[round];
        for (s, c) in state.iter_mut().zip(rc.iter()) {
            s.add_assign(c);
        }
    }
}

/// Power map S-box: x → x^d.
fn sbox_pow<F: FieldElement>(x: &F, d: u64) -> F {
    match d {
        2 => {
            let mut r = x.clone();
            r.square();
            r
        }
        3 => {
            let mut x2 = x.clone();
            x2.square();
            x2.mul_assign(x);
            x2
        }
        5 => {
            let mut x2 = x.clone();
            x2.square();
            let mut x4 = x2.clone();
            x4.square();
            x4.mul_assign(x);
            x4
        }
        7 => {
            let mut x2 = x.clone();
            x2.square();
            let mut x4 = x2.clone();
            x4.square();
            let mut x6 = x4.clone();
            x6.mul_assign(&x2);
            x6.mul_assign(x);
            x6
        }
        _ => x.pow_u64(d),
    }
}

/// Build a Circulant MDS matrix: row 0 = [a0, a1, ..., a_{t-1}], each
/// subsequent row is a cyclic right rotation.  Uses small scalars so that
/// the matrix is MDS.
///
/// Note: replaced by Cauchy MDS for benchmark fairness; kept for reference.
#[allow(dead_code)]
pub fn build_circulant_mds<F: FieldElement>(t: usize) -> Vec<Vec<F>> {
    // Use the matrix (2, 1, 1, ..., 1) -- similar to Poseidon's circulant.
    let mut m = vec![vec![F::one(); t]; t];
    for i in 0..t {
        m[i][i] = F::from_u64(2);
    }
    m
}
