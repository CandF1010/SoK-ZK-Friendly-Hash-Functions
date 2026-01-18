use super::monolith_params::{Monolith31Params, Monolith64Params, MonolithField32, MonolithField64};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Monolith64<F: MonolithField64> {
    pub(crate) params: Arc<Monolith64Params<F>>,
}

#[derive(Clone, Debug)]
pub struct Monolith31<F: MonolithField32> {
    pub(crate) params: Arc<Monolith31Params<F>>,
}

impl<F: MonolithField64> Monolith64<F> {
    pub fn new(params: &Arc<Monolith64Params<F>>) -> Self {
        Monolith64 {
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
        self.concrete(&mut state, None);

        for rc in self.params.round_constants.iter() {
            self.bars(&mut state);
            self.bricks(&mut state);
            self.concrete(&mut state, Some(rc));
        }

        self.bars(&mut state);
        self.bricks(&mut state);
        self.concrete(&mut state, None);
        state
    }

    fn concrete(&self, state: &mut [F], rc: Option<&[F]>) {
        let t = self.params.t;
        let mut out = vec![F::zero(); t];
        for row in 0..t {
            if let Some(rc) = rc {
                out[row].add_assign(&rc[row]);
            }
            for col in 0..t {
                let mut tmp = self.params.mds[row][col].clone();
                tmp.mul_assign(&state[col]);
                out[row].add_assign(&tmp);
            }
        }
        state.clone_from_slice(&out);
    }

    fn bricks(&self, state: &mut [F]) {
        let prev = state.to_vec();
        for i in 1..state.len() {
            let mut sq = prev[i - 1].clone();
            sq.square();
            state[i].add_assign(&sq);
        }
    }

    fn bars(&self, state: &mut [F]) {
        for el in state.iter_mut().take(Monolith64Params::<F>::BARS) {
            let mut value = el.to_u64();
            value = self.bar_u64_lookup(value);
            *el = F::from_u64(value);
        }
    }

    fn bar_u64_lookup(&self, value: u64) -> u64 {
        let l1 = self.params.lookup[(value & 0xffff) as usize] as u64;
        let l2 = self.params.lookup[((value >> 16) & 0xffff) as usize] as u64;
        let l3 = self.params.lookup[((value >> 32) & 0xffff) as usize] as u64;
        let l4 = self.params.lookup[((value >> 48) & 0xffff) as usize] as u64;
        l1 | (l2 << 16) | (l3 << 32) | (l4 << 48)
    }
}

impl<F: MonolithField32> Monolith31<F> {
    pub fn new(params: &Arc<Monolith31Params<F>>) -> Self {
        Monolith31 {
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
        self.concrete(&mut state, None);

        for rc in self.params.round_constants.iter() {
            self.bars(&mut state);
            self.bricks(&mut state);
            self.concrete(&mut state, Some(rc));
        }

        self.bars(&mut state);
        self.bricks(&mut state);
        self.concrete(&mut state, None);
        state
    }

    fn concrete(&self, state: &mut [F], rc: Option<&[F]>) {
        let t = self.params.t;
        let mut out = vec![F::zero(); t];
        for row in 0..t {
            if let Some(rc) = rc {
                out[row].add_assign(&rc[row]);
            }
            for col in 0..t {
                let mut tmp = self.params.mds[row][col].clone();
                tmp.mul_assign(&state[col]);
                out[row].add_assign(&tmp);
            }
        }
        state.clone_from_slice(&out);
    }

    fn bricks(&self, state: &mut [F]) {
        let prev = state.to_vec();
        for i in 1..state.len() {
            let mut sq = prev[i - 1].clone();
            sq.square();
            state[i].add_assign(&sq);
        }
    }

    fn bars(&self, state: &mut [F]) {
        for el in state.iter_mut().take(Monolith31Params::<F>::BARS) {
            let mut value = el.to_u32();
            value = self.bar_u32_lookup(value);
            *el = F::from_u64(value as u64);
        }
    }

    fn bar_u32_lookup(&self, value: u32) -> u32 {
        let low = self.params.lookup1[(value & 0xffff) as usize] as u32;
        let high = self.params.lookup2[(value >> 16) as usize] as u32;
        low | (high << 16)
    }
}
