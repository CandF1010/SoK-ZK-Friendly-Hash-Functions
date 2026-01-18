use super::tip4_params::{Tip4Field, Tip4Params, LOOKUP_TABLE, NUM_SPLIT_AND_LOOKUP};
use crate::fields::FieldElement;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Tip4<F: Tip4Field> {
    pub(crate) params: Arc<Tip4Params<F>>,
}

impl<F: Tip4Field> Tip4<F> {
    pub fn new(params: &Arc<Tip4Params<F>>) -> Self {
        Tip4 {
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
        for round in 0..self.params.rounds {
            self.sbox_layer(&mut state);
            state = self.matmul(&state, &self.params.mds);
            self.add_round_constants(&mut state, round);
        }
        state
    }

    fn add_round_constants(&self, state: &mut [F], round: usize) {
        for (el, rc) in state
            .iter_mut()
            .zip(self.params.round_constants[round].iter())
        {
            el.add_assign(rc);
        }
    }

    fn sbox_layer(&self, state: &mut [F]) {
        for i in 0..NUM_SPLIT_AND_LOOKUP {
            state[i] = self.split_and_lookup(&state[i]);
        }
        for i in NUM_SPLIT_AND_LOOKUP..self.params.t {
            state[i] = state[i].pow_u64(7);
        }
    }

    fn split_and_lookup(&self, element: &F) -> F {
        let mut monty = element.clone();
        monty.mul_assign(&self.params.r);
        let mut bytes = monty.to_u64().to_le_bytes();
        for b in bytes.iter_mut() {
            *b = LOOKUP_TABLE[*b as usize];
        }
        let mut out = F::from_u64(u64::from_le_bytes(bytes));
        out.mul_assign(&self.params.r_inv);
        out
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
}
