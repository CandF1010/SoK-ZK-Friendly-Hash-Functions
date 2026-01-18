use super::polocolo_params::PolocoloParams;
use crate::fields::{FieldElement, PrimeField, PrimeFieldExt};
use crate::utils::pow_biguint;
use num_bigint::BigUint;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Polocolo<F: PrimeField + PrimeFieldExt> {
    pub(crate) params: Arc<PolocoloParams<F>>,
}

impl<F: PrimeField + PrimeFieldExt> Polocolo<F> {
    pub fn new(params: &Arc<PolocoloParams<F>>) -> Self {
        Polocolo {
            params: Arc::clone(params),
        }
    }

    pub fn get_t(&self) -> usize {
        self.params.t
    }

    pub fn get_m(&self) -> usize {
        self.params.m
    }

    pub fn get_annihilator(&self) -> &BigUint {
        &self.params.ann
    }

    pub fn permutation(&self, input: &[F]) -> Vec<F> {
        let t = self.params.t;
        assert_eq!(input.len(), t);

        let mut state = input.to_vec();
        for r in 0..self.params.rounds {
            state = self.affine(&state, r);
            state = self.sbox(&state);
        }
        self.affine(&state, self.params.rounds)
    }

    fn sbox(&self, input: &[F]) -> Vec<F> {
        input.iter().map(|el| self.sbox_elem(el)).collect()
    }

    fn sbox_elem(&self, el: &F) -> F {
        if *el == F::zero() {
            return F::zero();
        }
        let lut_in = pow_biguint(el, &self.params.ann);
        let lut_out = self
            .params
            .lut
            .get(&lut_in.to_biguint())
            .expect("Polocolo LUT entry")
            .clone();
        let mut inv = pow_biguint(el, &self.params.modulus_minus_two);
        inv.mul_assign(&lut_out);
        inv
    }

    fn affine(&self, input: &[F], round_idx: usize) -> Vec<F> {
        let mat_result = self.matmul(input, &self.params.mds);
        if round_idx < self.params.rounds {
            Self::add_rc(&mat_result, &self.params.rc[round_idx])
        } else {
            mat_result
        }
    }

    fn matmul(&self, input: &[F], mat: &[Vec<F>]) -> Vec<F> {
        let t = mat.len();
        debug_assert_eq!(t, input.len());
        let mut out = vec![F::zero(); t];
        for row in 0..t {
            for (col, inp) in input.iter().enumerate().take(t) {
                let mut tmp = mat[row][col].clone();
                tmp.mul_assign(inp);
                out[row].add_assign(&tmp);
            }
        }
        out
    }

    fn add_rc(input: &[F], round_constants: &[F]) -> Vec<F> {
        debug_assert!(input.len() == round_constants.len());
        input
            .iter()
            .zip(round_constants.iter())
            .map(|(a, b)| {
                let mut r = a.clone();
                r.add_assign(b);
                r
            })
            .collect()
    }
}
