use super::neptune_params::NeptuneParams;
use crate::fields::FieldElement;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Neptune<F: FieldElement> {
    pub(crate) params: Arc<NeptuneParams<F>>,
}

impl<F: FieldElement> Neptune<F> {
    pub fn new(params: &Arc<NeptuneParams<F>>) -> Self {
        Neptune {
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
        self.matmul_external(&mut state);

        let mut round = 0usize;
        for _ in 0..self.params.rounds_e_first {
            self.add_round_constants(&mut state, round);
            self.sbox_external(&mut state);
            self.matmul_external(&mut state);
            round += 1;
        }

        for _ in 0..self.params.rounds_i {
            self.add_round_constants(&mut state, round);
            self.sbox_internal(&mut state);
            self.matmul_internal(&mut state);
            round += 1;
        }

        for _ in 0..self.params.rounds_e_last {
            self.add_round_constants(&mut state, round);
            self.sbox_external(&mut state);
            self.matmul_external(&mut state);
            round += 1;
        }

        state
    }

    fn add_round_constants(&self, state: &mut [F], round: usize) {
        for (el, rc) in state.iter_mut().zip(self.params.round_constants[round].iter()) {
            el.add_assign(rc);
        }
    }

    fn sbox_internal(&self, state: &mut [F]) {
        state[0] = state[0].pow_u64(self.params.d);
    }

    fn sbox_external(&self, state: &mut [F]) {
        for i in 0..self.params.t_prime {
            let (y0, y1) = self.sbox_external_pair(&state[2 * i], &state[2 * i + 1]);
            state[2 * i] = y0;
            state[2 * i + 1] = y1;
        }
    }

    fn sbox_external_pair(&self, x0: &F, x1: &F) -> (F, F) {
        let alpha = &self.params.alpha;
        let gamma = &self.params.gamma;

        let mut alpha_sq = alpha.clone();
        alpha_sq.square();

        let mut x0_minus_x1 = x0.clone();
        x0_minus_x1.sub_assign(x1);
        let mut x0_minus_x1_sq = x0_minus_x1.clone();
        x0_minus_x1_sq.square();

        let mut two_x0 = x0.clone();
        two_x0.double();
        let mut two_x0_plus_x1 = two_x0.clone();
        two_x0_plus_x1.add_assign(x1);

        let mut two_x1 = x1.clone();
        two_x1.double();
        let mut x0_minus_2x1 = x0.clone();
        x0_minus_2x1.sub_assign(&two_x1);

        let mut inner = gamma.clone();
        let mut alpha_x0_minus_2x1 = alpha.clone();
        alpha_x0_minus_2x1.mul_assign(&x0_minus_2x1);
        inner.add_assign(&alpha_x0_minus_2x1);
        inner.sub_assign(&x0_minus_x1_sq);
        let mut inner_sq = inner.clone();
        inner_sq.square();

        let mut three_alpha = alpha.clone();
        three_alpha.double();
        three_alpha.add_assign(alpha);
        let mut four_alpha = alpha.clone();
        four_alpha.double();
        four_alpha.double();

        let mut y0 = two_x0_plus_x1;
        y0.mul_assign(&alpha_sq);
        let mut term0 = x0_minus_x1_sq.clone();
        term0.mul_assign(&three_alpha);
        y0.add_assign(&term0);
        y0.add_assign(&inner_sq);

        let mut y1 = x0.clone();
        y1.add_assign(x1);
        y1.add_assign(x1);
        y1.add_assign(x1);
        y1.mul_assign(&alpha_sq);
        let mut term1 = x0_minus_x1_sq;
        term1.mul_assign(&four_alpha);
        y1.add_assign(&term1);
        y1.add_assign(&inner_sq);

        (y0, y1)
    }

    fn matmul_external(&self, state: &mut [F]) {
        let t_prime = self.params.t_prime;
        let mut even = Vec::with_capacity(t_prime);
        let mut odd = Vec::with_capacity(t_prime);
        for i in 0..t_prime {
            even.push(state[2 * i].clone());
            odd.push(state[2 * i + 1].clone());
        }

        let even = self.matmul(&even, &self.params.mds_even);
        let odd = self.matmul(&odd, &self.params.mds_odd);

        for i in 0..t_prime {
            state[2 * i] = even[i].clone();
            state[2 * i + 1] = odd[i].clone();
        }
    }

    fn matmul_internal(&self, state: &mut [F]) {
        let mut sum = state[0].clone();
        for el in state.iter().skip(1) {
            sum.add_assign(el);
        }

        for (i, val) in state.iter_mut().enumerate() {
            let mut scaled = val.clone();
            scaled.mul_assign(&self.params.mat_internal_diag_m_1[i]);
            scaled.add_assign(&sum);
            *val = scaled;
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
}
