use super::gmimc_erf_params::GmimcErfParams;
use crate::fields::FieldElement;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct GmimcErf<F: FieldElement> {
    pub(crate) params: Arc<GmimcErfParams<F>>,
}

impl<F: FieldElement> GmimcErf<F> {
    pub fn new(params: &Arc<GmimcErfParams<F>>) -> Self {
        GmimcErf {
            params: Arc::clone(params),
        }
    }

    pub fn get_t(&self) -> usize {
        self.params.t
    }

    pub fn permutation(&self, input: &[F]) -> Vec<F> {
        let t = self.params.t;
        assert_eq!(input.len(), t);

        let mut current_state = input.to_vec();
        for r in 0..self.params.rounds - 1 {
            self.round(&mut current_state, r);
            current_state.rotate_right(1);
        }
        self.round(&mut current_state, self.params.rounds - 1);
        current_state
    }

    fn round(&self, state: &mut [F], round: usize) {
        let power = self.sbox(&state[0], round);
        for el in state.iter_mut().skip(1) {
            el.add_assign(&power);
        }
    }

    fn sbox(&self, state_0: &F, round: usize) -> F {
        let mut input = state_0.clone();
        input.add_assign(&self.params.round_constants[round]);

        let mut input2 = input.clone();
        input2.square();
        match self.params.d {
            3 => {
                let mut out = input2;
                out.mul_assign(&input);
                out
            }
            5 => {
                let mut out = input2;
                out.square();
                out.mul_assign(&input);
                out
            }
            7 => {
                let mut out = input2.clone();
                out.square();
                out.mul_assign(&input2);
                out.mul_assign(&input);
                out
            }
            11 => input.pow_u64(11),
            _ => input.pow_u64(self.params.d),
        }
    }
}
