#![no_std]

extern crate alloc;

use alloc::vec::Vec;

use p3_field::{Algebra, PrimeField};
use p3_symmetric::Permutation;
use rand::distr::{Distribution, StandardUniform};
use rand::{Rng, RngExt};


#[derive(Clone, Debug)]
pub struct GMiMC2<F, const WIDTH: usize, const D: usize> {
    num_rounds: usize,
    constants: Vec<F>,
}

impl<F, const WIDTH: usize, const D: usize> GMiMC2<F, WIDTH, D>
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
    fn matrix_io<A: Algebra<F>>(state: &[A; WIDTH]) -> [A; WIDTH] {   
        let mut result = state.clone();
        match WIDTH {
            4 => {
                // do nothing
            },
            8 => {
                // circulant(1, 0, 0, 0, 2, 0, 0, 0)
                result[0] += state[4].clone().double();
                result[1] += state[5].clone().double();
                result[2] += state[6].clone().double();
                result[3] += state[7].clone().double();
                result[4] += state[0].clone().double();
                result[5] += state[1].clone().double();
                result[6] += state[2].clone().double();
                result[7] += state[3].clone().double();
            },
            12 => {         
                // circulant(1, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 0)
                result[0] += state[3].clone().double() + state[5].clone().double();
                result[1] += state[4].clone().double() + state[6].clone().double();
                result[2] += state[5].clone().double() + state[7].clone().double();
                result[3] += state[6].clone().double() + state[8].clone().double();
                result[4] += state[7].clone().double() + state[9].clone().double();
                result[5] += state[8].clone().double() + state[10].clone().double();
                result[6] += state[9].clone().double() + state[11].clone().double();
                result[7] += state[10].clone().double() + state[0].clone().double();
                result[8] += state[11].clone().double() + state[1].clone().double();
                result[9] += state[0].clone().double() + state[2].clone().double();
                result[10] += state[1].clone().double() + state[3].clone().double();
                result[11] += state[2].clone().double() + state[4].clone().double();
            },
            16 => {
                // circulant(1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0)
                result[0] += state[8].clone().double();
                result[1] += state[9].clone().double();
                result[2] += state[10].clone().double();
                result[3] += state[11].clone().double();
                result[4] += state[12].clone().double();
                result[5] += state[13].clone().double();
                result[6] += state[14].clone().double();
                result[7] += state[15].clone().double();
                result[8] += state[0].clone().double();
                result[9] += state[1].clone().double();
                result[10] += state[2].clone().double();
                result[11] += state[3].clone().double();
                result[12] += state[4].clone().double();
                result[13] += state[5].clone().double();
                result[14] += state[6].clone().double();
                result[15] += state[7].clone().double();
            },
            24 => {
                // circulant(1, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
                result[0] += state[7].clone().double() + state[11].clone().double();
                result[1] += state[8].clone().double() + state[12].clone().double();
                result[2] += state[9].clone().double() + state[13].clone().double();
                result[3] += state[10].clone().double() + state[14].clone().double();
                result[4] += state[11].clone().double() + state[15].clone().double();
                result[5] += state[12].clone().double() + state[16].clone().double();
                result[6] += state[13].clone().double() + state[17].clone().double();
                result[7] += state[14].clone().double() + state[18].clone().double();
                result[8] += state[15].clone().double() + state[19].clone().double();
                result[9] += state[16].clone().double() + state[20].clone().double();
                result[10] += state[17].clone().double() + state[21].clone().double();
                result[11] += state[18].clone().double() + state[22].clone().double();
                result[12] += state[19].clone().double() + state[23].clone().double();
                result[13] += state[20].clone().double() + state[0].clone().double();
                result[14] += state[21].clone().double() + state[1].clone().double();
                result[15] += state[22].clone().double() + state[2].clone().double();
                result[16] += state[23].clone().double() + state[3].clone().double();
                result[17] += state[0].clone().double() + state[4].clone().double();
                result[18] += state[1].clone().double() + state[5].clone().double();
                result[19] += state[2].clone().double() + state[6].clone().double();
                result[20] += state[3].clone().double() + state[7].clone().double();
                result[21] += state[4].clone().double() + state[8].clone().double();
                result[22] += state[5].clone().double() + state[9].clone().double();
                result[23] += state[6].clone().double() + state[10].clone().double();
            },
            _ => {
                // circulant(1, 0, ..., 0, 2, 0, .., 0)
                let mut state_cpy = state.clone();
                state_cpy.rotate_right(WIDTH / 2);
                for i in (0..WIDTH).into_iter() {
                    result[i] += state_cpy[i].clone().double();
                }
            }
        }

        result
    }

    pub fn permute_unopt<A: Algebra<F>>(&self, state: &mut [A; WIDTH]) {
        *state = Self::matrix_io(&state);

        for round in (0..self.num_rounds).into_iter() {
            let x_0 = state[0].clone() + self.constants[round].clone();
            let mut pow = x_0.clone().square();
            match D {
                3 => {
                    pow *= x_0.clone();
                },
                4 => {
                    pow = pow.square();
                },
                5 => {
                    pow = pow.square() * x_0.clone();
                },
                8 => {
                    pow = pow.square().square();
                },
                _ => {
                    for _ in 0..(D - 2) {
                        pow *= x_0.clone();
                    }
                }
            };
            for i in (0..(WIDTH - 1)).into_iter() {
                state[i] = state[i + 1].clone() + pow.clone();
            }
            state[WIDTH - 1] = x_0.clone();
        }

        *state = Self::matrix_io(&state);
    }
}

