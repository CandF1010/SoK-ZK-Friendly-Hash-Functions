use crate::fields::FieldElement;
use std::sync::Arc;

/// Arion permutation parameters.
///
/// Based on "Arion: Arithmetization-Oriented Permutation and Hashing from
/// Generalized Triangular Dynamical Systems" (arXiv:2303.04639).
#[derive(Clone, Debug)]
pub struct ArionParams<F: FieldElement> {
    pub(crate) t: usize,
    /// High-degree exponent for the inverse S-box on the last state element.
    pub(crate) d: u64,
    /// d^{-1} mod (p-1) as little-endian u64 limbs.
    pub(crate) d_inv: [u64; 4],
    pub(crate) rounds: usize,
    /// GTDS g-coefficients: [round][non_last_idx][0] = alpha1, [1] = alpha2.
    /// g(s) = s^2 + alpha1 * s + alpha2 must be irreducible over F_p.
    pub(crate) g_coeffs: Vec<Vec<[F; 2]>>,
    /// GTDS h-coefficients: [round][non_last_idx] = beta1.
    /// h(s) = s^2 + beta1 * s.
    pub(crate) h_coeffs: Vec<Vec<F>>,
    /// Affine round constants applied after circulant matrix: [round][elem].
    pub(crate) round_constants: Vec<Vec<F>>,
}

impl<F: FieldElement> ArionParams<F> {
    pub fn new(
        t: usize,
        d: u64,
        d_inv: [u64; 4],
        rounds: usize,
        g_coeffs: &[Vec<[F; 2]>],
        h_coeffs: &[Vec<F>],
        round_constants: &[Vec<F>],
    ) -> Self {
        assert!(t >= 3);
        assert_eq!(g_coeffs.len(), rounds);
        assert_eq!(h_coeffs.len(), rounds);
        assert_eq!(round_constants.len(), rounds);
        for r in 0..rounds {
            assert_eq!(g_coeffs[r].len(), t - 1);
            assert_eq!(h_coeffs[r].len(), t - 1);
            assert_eq!(round_constants[r].len(), t);
        }
        ArionParams {
            t,
            d,
            d_inv,
            rounds,
            g_coeffs: g_coeffs.to_owned(),
            h_coeffs: h_coeffs.to_owned(),
            round_constants: round_constants.to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Arion<F: FieldElement> {
    pub(crate) params: Arc<ArionParams<F>>,
}

impl<F: FieldElement> Arion<F> {
    pub fn new(params: &Arc<ArionParams<F>>) -> Self {
        Arion {
            params: Arc::clone(params),
        }
    }

    pub fn get_t(&self) -> usize {
        self.params.t
    }

    /// Arion permutation: A \circ (L \circ GTDS)^R on input.
    pub fn permutation(&self, input: &[F]) -> Vec<F> {
        let t = self.params.t;
        assert_eq!(input.len(), t);

        // Initial affine layer: apply circulant matrix.
        let mut state = self.apply_circulant(input);

        // Main rounds: GTDS, then affine (circulant + constants).
        for r in 0..self.params.rounds {
            self.gtds(&mut state, r);
            state = self.affine_layer(&state, r);
        }

        state
    }

    // ------------------------------------------------------------------
    // GTDS: Generalized Triangular Dynamical System
    // ------------------------------------------------------------------
    fn gtds(&self, state: &mut [F], round: usize) {
        let t = self.params.t;
        let mut f = state.to_vec();

        // Last element: high-degree inverse S-box.
        f[t - 1] = state[t - 1].pow_words_le(&self.params.d_inv);

        // Accumulate sigma from the top down.
        let mut sigma = state[t - 1].clone();
        sigma.add_assign(&f[t - 1]);

        for k in (0..t - 1).rev() {
            // Quintic S-box: x -> x^5.
            let x5 = quintic(&state[k]);

            // sigma^2
            let mut sigma_sq = sigma.clone();
            sigma_sq.square();

            // g(sigma) = sigma^2 + alpha1 * sigma + alpha2
            let g_coeff = &self.params.g_coeffs[round][k];
            let mut g = sigma_sq.clone();
            let mut alpha1_s = sigma.clone();
            alpha1_s.mul_assign(&g_coeff[0]);
            g.add_assign(&alpha1_s);
            g.add_assign(&g_coeff[1]);

            // h(sigma) = sigma^2 + beta1 * sigma
            let h_coeff = &self.params.h_coeffs[round][k];
            let mut h = sigma_sq;
            let mut beta1_s = sigma.clone();
            beta1_s.mul_assign(h_coeff);
            h.add_assign(&beta1_s);

            // f[k] = x^5 * g(sigma) + h(sigma)
            f[k] = x5;
            f[k].mul_assign(&g);
            f[k].add_assign(&h);

            // sigma += f[k] + state[k]
            sigma.add_assign(&f[k]);
            sigma.add_assign(&state[k]);
        }

        for (s, val) in state.iter_mut().zip(f.iter()) {
            *s = val.clone();
        }
    }

    // ------------------------------------------------------------------
    // Circulant matrix: first row = (0, 1, 2, ..., t-1), cyclic shifts.
    // ------------------------------------------------------------------
    fn apply_circulant(&self, x: &[F]) -> Vec<F> {
        let t = x.len();
        let sum = sum_all(x);
        let mut out = vec![F::zero(); t];
        for i in 0..t {
            let mut acc = sum.clone();
            for (j, xj) in x.iter().enumerate() {
                let coeff = circulant_elem(t, i, j);
                let mut term = xj.clone();
                term.mul_assign(&coeff);
                acc.add_assign(&term);
            }
            out[i] = acc;
        }
        out
    }

    fn affine_layer(&self, x: &[F], round: usize) -> Vec<F> {
        let t = x.len();
        let sum = sum_all(x);
        let rc = &self.params.round_constants[round];
        let mut out = vec![F::zero(); t];
        for i in 0..t {
            let mut acc = sum.clone();
            // Circulant sum part
            for (j, xj) in x.iter().enumerate() {
                let coeff = circulant_elem(t, i, j);
                let mut term = xj.clone();
                term.mul_assign(&coeff);
                acc.add_assign(&term);
            }
            // Add round constant
            acc.add_assign(&rc[i]);
            out[i] = acc;
        }
        out
    }
}

/// Circulant matrix element C[i][j] for a t×t matrix with first row (0, 1, ..., t-1).
/// Row i is a cyclic right rotation of row 0 by i positions.
#[inline(always)]
fn circulant_elem<F: FieldElement>(t: usize, i: usize, j: usize) -> F {
    let val = ((j + t - i) % t) as u64;
    F::from_u64(val)
}

fn sum_all<F: FieldElement>(x: &[F]) -> F {
    let mut s = F::zero();
    for xi in x {
        s.add_assign(xi);
    }
    s
}

/// S-box: x → x^5.
#[inline(always)]
fn quintic<F: FieldElement>(x: &F) -> F {
    let mut x2 = x.clone();
    x2.square(); // x²
    let mut x4 = x2.clone();
    x4.square(); // x⁴
    x4.mul_assign(x); // x⁵
    x4
}
