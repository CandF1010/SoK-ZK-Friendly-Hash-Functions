/// Vision-mark32: Flystel-based ZK-friendly hash permutation.
///
/// From the "Marvelous" family [AAB+20], Vision uses a flystel non-linear
/// layer applied to pairs of state elements, combined with an MDS linear
/// layer.  Vision-mark32 is the 32-element variant, targeting small fields.
///
/// The original Vision operates over binary tower fields (F_{2^n}) where
/// inversion is cheap.  Adapted here to prime fields using a small power-map
/// exponent for the S-box (alpha >= 3, gcd(alpha, p-1) = 1).
///
/// Each half-round:
///   1. Flystel (inverse) on pairs: (x, y) → (y + x^alpha, x)
///   2. MDS matrix
///   3. Round constants
/// A full round = two half-rounds.

use crate::fields::FieldElement;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct VisionMark32Params<F: FieldElement> {
    /// State size = 2 * num_pairs.
    pub(crate) t: usize,
    pub(crate) num_pairs: usize,
    /// S-box exponent (alpha).
    pub(crate) alpha: u64,
    pub(crate) alpha_inv: [u64; 4],
    /// Half-rounds = 2 * full_rounds.
    pub(crate) half_rounds: usize,
    /// MDS matrix: t×t.
    pub(crate) mds: Vec<Vec<F>>,
    /// Round constants: [half_round][elem].
    pub(crate) round_constants: Vec<Vec<F>>,
}

impl<F: FieldElement> VisionMark32Params<F> {
    pub fn new(
        t: usize,
        alpha: u64,
        alpha_inv: [u64; 4],
        half_rounds: usize,
        mds: &[Vec<F>],
        round_constants: &[Vec<F>],
    ) -> Self {
        assert_eq!(t % 2, 0);
        assert_eq!(mds.len(), t);
        assert_eq!(round_constants.len(), half_rounds);
        for rc in round_constants {
            assert_eq!(rc.len(), t);
        }
        VisionMark32Params {
            t,
            num_pairs: t / 2,
            alpha,
            alpha_inv,
            half_rounds,
            mds: mds.to_owned(),
            round_constants: round_constants.to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VisionMark32<F: FieldElement> {
    pub(crate) params: Arc<VisionMark32Params<F>>,
}

impl<F: FieldElement> VisionMark32<F> {
    pub fn new(params: &Arc<VisionMark32Params<F>>) -> Self {
        VisionMark32 {
            params: Arc::clone(params),
        }
    }

    pub fn get_t(&self) -> usize {
        self.params.t
    }

    /// Vision-mark32 permutation: (ARK ○ MatMul ○ FlystelInv)^R
    pub fn permutation(&self, input: &[F]) -> Vec<F> {
        let t = self.params.t;
        assert_eq!(input.len(), t);

        let mut state = input.to_vec();
        for r in 0..self.params.half_rounds {
            self.flystel_inv(&mut state);
            state = self.matmul(&state);
            self.add_rc(&mut state, r);
        }
        state
    }

    /// Inverse flystel on each pair: (x, y) → (y + x^alpha, x)
    fn flystel_inv(&self, state: &mut [F]) {
        let pairs = self.params.num_pairs;
        // Temporary copy of x values.
        let mut old_x = Vec::with_capacity(pairs);
        for i in 0..pairs {
            old_x.push(state[2 * i].clone());
        }
        for i in 0..pairs {
            // x^alpha
            let mut x_alpha = old_x[i].clone();
            x_alpha = sbox_pow_vis(&x_alpha, self.params.alpha);
            // new_x = y + x^alpha
            let mut new_x = state[2 * i + 1].clone();
            new_x.add_assign(&x_alpha);
            // new_y = old_x
            state[2 * i] = new_x;
            state[2 * i + 1] = old_x[i].clone();
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

/// Power map S-box for Vision: x → x^alpha.
fn sbox_pow_vis<F: FieldElement>(x: &F, alpha: u64) -> F {
    match alpha {
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
        _ => x.pow_u64(alpha),
    }
}

/// Build an MDS matrix via a simple Circulant (2, 1, 1, ..., 1).
///
/// Note: replaced by Cauchy MDS for benchmark fairness; kept for reference.
#[allow(dead_code)]
pub fn build_circulant_mds_vis<F: FieldElement>(t: usize) -> Vec<Vec<F>> {
    let mut m = vec![vec![F::one(); t]; t];
    for i in 0..t {
        m[i][i] = F::from_u64(2);
    }
    m
}