impl<F, A, const WIDTH: usize, const D: usize> Permutation<[A; WIDTH]>
    for GMiMC2<F, WIDTH, D>
where
    F: PrimeField,
    A: Algebra<F>,
{
    fn permute_mut(&self, state: &mut [A; WIDTH]) {
        *state = Self::matrix_io(&state);

        let mut x_acc = A::ZERO;
        let mut acc = [A::ZERO; WIDTH]; // One element larger to outsmart Rust compilers

        let mut offset: usize = 0;
        let mut offset_acc: usize = 0;

        // Rounds 1..(R - 1)
        state[offset] += self.constants[0].clone();
        for round in (0..(self.num_rounds - 1)).into_iter() {
            // Compute power fuunction
            let mut pow = state[offset].clone().square();
            match D {
                3 => {
                    pow *= state[offset].clone();
                },
                4 => {
                    pow = pow.square();
                },
                5 => {
                    pow = pow.square() * state[offset].clone();
                },
                8 => {
                    pow = pow.square().square();
                },
                _ => {
                    for _ in 0..(D - 2) {
                        pow *= state[offset].clone();
                    }
                }
            };
            // Rotate accumulator via offset
            offset_acc += 1;
            offset_acc = offset_acc % (WIDTH - 1);
            // Update accumulators
            x_acc -= acc[offset_acc].clone();
            acc[offset_acc] = pow.clone();
            x_acc += pow.clone();
            // Rotate state via offset
            offset += 1;
            offset = offset % WIDTH;
            // Update state
            state[offset] += x_acc.clone() + self.constants[round + 1];
        }
        // Final Round
        let mut pow = state[offset].clone().square();
        match D {
            3 => {
                pow *= state[offset].clone();
            },
            4 => {
                pow = pow.square();
            },
            5 => {
                pow = pow.square() * state[offset].clone();
            },
            8 => {
                pow = pow.square().square();
            },
            _ => {
                for _ in 0..(D - 2) {
                    pow *= state[offset].clone();
                }
            }
        };
        // Rotate accumulator via offset
        offset_acc += 1;
        offset_acc = offset_acc % (WIDTH - 1);
        // Update accumulators
        x_acc -= acc[offset_acc].clone();
        acc[offset_acc] = pow.clone();
        x_acc += pow.clone();
        // Rotate state via offset
        offset += 1;
        offset = offset % WIDTH;
        // Update state
        state[offset] += x_acc.clone();

        // Rotate state with offset
        state.rotate_left(offset);
        // Final Additions
        for j in (1..(WIDTH - 1)).into_iter() {
            // Rotate accumulator via offset
            offset_acc += 1;
            offset_acc = offset_acc % (WIDTH - 1);
            // Update accumulator
            x_acc -= acc[offset_acc].clone();
            // Update state
            state[j] += x_acc.clone();
        }

        *state = Self::matrix_io(&state);
    }
}

#[cfg(test)]
mod gmimc_2_tests{
    use p3_baby_bear::BabyBear;
    use p3_field::PrimeField;
    use p3_goldilocks::Goldilocks;
    use p3_koala_bear::KoalaBear;
    use p3_mersenne_31::Mersenne31;
    use p3_symmetric::Permutation;
    use rand::{RngExt, SeedableRng};
    use rand::rngs::SmallRng;
    use rand::distr::{Distribution, StandardUniform};

