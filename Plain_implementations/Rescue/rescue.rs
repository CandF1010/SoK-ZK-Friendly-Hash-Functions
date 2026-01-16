use super::rescue_params::RescueParams;
use crate::fields::FieldElement;
use crate::utils::pow_biguint;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Rescue<F: FieldElement> {
    pub(crate) params: Arc<RescueParams<F>>,
}

impl<F: FieldElement> Rescue<F> {
    pub fn new(params: &Arc<RescueParams<F>>) -> Self {
        Rescue {
            params: Arc::clone(params),
        }
    }

    pub fn get_t(&self) -> usize {
        self.params.t
    }

    pub fn permutation(&self, input: &[F]) -> Vec<F> {
        let t = self.params.t;
        assert_eq!(input.len(), t);

        let mut state = input.to_vec();
        let rc = &self.params.round_constants;

        for r in 0..self.params.rounds {
            for i in 0..t {
                state[i] = self.sbox_alpha(&state[i]);
            }
            state = self.matmul(&state, &self.params.mds);
            self.add_constants(&mut state, rc, r * 2 * t);

            for i in 0..t {
                state[i] = self.sbox_alphainv(&state[i]);
            }
            state = self.matmul(&state, &self.params.mds);
            self.add_constants(&mut state, rc, r * 2 * t + t);
        }

        state
    }

    fn sbox_alpha(&self, input: &F) -> F {
        input.pow_u64(self.params.alpha)
    }

    fn sbox_alphainv(&self, input: &F) -> F {
        pow_biguint(input, &self.params.alphainv)
    }

    fn matmul(&self, input: &[F], mat: &[Vec<F>]) -> Vec<F> {
        let t = mat.len();
        debug_assert!(t == input.len());
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

    fn add_constants(&self, state: &mut [F], rc: &[F], offset: usize) {
        for (i, val) in state.iter_mut().enumerate() {
            val.add_assign(&rc[offset + i]);
        }
    }
}
