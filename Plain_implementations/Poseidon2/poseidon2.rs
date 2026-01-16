use super::poseidon2_params::Poseidon2Params;
use crate::fields::FieldElement;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Poseidon2<F: FieldElement> {
    pub(crate) params: Arc<Poseidon2Params<F>>,
}

impl<F: FieldElement> Poseidon2<F> {
    pub fn new(params: &Arc<Poseidon2Params<F>>) -> Self {
        Poseidon2 {
            params: Arc::clone(params),
        }
    }

    pub fn get_t(&self) -> usize {
        self.params.t
    }

    pub fn permutation(&self, input: &[F]) -> Vec<F> {
        let t = self.params.t;
        assert_eq!(input.len(), t);

        let mut current_state = input.to_vec();

        self.matmul_external(&mut current_state);

        for r in 0..self.params.rounds_f_beginning {
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(&current_state);
            self.matmul_external(&mut current_state);
        }

        let p_end = self.params.rounds_f_beginning + self.params.rounds_p;
        for r in self.params.rounds_f_beginning..p_end {
            current_state[0].add_assign(&self.params.round_constants[r][0]);
            current_state[0] = self.sbox_p(&current_state[0]);
            self.matmul_internal(&mut current_state, &self.params.mat_internal_diag_m_1);
        }

        for r in p_end..self.params.rounds {
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(&current_state);
            self.matmul_external(&mut current_state);
        }

        current_state
    }

    fn sbox(&self, input: &[F]) -> Vec<F> {
        input.iter().map(|el| self.sbox_p(el)).collect()
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

    fn matmul_external(&self, input: &mut [F]) {
        let t = self.params.t;
        match t {
            2 => {
                let mut sum = input[0].clone();
                sum.add_assign(&input[1]);
                input[0].add_assign(&sum);
                input[1].add_assign(&sum);
            }
            3 => {
                let mut sum = input[0].clone();
                sum.add_assign(&input[1]);
                sum.add_assign(&input[2]);
                input[0].add_assign(&sum);
                input[1].add_assign(&sum);
                input[2].add_assign(&sum);
            }
            4 | 8 | 12 | 16 | 20 | 24 => {
                let t4 = t / 4;
                for i in 0..t4 {
                    let start_index = i * 4;
                    let mut t_0 = input[start_index].clone();
                    t_0.add_assign(&input[start_index + 1]);
                    let mut t_1 = input[start_index + 2].clone();
                    t_1.add_assign(&input[start_index + 3]);
                    let mut t_2 = input[start_index + 1].clone();
                    t_2.double();
                    t_2.add_assign(&t_1);
                    let mut t_3 = input[start_index + 3].clone();
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
                    let mut t_6 = t_3.clone();
                    t_6.add_assign(&t_5);
                    let mut t_7 = t_2.clone();
                    t_7.add_assign(&t_4);
                    input[start_index] = t_6;
                    input[start_index + 1] = t_5;
                    input[start_index + 2] = t_7;
                    input[start_index + 3] = t_4;
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
                }
            }
            _ => panic!("unsupported width"),
        }
    }

    fn matmul_internal(&self, input: &mut [F], mat_internal_diag_m_1: &[F]) {
        let t = self.params.t;

        match t {
            2 => {
                let mut sum = input[0].clone();
                sum.add_assign(&input[1]);
                input[0].add_assign(&sum);
                input[1].double();
                input[1].add_assign(&sum);
            }
            3 => {
                let mut sum = input[0].clone();
                sum.add_assign(&input[1]);
                sum.add_assign(&input[2]);
                input[0].add_assign(&sum);
                input[1].add_assign(&sum);
                input[2].double();
                input[2].add_assign(&sum);
            }
            4 | 8 | 12 | 16 | 20 | 24 => {
                let mut sum = input[0].clone();
                for el in input.iter().skip(1) {
                    sum.add_assign(el);
                }
                for i in 0..input.len() {
                    input[i].mul_assign(&mat_internal_diag_m_1[i]);
                    input[i].add_assign(&sum);
                }
            }
            _ => panic!("unsupported width"),
        }
    }

    fn add_rc(&self, input: &[F], rc: &[F]) -> Vec<F> {
        input
            .iter()
            .zip(rc.iter())
            .map(|(a, b)| {
                let mut r = a.clone();
                r.add_assign(b);
                r
            })
            .collect()
    }
}
