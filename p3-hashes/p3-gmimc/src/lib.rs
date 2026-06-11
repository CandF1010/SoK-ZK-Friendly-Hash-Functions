#![no_std]

extern crate alloc;

use alloc::{vec, vec::Vec};

use p3_field::{Algebra, PrimeField};
use p3_symmetric::Permutation;
use rand::distr::{Distribution, StandardUniform};
use rand::{Rng, RngExt};


#[derive(Clone, Debug)]
pub struct GMiMC<F, const WIDTH: usize, const D: usize> {
    num_rounds: usize,
    constants: Vec<F>,
}

impl<F, const WIDTH: usize, const D: usize> GMiMC<F, WIDTH, D>
where
    F: PrimeField,
{
    pub fn new(num_rounds: usize, constants: Vec<F>) -> Self {
        Self {
            num_rounds,
            constants,
        }
    }

    pub fn new_from_rng<R: Rng>(num_rounds: usize, rng: &mut R) -> Self
    where
        StandardUniform: Distribution<F> 
    {
        let constants = rng
            .sample_iter(StandardUniform)
            .take(num_rounds)
            .collect::<Vec<_>>();
        Self {
            num_rounds,
            constants,
        }
    }

    #[inline(always)]
    fn feistel_erf<A: Algebra<F>>(
        state: &[A; WIDTH],
        constant: &F,
    ) -> [A; WIDTH]
    where
        F: PrimeField,
        A: Algebra<F>,
    {
        let mut result = [A::ZERO; WIDTH];
        let x_0 = state[0].clone() + constant.clone();
        let mut pow = x_0.clone().square();
        match D {
            3 => {
                pow *= x_0.clone();
            },
            5 => {
                pow = pow.square().square();
                pow *= x_0.clone();
            },
            7 => {
                let pow_2 = x_0.clone();
                pow = pow.square();
                pow *= pow_2.clone();
                pow *= x_0.clone();
            },
            _ => {
                for _ in 0..D {
                    pow *= x_0.clone();
                }
            }
        };
        for i in (0..(WIDTH - 1)).into_iter()  {
            result[i] += state[i + 1].clone() + pow.clone();
        }
        result[WIDTH - 1] += state[0].clone();
        result
    }

    pub fn permute_unopt<A: Algebra<F>>(&self, state: &mut [A; WIDTH]) {
        for round in (0..self.num_rounds).into_iter() {
            *state = Self::feistel_erf(state, &self.constants[round]);
        }
    }

    pub fn permute_acc<A: Algebra<F>>(&self, state: &mut [A; WIDTH]) {
        let mut x_acc = A::ZERO;
        let mut a = vec![A::ZERO; WIDTH - 1];

        for round in (0..self.num_rounds).into_iter() {
            let x_0 = state[0].clone() + self.constants[round].clone();
            let mut pow = x_0.clone().square();
            match D {
                3 => {
                    pow *= x_0.clone();
                },
                5 => {
                    pow = pow.square().square();
                    pow *= x_0.clone();
                },
                7 => {
                    let pow_2 = x_0.clone();
                    pow = pow.square();
                    pow *= pow_2.clone();
                    pow *= x_0.clone();
                },
                _ => {
                    for _ in 0..D {
                        pow *= x_0.clone();
                    }
                }
            };
            a.rotate_left(1);
            x_acc -= a[0].clone();
            a[0] = pow.clone();
            x_acc += pow.clone();
            state.rotate_left(1);
            state[0] += x_acc.clone();
        }
        for j in (1..(WIDTH - 1)).into_iter() {
            a.rotate_left(1);
            x_acc -= a[0].clone();
            state[j] += x_acc.clone();
        }
    }
}

impl<F, A, const WIDTH: usize, const D: usize> Permutation<[A; WIDTH]>
    for GMiMC<F, WIDTH, D>
