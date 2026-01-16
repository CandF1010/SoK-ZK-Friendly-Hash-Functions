use super::anemoi_params::AnemoiParams;
use crate::fields::FieldElement;
use crate::utils::pow_biguint;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Anemoi<F: FieldElement> {
    pub(crate) params: Arc<AnemoiParams<F>>,
}

impl<F: FieldElement> Anemoi<F> {
    pub fn new(params: &Arc<AnemoiParams<F>>) -> Self {
        Anemoi {
            params: Arc::clone(params),
        }
    }

    pub fn get_t(&self) -> usize {
        self.params.width
    }

    pub fn permutation(&self, _input: &[F]) -> Vec<F> {
        let width = self.params.width;
        let n_cols = self.params.n_cols;
        assert_eq!(width, 2 * n_cols);
        assert_eq!(_input.len(), width);

        let mut state = _input.to_vec();
        for r in 0..self.params.rounds {
            self.add_round_constants(&mut state, r);
            self.linear_layer(&mut state);
            self.sbox_layer(&mut state);
        }
        self.linear_layer(&mut state);
        state
    }

    fn add_round_constants(&self, state: &mut [F], round: usize) {
        let n_cols = self.params.n_cols;
        for i in 0..n_cols {
            let mut x = state[i].clone();
            x.add_assign(&self.params.round_constants_c[round][i]);
            state[i] = x;
            let mut y = state[n_cols + i].clone();
            y.add_assign(&self.params.round_constants_d[round][i]);
            state[n_cols + i] = y;
        }
    }

    fn linear_layer(&self, state: &mut [F]) {
        let n_cols = self.params.n_cols;
        let mut x = state[..n_cols].to_vec();
        let mut y = state[n_cols..].to_vec();
        y.rotate_left(1);

        self.params.mds.apply(&mut x);
        self.params.mds.apply(&mut y);

        for i in 0..n_cols {
            y[i].add_assign(&x[i]);
            x[i].add_assign(&y[i]);
        }

        state[..n_cols].clone_from_slice(&x);
        state[n_cols..].clone_from_slice(&y);
    }

    fn sbox_layer(&self, state: &mut [F]) {
        let n_cols = self.params.n_cols;
        for i in 0..n_cols {
            let mut x = state[i].clone();
            let mut y = state[n_cols + i].clone();

            let y_quad = pow_u64(&y, self.params.quad);
            let mut beta_y_quad = self.params.beta.clone();
            beta_y_quad.mul_assign(&y_quad);
            x.sub_assign(&beta_y_quad);

            let x_alpha_inv = pow_biguint(&x, &self.params.alpha_inv);
            y.sub_assign(&x_alpha_inv);

            let y_quad = pow_u64(&y, self.params.quad);
            let mut beta_y_quad = self.params.beta.clone();
            beta_y_quad.mul_assign(&y_quad);
            x.add_assign(&beta_y_quad);
            x.add_assign(&self.params.delta);

            state[i] = x;
            state[n_cols + i] = y;
        }
    }
}

fn pow_u64<F: FieldElement>(base: &F, exp: u64) -> F {
    match exp {
        2 => {
            let mut out = base.clone();
            out.square();
            out
        }
        3 => {
            let mut out = base.clone();
            out.square();
            out.mul_assign(base);
            out
        }
        _ => base.pow_u64(exp),
    }
}
