use crate::fields::FieldElement;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AnemoiParams<F: FieldElement> {
    pub(crate) n_cols: usize,
    pub(crate) width: usize,
    pub(crate) rounds: usize,
    pub(crate) alpha: u64,
    pub(crate) alpha_inv: [u64; 4],
    pub(crate) beta: F,
    pub(crate) delta: F,
    pub(crate) mds: Vec<Vec<F>>,
    pub(crate) round_constants_c: Vec<Vec<F>>,
    pub(crate) round_constants_d: Vec<Vec<F>>,
}

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
        self.apply_mds_only(&mut state);
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
        let x = state[..n_cols].to_vec();
        let mut y = state[n_cols..].to_vec();
        y.rotate_left(1);

        let mut new_x = vec![F::zero(); n_cols];
        let mut new_y = vec![F::zero(); n_cols];

        for r in 0..n_cols {
            let mut acc = F::zero();
            for c in 0..n_cols {
                let mut tmp = self.params.mds[r][c].clone();
                tmp.mul_assign(&x[c]);
                acc.add_assign(&tmp);
            }
            new_x[r] = acc;
        }

        for r in 0..n_cols {
            let mut acc = F::zero();
            for c in 0..n_cols {
                let mut tmp = self.params.mds[r][c].clone();
                tmp.mul_assign(&y[c]);
                acc.add_assign(&tmp);
            }
            new_y[r] = acc;
        }

        for i in 0..n_cols {
            new_y[i].add_assign(&new_x[i]);
            new_x[i].add_assign(&new_y[i]);
        }

        state[..n_cols].clone_from_slice(&new_x);
        state[n_cols..].clone_from_slice(&new_y);
    }

    fn apply_mds_only(&self, state: &mut [F]) {
        let n_cols = self.params.n_cols;
        let x = state[..n_cols].to_vec();
        let mut y = state[n_cols..].to_vec();
        y.rotate_left(1);

        let mut new_x = vec![F::zero(); n_cols];
        let mut new_y = vec![F::zero(); n_cols];

        for r in 0..n_cols {
            let mut acc = F::zero();
            for c in 0..n_cols {
                let mut tmp = self.params.mds[r][c].clone();
                tmp.mul_assign(&x[c]);
                acc.add_assign(&tmp);
            }
            new_x[r] = acc;
        }

        for r in 0..n_cols {
            let mut acc = F::zero();
            for c in 0..n_cols {
                let mut tmp = self.params.mds[r][c].clone();
                tmp.mul_assign(&y[c]);
                acc.add_assign(&tmp);
            }
            new_y[r] = acc;
        }

        state[..n_cols].clone_from_slice(&new_x);
        state[n_cols..].clone_from_slice(&new_y);
    }

    fn sbox_layer(&self, state: &mut [F]) {
        let n_cols = self.params.n_cols;
        for i in 0..n_cols {
            let mut x = state[i].clone();
            let mut y = state[n_cols + i].clone();

            let y_pow = pow_u64(&y, self.params.alpha);
            let mut beta_y_pow = self.params.beta.clone();
            beta_y_pow.mul_assign(&y_pow);
            x.sub_assign(&beta_y_pow);
            x.sub_assign(&self.params.delta);

            let x_alpha_inv = x.pow_words_le(&self.params.alpha_inv);
            y.sub_assign(&x_alpha_inv);

            let y_pow_new = pow_u64(&y, self.params.alpha);
            let mut beta_y_pow = self.params.beta.clone();
            beta_y_pow.mul_assign(&y_pow_new);
            x.add_assign(&beta_y_pow);

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
