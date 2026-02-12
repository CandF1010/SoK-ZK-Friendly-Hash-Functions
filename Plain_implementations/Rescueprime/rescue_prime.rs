use crate::fields::FieldElement;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct RescuePrimeParams<F: FieldElement> {
    pub(crate) t: usize,
    pub(crate) d: u64,
    pub(crate) d_inv: [u64; 4],
    pub(crate) rounds: usize,
    pub(crate) mds: Vec<Vec<F>>,
    pub(crate) round_constants: Vec<Vec<F>>, // [round_idx][state_idx], round_idx in [0, 2 * rounds)
}

impl<F: FieldElement> RescuePrimeParams<F> {
    pub fn new(
        t: usize,
        d: u64,
        d_inv_words: [u64; 4],
        rounds: usize,
        mds: &[Vec<F>],
        round_constants: &[Vec<F>],
    ) -> Self {
        assert!(d == 3 || d == 5 || d == 7);
        assert_eq!(mds.len(), t);
        for row in mds {
            assert_eq!(row.len(), t);
        }
        assert_eq!(round_constants.len(), 2 * rounds);
        for rc in round_constants {
            assert_eq!(rc.len(), t);
        }

        RescuePrimeParams {
            t,
            d,
            d_inv: d_inv_words,
            rounds,
            mds: mds.to_owned(),
            round_constants: round_constants.to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RescuePrime<F: FieldElement> {
    pub(crate) params: Arc<RescuePrimeParams<F>>,
}

impl<F: FieldElement> RescuePrime<F> {
    pub fn new(params: &Arc<RescuePrimeParams<F>>) -> Self {
        RescuePrime {
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
        for r in 0..self.params.rounds {
            for x in state.iter_mut() {
                *x = self.sbox_p(x);
            }
            self.affine_round(&mut state, 2 * r);

            for x in state.iter_mut() {
                *x = self.sbox_p_inv(x);
            }
            self.affine_round(&mut state, 2 * r + 1);
        }
        state
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
            _ => panic!("unsupported s-box degree"),
        }
    }

    fn sbox_p_inv(&self, input: &F) -> F {
        input.pow_words_le(&self.params.d_inv)
    }

    fn affine_round(&self, state: &mut [F], round: usize) {
        self.matmul_in_place(state, &self.params.mds);
        self.add_rc_in_place(state, round);
    }

    fn matmul_in_place(&self, state: &mut [F], mat: &[Vec<F>]) {
        let t = state.len();
        let mut out = vec![F::zero(); t];
        for row in 0..t {
            for (col, val) in state.iter().enumerate().take(t) {
                let mut tmp = mat[row][col].clone();
                tmp.mul_assign(val);
                out[row].add_assign(&tmp);
            }
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
