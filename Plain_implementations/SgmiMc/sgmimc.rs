use crate::fields::FieldElement;
use std::sync::Arc;

/// S-GMiMC: GMiMC_erf with square S-box (x → x²).
///
/// As proposed in the SoK paper, this variant replaces the original
/// high-exponent power map with x² for better plain performance,
/// and uses updated round numbers derived from a hash-focused
/// (rather than cipher-focused) security analysis.

#[derive(Clone, Debug)]
pub struct SgmiMcParams<F: FieldElement> {
    pub(crate) t: usize,
    pub(crate) d: u64,
    pub(crate) rounds: usize,
    pub(crate) round_constants: Vec<F>,
}

impl<F: FieldElement> SgmiMcParams<F> {
    pub fn new(t: usize, d: u64, rounds: usize, round_constants: &[F]) -> Self {
        SgmiMcParams {
            t,
            d,
            rounds,
            round_constants: round_constants.to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SgmiMc<F: FieldElement> {
    pub(crate) params: Arc<SgmiMcParams<F>>,
}

impl<F: FieldElement> SgmiMc<F> {
    pub fn new(params: &Arc<SgmiMcParams<F>>) -> Self {
        SgmiMc {
            params: Arc::clone(params),
        }
    }

    pub fn get_t(&self) -> usize {
        self.params.t
    }

    fn sbox(&self, state_0: &F, round: usize) -> F {
        let mut input = state_0.clone();
        input.add_assign(&self.params.round_constants[round]);

        match self.params.d {
            2 => {
                input.square();
                input
            }
            3 => {
                let mut t = input.clone();
                t.square();
                t.mul_assign(&input);
                t
            }
            8 => {
                input.square();    // x^2
                input.square();    // x^4
                input.square();    // x^8
                input
            }
            _ => input.pow_u64(self.params.d),
        }
    }

    fn round(&self, state: &mut [F], round: usize) {
        let power = self.sbox(&state[0], round);
        for el in state.iter_mut().skip(1) {
            el.add_assign(&power);
        }
    }

    fn permutation_not_opt(&self, input: &[F]) -> Vec<F> {
        let mut state = input.to_vec();
        if self.params.rounds == 0 {
            return state;
        }

        for r in 0..self.params.rounds - 1 {
            self.round(&mut state, r);
            state.rotate_right(1);
        }

        self.round(&mut state, self.params.rounds - 1);
        state
    }

    pub fn permutation(&self, input: &[F]) -> Vec<F> {
        let t = self.params.t;
        assert_eq!(input.len(), t);

        if t < 8 {
            return self.permutation_not_opt(input);
        }

        let mut state = input.to_vec();
        if self.params.rounds == 0 {
            return state;
        }

        let mut acc = F::zero();
        let mut acc_queue = vec![F::zero(); t - 1];

        for r in 0..self.params.rounds - 1 {
            let power = self.sbox(&state[0], r);

            acc_queue.rotate_right(1);
            acc.sub_assign(&acc_queue[0]);
            acc_queue[0] = power.clone();
            acc.add_assign(&power);

            state.rotate_right(1);
            state[0].add_assign(&acc);
        }

        let power = self.sbox(&state[0], self.params.rounds - 1);
        acc_queue.rotate_right(1);
        acc.sub_assign(&acc_queue[0]);
        acc_queue[0] = power;
        acc.add_assign(&acc_queue[0]);
        state[t - 1].add_assign(&acc);

        for el in state.iter_mut().skip(1).take(t - 2).rev() {
            acc_queue.rotate_right(1);
            acc.sub_assign(&acc_queue[0]);
            el.add_assign(&acc);
        }

        state
    }
}