    use crate::GMiMC2;

    fn gmimc_2_permutation_consistency_test<F, const WIDTH: usize, const ROUNDS: usize>()
    where
        F: PrimeField,
        StandardUniform: Distribution<F>, 
    {
        let mut rng = SmallRng::seed_from_u64(42);

        let gmimc_2 = GMiMC2::<F, WIDTH, 2>::new_from_rng(ROUNDS, &mut rng);
        let gmimc_3 = GMiMC2::<F, WIDTH, 3>::new_from_rng(ROUNDS, &mut rng);
        let gmimc_4 = GMiMC2::<F, WIDTH, 4>::new_from_rng(ROUNDS, &mut rng);
        let gmimc_5 = GMiMC2::<F, WIDTH, 5>::new_from_rng(ROUNDS, &mut rng);
        let gmimc_8 = GMiMC2::<F, WIDTH, 8>::new_from_rng(ROUNDS, &mut rng);

        let mut input_1 = [F::ZERO; WIDTH];
        for i in 0..WIDTH {
            let val: F = rng.sample(StandardUniform);
            input_1[i] += val.clone();
        }
        let mut input_2 = input_1.clone();

        gmimc_2.permute_mut(&mut input_1);
        gmimc_2.permute_unopt(&mut input_2);
        for (a, b) in input_1.iter().zip(input_2.iter()) {
            assert_eq!(a, b, "The optimized and unoptimized outputs do not match for d = 2!\n{:?}\n{:?}", input_1, input_2);
        }

        gmimc_3.permute_mut(&mut input_1);
        gmimc_3.permute_unopt(&mut input_2);
        for (a, b) in input_1.iter().zip(input_2.iter()) {
            assert_eq!(a, b, "The optimized and unoptimized outputs do not match for d = 3!\n{:?}\n{:?}", input_1, input_2);
        }

        gmimc_4.permute_mut(&mut input_1);
        gmimc_4.permute_unopt(&mut input_2);
        for (a, b) in input_1.iter().zip(input_2.iter()) {
            assert_eq!(a, b, "The optimized and unoptimized outputs do not match for d = 4!\n{:?}\n{:?}", input_1, input_2);
        }

        gmimc_5.permute_mut(&mut input_1);
        gmimc_5.permute_unopt(&mut input_2);
        for (a, b) in input_1.iter().zip(input_2.iter()) {
            assert_eq!(a, b, "The optimized and unoptimized outputs do not match for d = 5!\n{:?}\n{:?}", input_1, input_2);
        }

        gmimc_8.permute_mut(&mut input_1);
        gmimc_8.permute_unopt(&mut input_2);
        for (a, b) in input_1.iter().zip(input_2.iter()) {
            assert_eq!(a, b, "The optimized and unoptimized outputs do not match for d = 8!\n{:?}\n{:?}", input_1, input_2);
        }
    }

    #[test]
    fn gmimc_2_permutation_consistency_test_goldilocks_8() {
        gmimc_2_permutation_consistency_test::<Goldilocks, 8, 100>();
    }

    #[test]
    fn gmimc_2_permutation_consistency_test_goldilocks_12() {
        gmimc_2_permutation_consistency_test::<Goldilocks, 12, 100>();
    }

    #[test]
    fn gmimc_2_permutation_consistency_test_mersenne_31_16() {
        gmimc_2_permutation_consistency_test::<Mersenne31, 16, 100>();
    }

    #[test]
    fn gmimc_2_permutation_consistency_test_baby_bear_16() {
        gmimc_2_permutation_consistency_test::<BabyBear, 16, 100>();
    }

    #[test]
    fn gmimc_2_permutation_consistency_test_koala_bear_16() {
        gmimc_2_permutation_consistency_test::<KoalaBear, 16, 100>();
    }

    #[test]
    fn gmimc_2_permutation_consistency_test_mersenne_31_24() {
        gmimc_2_permutation_consistency_test::<Mersenne31, 24, 100>();
    }

    #[test]
    fn gmimc_2_permutation_consistency_test_baby_bear_24() {
        gmimc_2_permutation_consistency_test::<BabyBear, 24, 100>();
    }

    #[test]
    fn gmimc_2_permutation_consistency_test_koala_bear_24() {
        gmimc_2_permutation_consistency_test::<KoalaBear, 24, 100>();
    }
}
