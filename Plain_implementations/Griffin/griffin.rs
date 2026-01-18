use super::griffin_params::GriffinParams;
use crate::fields::FieldElement;
use crate::utils::pow_biguint;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Griffin<F: FieldElement> {
    pub(crate) params: Arc<GriffinParams<F>>,
}

impl<F: FieldElement> Griffin<F> {
    pub fn new(params: &Arc<GriffinParams<F>>) -> Self {
        Griffin {
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
            state = self.non_linear(&state);
            self.affine(&mut state, r);
        }

        state
    }

    fn affine_3(&self, input: &mut [F], round: usize) {
        let mut sum = input[0].clone();
        for el in input.iter().skip(1) {
            sum.add_assign(el);
        }

        if round < self.params.rounds - 1 {
            for (el, rc) in input
                .iter_mut()
                .zip(self.params.round_constants[round].iter())
            {
                el.add_assign(&sum);
                el.add_assign(rc);
            }
        } else {
            for el in input.iter_mut() {
                el.add_assign(&sum);
            }
        }
    }

    fn affine_4(&self, input: &mut [F], round: usize) {
        let mut t_0 = input[0].clone();
        t_0.add_assign(&input[1]);
        let mut t_1 = input[2].clone();
        t_1.add_assign(&input[3]);
        let mut t_2 = input[1].clone();
        t_2.double();
        t_2.add_assign(&t_1);
        let mut t_3 = input[3].clone();
        t_3.double();
        t_3.add_assign(&t_0);
        let mut t_4 = t_1.clone();
        t_4.double();
        t_4.double();
        t_4.add_assign(&t_3);
        let mut t_5 = t_0.clone();
        t_5.double();
        t_5.double();
        t_5.add_assign(&t_2);
        let mut t_6 = t_3;
        t_6.add_assign(&t_5);
        let mut t_7 = t_2;
        t_7.add_assign(&t_4);
        input[0] = t_6;
        input[1] = t_5;
        input[2] = t_7;
        input[3] = t_4;

        if round < self.params.rounds - 1 {
            for (el, rc) in input
                .iter_mut()
                .zip(self.params.round_constants[round].iter())
            {
                el.add_assign(rc);
            }
        }
    }

    fn affine(&self, input: &mut [F], round: usize) {
        if self.params.t == 3 {
            self.affine_3(input, round);
            return;
        }
        if self.params.t == 4 {
            self.affine_4(input, round);
            return;
        }

        let t4 = self.params.t / 4;
        for chunk in input.chunks_exact_mut(4) {
            let mut t_0 = chunk[0].clone();
            t_0.add_assign(&chunk[1]);
            let mut t_1 = chunk[2].clone();
            t_1.add_assign(&chunk[3]);
            let mut t_2 = chunk[1].clone();
            t_2.double();
            t_2.add_assign(&t_1);
            let mut t_3 = chunk[3].clone();
            t_3.double();
            t_3.add_assign(&t_0);
            let mut t_4 = t_1.clone();
            t_4.double();
            t_4.double();
            t_4.add_assign(&t_3);
            let mut t_5 = t_0.clone();
            t_5.double();
            t_5.double();
            t_5.add_assign(&t_2);
            let mut t_6 = t_3;
            t_6.add_assign(&t_5);
            let mut t_7 = t_2;
            t_7.add_assign(&t_4);
            chunk[0] = t_6;
            chunk[1] = t_5;
            chunk[2] = t_7;
            chunk[3] = t_4;
        }

        let mut stored = vec![F::zero(); 4];
        for l in 0..4 {
            stored[l] = input[l].clone();
            for j in 1..t4 {
                stored[l].add_assign(&input[4 * j + l]);
            }
        }

        for i in 0..input.len() {
            input[i].add_assign(&stored[i % 4]);
            if round < self.params.rounds - 1 {
                input[i].add_assign(&self.params.round_constants[round][i]);
            }
        }
    }

    fn l(y01_i: &mut F, y0: &F, x: &F, i: usize) -> F {
        if i == 0 {
            y01_i.to_owned()
        } else {
            y01_i.add_assign(y0);
            let mut out = y01_i.to_owned();
            out.add_assign(x);
            out
        }
    }

    fn non_linear(&self, input: &[F]) -> Vec<F> {
        let mut output = input.to_owned();
        output[0] = pow_biguint(&output[0], &self.params.d_inv);
        output[1] = output[1].pow_u64(self.params.d);

        let mut y01_i = output[0].to_owned();
        let y0 = y01_i.to_owned();
        y01_i.add_assign(&output[1]);

        for (i, ((out, inp), con)) in output
            .iter_mut()
            .skip(2)
            .zip(input.iter().skip(1))
            .zip(self.params.alpha_beta.iter())
            .enumerate()
        {
            let mut l = Self::l(&mut y01_i, &y0, inp, i);
            let mut l_squ = l.to_owned();
            l_squ.square();
            l.mul_assign(&con[0]);
            l.add_assign(&l_squ);
            l.add_assign(&con[1]);
            out.mul_assign(&l);
        }

        output
    }
}
