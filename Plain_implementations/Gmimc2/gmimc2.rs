use crate::fields::FieldElement;
use std::sync::Arc;

/// GMiMCHash2 — GMiMC^π_erf2 permutation as specified in §5.1.2 of the SoK paper.
///
/// Differences from the original GMiMC_erf:
///   1. S-box exponent α is fixed to 2 (square).
///   2. An Input/Output matrix `M_IO` is applied before the first round and after the
///      last round. For 256-bit fields (c=1) it is the identity; for smaller fields it is
///      a sparse circulant matrix (see §5.1.2, page 25).
///   3. Round numbers are taken from Table 9 of the SoK paper.
///
/// Permutation structure:
///   π(x) = M_IO ∘ R^(R) ∘ … ∘ R^(1) (M_IO · x)
///
/// Round function R^(i) (Figure 5):
///   state[0]  += rc[i]                 // constant addition
///   pow        = state[0]²             // square S-box
///   state[1..] += pow                  // distribute
///   rotate_right(1)  (all rounds except the last)

#[derive(Clone, Debug)]
pub struct Gmimc2Params<F: FieldElement> {
    pub(crate) t: usize,
    pub(crate) rounds: usize,
    pub(crate) round_constants: Vec<F>,
}

impl<F: FieldElement> Gmimc2Params<F> {
    pub fn new(t: usize, rounds: usize, round_constants: &[F]) -> Self {
        Gmimc2Params {
            t,
            rounds,
            round_constants: round_constants.to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Gmimc2<F: FieldElement> {
    pub(crate) params: Arc<Gmimc2Params<F>>,
}

impl<F: FieldElement> Gmimc2<F> {
    pub fn new(params: &Arc<Gmimc2Params<F>>) -> Self {
        Gmimc2 {
            params: Arc::clone(params),
        }
    }

    pub fn get_t(&self) -> usize {
        self.params.t
    }

    // ── Input/Output matrix (§5.1.2, page 25) ──
    //
    //  c=1 (256-bit)           → identity
    //  t∈{8,16}, c=r=t/2      → circ(1,0,…,0,2,0,…,0)   with 2 at offset t/2
    //  t∈{12,24}, c=t/3       → circ(1,0,…,0,2,0,…,0,2,0,…,0)  with 2s at t/3 and t/2
    //  t=4                    → identity (implicit from c=1 rule)

    fn apply_circulant(state: &[F]) -> Vec<F> {
        let t = state.len();
        let offsets: &[usize] = match t {
            // 256-bit fields: identity
            2 | 3 | 4 | 5 => return state.to_vec(),
            // t∈{8,16}: 2 at t/2
            8 => &[4],
            16 => &[8],
            // t∈{12,24}: 2 at t/3 and t/2
            12 => &[4, 6],
            24 => &[8, 12],
            // fallback
            _ => &[t / 2],
        };
        let mut result = state.to_vec();
        for (i, ri) in result.iter_mut().enumerate() {
            for &off in offsets {
                let mut term = state[(i + off) % t].clone();
                term.double();
                ri.add_assign(&term);
            }
        }
        result
    }

    // ── S-box: (x + rc)² ──

    #[inline(always)]
    fn sbox_p(&self, x: &F, round: usize) -> F {
        let mut val = x.clone();
        val.add_assign(&self.params.round_constants[round]);
        val.square();
        val
    }

    // ── Permutation ──

    pub fn permutation(&self, input: &[F]) -> Vec<F> {
        let t = self.params.t;
        let r = self.params.rounds;
        assert_eq!(input.len(), t);
        if r == 0 {
            return input.to_vec();
        }

        let mut state = Self::apply_circulant(input);

        // For small t the accumulator queue doesn't pay off — use the simple loop.
        if t < 8 {
            for round in 0..r - 1 {
                let pow = self.sbox_p(&state[0], round);
                for el in state.iter_mut().skip(1) {
                    el.add_assign(&pow);
                }
                state.rotate_right(1);
            }
            let pow = self.sbox_p(&state[0], r - 1);
            for el in state.iter_mut().skip(1) {
                el.add_assign(&pow);
            }
            return Self::apply_circulant(&state);
        }

        let mut acc = F::zero();
        let mut acc_q = vec![F::zero(); t - 1];

        // Rounds 0 .. R-2 (with rotation)
        for round in 0..r - 1 {
            let pow = self.sbox_p(&state[0], round);

            acc_q.rotate_right(1);
            acc.sub_assign(&acc_q[0]);
            acc_q[0] = pow;
            acc.add_assign(&acc_q[0]);

            state.rotate_right(1);
            state[0].add_assign(&acc);
        }

        // Final round — no rotation, inject at state[t-1]
        {
            let pow = self.sbox_p(&state[0], r - 1);

            acc_q.rotate_right(1);
            acc.sub_assign(&acc_q[0]);
            acc_q[0] = pow;
            acc.add_assign(&acc_q[0]);

            state[t - 1].add_assign(&acc);
        }

        // Flush accumulator into remaining positions (reverse order)
        for el in state.iter_mut().skip(1).take(t - 2).rev() {
            acc_q.rotate_right(1);
            acc.sub_assign(&acc_q[0]);
            el.add_assign(&acc);
        }

        Self::apply_circulant(&state)
    }
}
