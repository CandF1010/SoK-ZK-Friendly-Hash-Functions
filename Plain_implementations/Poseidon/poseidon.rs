use crate::fields::FieldElement;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct PoseidonParams<F: FieldElement> {
    pub(crate) t: usize,
    pub(crate) d: u64,
    pub(crate) rounds_f_beginning: usize,
    pub(crate) rounds_p: usize,
    pub(crate) round_constants: Vec<Vec<F>>, // [round_idx][state_idx]
    pub(crate) mds_full: Vec<Vec<F>>,
    pub(crate) mds_partial: Vec<Vec<F>>,
}

impl<F: FieldElement> PoseidonParams<F> {
    pub fn new(
        t: usize,
        d: u64,
        rounds_f: usize,
        rounds_p: usize,
        mds_full: &[Vec<F>],
        mds_partial: &[Vec<F>],
        round_constants: &[Vec<F>],
    ) -> Self {
        PoseidonParams {
            t,
            d,
            rounds_f_beginning: rounds_f / 2,
            rounds_p,
            round_constants: round_constants.to_owned(), // [round_idx][state_idx]
            mds_full: mds_full.to_owned(),
            mds_partial: mds_partial.to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Poseidon<F: FieldElement> {
    pub(crate) params: Arc<PoseidonParams<F>>,
}

impl<F: FieldElement> Poseidon<F> {
    pub fn new(params: &Arc<PoseidonParams<F>>) -> Self {
        Poseidon {
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
        let half_f = self.params.rounds_f_beginning;
        let mut round = 0usize;

        // Match Poseidon2b-style flow: Minit + full/partial/full.
        self.mul_mds_full(&mut state);

        for _ in 0..half_f {
            self.round_full(&mut state, round);
            round += 1;
        }

        for _ in 0..self.params.rounds_p {
            self.round_partial(&mut state, round);
            round += 1;
        }

        for _ in 0..half_f {
            self.round_full(&mut state, round);
            round += 1;
        }

        state
    }

    #[inline(always)]
    fn round_full(&self, state: &mut [F], round: usize) {
        self.add_rc_in_place(state, round);
        for el in state.iter_mut() {
            *el = self.sbox_p(el);
        }
        self.mul_mds_full(state);
    }

    #[inline(always)]
    fn round_partial(&self, state: &mut [F], round: usize) {
        self.add_rc_in_place(state, round);
        state[0] = self.sbox_p(&state[0]);
        self.mul_mds_partial(state);
    }

    fn sbox_p(&self, input: &F) -> F {
        let mut input2 = input.clone();
        input2.square();

        match self.params.d {
            3 => {
                let mut out = input2;
                out.mul_assign(input);
                out
            }
            5 => {
                let mut out = input2;
                out.square();
                out.mul_assign(input);
                out
            }
            7 => {
                let mut out = input2.clone();
                out.square();
                out.mul_assign(&input2);
                out.mul_assign(input);
                out
            }
            11 => {
                let mut x4 = input2.clone();
                x4.square();
                let mut x8 = x4.clone();
                x8.square();
                x8.mul_assign(&input2);
                x8.mul_assign(input);
                x8
            }
            _ => panic!("unsupported s-box degree"),
        }
    }

    fn mul_mds_full(&self, state: &mut [F]) {
        self.matmul_in_place(state, &self.params.mds_full);
    }

    fn mul_mds_partial(&self, state: &mut [F]) {
        // y_i = (mu_i - 1) * x_i + sum_j x_j for diag-plus-ones partial MDS.
        let mut sum = F::zero();
        for x in state.iter() {
            sum.add_assign(x);
        }

        let one = F::one();
        for i in 0..state.len() {
            let mut mu_minus_one = self.params.mds_partial[i][i].clone();
            mu_minus_one.sub_assign(&one);

            let mut corr = mu_minus_one;
            corr.mul_assign(&state[i]);

            let mut out = sum.clone();
            out.add_assign(&corr);
            state[i] = out;
        }
    }

    fn matmul_in_place(&self, state: &mut [F], mat: &[Vec<F>]) {
        let t = state.len();
        let mut out = vec![F::zero(); t];
        for row in 0..t {
            let mut acc = F::zero();
            for (col, val) in state.iter().enumerate() {
                let mut tmp = mat[row][col].clone();
                tmp.mul_assign(val);
                acc.add_assign(&tmp);
            }
            out[row] = acc;
        }
        state.clone_from_slice(&out);
    }

    fn add_rc_in_place(&self, state: &mut [F], round: usize) {
        let rc = &self.params.round_constants[round];
        for (x, c) in state.iter_mut().zip(rc.iter()) {
            x.add_assign(c);
        }
    }
}
