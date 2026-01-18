use super::reinforced_concrete_params::ReinforcedConcreteParams;
use crate::fields::{FieldElement, PrimeFieldWords};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ReinforcedConcrete<F: PrimeFieldWords> {
    pub(crate) params: Arc<ReinforcedConcreteParams<F>>,
}

impl<F: PrimeFieldWords> ReinforcedConcrete<F> {
    pub fn new(params: &Arc<ReinforcedConcreteParams<F>>) -> Self {
        ReinforcedConcrete {
            params: Arc::clone(params),
        }
    }

    pub fn get_t(&self) -> usize {
        self.params.get_t()
    }

    pub fn permutation(&self, input: &[F]) -> Vec<F> {
        assert_eq!(input.len(), ReinforcedConcreteParams::<F>::T);
        let mut state = [input[0].clone(), input[1].clone(), input[2].clone()];

        self.concrete(&mut state, 0);

        for round in 1..=ReinforcedConcreteParams::<F>::PRE_ROUNDS {
            state = self.bricks(&state);
            self.concrete(&mut state, round);
        }

        state = self.bars(&state);
        self.concrete(
            &mut state,
            ReinforcedConcreteParams::<F>::PRE_ROUNDS + 1,
        );

        for round in ReinforcedConcreteParams::<F>::PRE_ROUNDS + 2
            ..=ReinforcedConcreteParams::<F>::TOTAL_ROUNDS
        {
            state = self.bricks(&state);
            self.concrete(&mut state, round);
        }

        state.to_vec()
    }

    fn concrete(&self, state: &mut [F; 3], round: usize) {
        let mut sum = state[0].clone();
        sum.add_assign(&state[1]);
        sum.add_assign(&state[2]);

        for (el, rc) in state.iter_mut().zip(self.params.round_constants[round].iter()) {
            el.add_assign(&sum);
            el.add_assign(rc);
        }
    }

    fn bricks(&self, state: &[F; 3]) -> [F; 3] {
        let mut out = [F::zero(), F::zero(), F::zero()];
        let d = self.params.d;

        let mut x0_sq = state[0].clone();
        x0_sq.square();
        let mut x1_sq = state[1].clone();
        x1_sq.square();

        let mut x0_pow = match d {
            3 | 5 => x0_sq.clone(),
            _ => state[0].pow_u64(d),
        };
        if d == 5 {
            x0_pow.square();
        }
        if d == 3 || d == 5 {
            x0_pow.mul_assign(&state[0]);
        }
        out[0] = x0_pow;

        let mut t1 = x0_sq;
        let mut alpha0 = self.params.alpha[0].clone();
        alpha0.mul_assign(&state[0]);
        t1.add_assign(&alpha0);
        t1.add_assign(&self.params.beta[0]);
        t1.mul_assign(&state[1]);
        out[1] = t1;

        let mut t2 = x1_sq;
        let mut alpha1 = self.params.alpha[1].clone();
        alpha1.mul_assign(&state[1]);
        t2.add_assign(&alpha1);
        t2.add_assign(&self.params.beta[1]);
        t2.mul_assign(&state[2]);
        out[2] = t2;

        out
    }

    fn bars(&self, state: &[F; 3]) -> [F; 3] {
        let mut out = state.to_owned();
        for el in out.iter_mut() {
            let mut digits = self.decompose(el);
            for digit in digits.iter_mut() {
                if *digit < self.params.p_prime {
                    *digit = self.params.sbox[*digit as usize];
                }
            }
            *el = self.compose(&digits);
        }
        out
    }

    fn decompose(&self, val: &F) -> Vec<u16> {
        let mut limbs = val.to_words_le();
        let mut res = vec![0u16; self.params.si.len()];
        for i in (1..self.params.si.len()).rev() {
            let (quot, rem) = div_rem_small(&limbs, self.params.si[i] as u64);
            res[i] = rem;
            limbs = quot;
        }
        res[0] = limbs[0] as u16;
        res
    }

    fn compose(&self, digits: &[u16]) -> F {
        let mut acc = F::from_u64(digits[0] as u64);
        for (digit, base) in digits.iter().zip(self.params.si.iter()).skip(1) {
            let base_f = F::from_u64(*base as u64);
            acc.mul_assign(&base_f);
            acc.add_assign(&F::from_u64(*digit as u64));
        }
        acc
    }
}

fn div_rem_small(limbs: &[u64; 4], base: u64) -> ([u64; 4], u16) {
    let mut out = [0u64; 4];
    let mut rem: u128 = 0;
    for i in (0..4).rev() {
        let cur = (rem << 64) | limbs[i] as u128;
        out[i] = (cur / base as u128) as u64;
        rem = cur % base as u128;
    }
    (out, rem as u16)
}