where
    F: PrimeField,
    A: Algebra<F>,
{
    fn permute_mut(&self, state: &mut [A; WIDTH]) {
        let mut x_acc = A::ZERO;
        let mut a = [A::ZERO; WIDTH]; // One element larger to outsmart Rust compilers

        let mut offset: usize = 0;
        let mut offset_a: usize = 0;

        for round in (0..self.num_rounds).into_iter() {
            let x_0 = state[offset].clone() + self.constants[round].clone();
            let mut pow = x_0.clone().square();
            match D {
                3 => {
                    pow *= x_0.clone();
                },
                5 => {
                    pow = pow.square().square();
                    pow *= x_0.clone();
                },
                7 => {
                    let pow_2 = x_0.clone();
                    pow = pow.square();
                    pow *= pow_2.clone();
                    pow *= x_0.clone();
                },
                _ => {
                    for _ in 0..D {
                        pow *= x_0.clone();
                    }
                }
            };
            offset_a += 1;
            offset_a = offset_a % (WIDTH - 1);
            x_acc -= a[offset_a].clone();
            a[offset_a] = pow.clone();
            x_acc += pow.clone();
            offset += 1;
            offset = offset % WIDTH;
            state[offset] += x_acc.clone();
        }
        state.rotate_left(offset);
        for j in (1..(WIDTH - 1)).into_iter() {
            offset_a += 1;
            offset_a = offset_a % (WIDTH - 1);
            x_acc -= a[offset_a].clone();
            state[j] += x_acc.clone();
        }
    }
}

#[cfg(test)]
mod gmimc_tests{
    use p3_field::PrimeField;
    use p3_mersenne_31::Mersenne31;
    use p3_symmetric::Permutation;
    use rand::{RngExt, SeedableRng};
    use rand::rngs::SmallRng;
    use rand::distr::{Distribution, StandardUniform};

    use crate::GMiMC;

    fn gmimc_permutation_consistency_test<F, const WIDTH: usize, const ROUNDS: usize>()
    where
        F: PrimeField,
        StandardUniform: Distribution<F>, 
    {
        let mut rng = SmallRng::seed_from_u64(42);

        let gmimc_3 = GMiMC::<F, WIDTH, 3>::new_from_rng(ROUNDS, &mut rng);
        let gmimc_5 = GMiMC::<F, WIDTH, 5>::new_from_rng(ROUNDS, &mut rng);
        let gmimc_7 = GMiMC::<F, WIDTH, 7>::new_from_rng(ROUNDS, &mut rng);

        let mut input_1 = [F::ZERO; WIDTH];
        for i in 0..WIDTH {
            let val: F = rng.sample(StandardUniform);
            input_1[i] += val.clone();
        }
        let mut input_2 = input_1.clone();

        gmimc_3.permute_acc(&mut input_1);
        gmimc_3.permute_unopt(&mut input_2);
        for (a, b) in input_1.iter().zip(input_2.iter()) {
            assert_eq!(a, b, "The accumulated and unoptimized outputs do not match for d = 2!\n{:?}\n{:?}", input_1, input_2);
        }

        gmimc_3.permute_mut(&mut input_1);
        gmimc_3.permute_unopt(&mut input_2);
        for (a, b) in input_1.iter().zip(input_2.iter()) {
            assert_eq!(a, b, "The optimized and unoptimized outputs do not match for d = 2!\n{:?}\n{:?}", input_1, input_2);
        }

        gmimc_5.permute_acc(&mut input_1);
        gmimc_5.permute_unopt(&mut input_2);
        for (a, b) in input_1.iter().zip(input_2.iter()) {
            assert_eq!(a, b, "The accumulated and unoptimized outputs do not match for d = 4!\n{:?}\n{:?}", input_1, input_2);
        }

        gmimc_5.permute_mut(&mut input_1);
        gmimc_5.permute_unopt(&mut input_2);
        for (a, b) in input_1.iter().zip(input_2.iter()) {
            assert_eq!(a, b, "The optimized and unoptimized outputs do not match for d = 4!\n{:?}\n{:?}", input_1, input_2);
        }

        gmimc_7.permute_acc(&mut input_1);
        gmimc_7.permute_unopt(&mut input_2);
        for (a, b) in input_1.iter().zip(input_2.iter()) {
            assert_eq!(a, b, "The accumulated and unoptimized outputs do not match for d = 8!\n{:?}\n{:?}", input_1, input_2);
        }

        gmimc_7.permute_mut(&mut input_1);
        gmimc_7.permute_unopt(&mut input_2);
        for (a, b) in input_1.iter().zip(input_2.iter()) {
            assert_eq!(a, b, "The optimized and unoptimized outputs do not match for d = 8!\n{:?}\n{:?}", input_1, input_2);
        }
    }

    #[test]
    fn gmimc_permutation_consistency_test_16() {
        gmimc_permutation_consistency_test::<Mersenne31, 16, 100>();
    }

    #[test]
    fn gmimc_permutation_consistency_test_24() {
        gmimc_permutation_consistency_test::<Mersenne31, 24, 100>();
    }
}
