#![no_std]

extern crate alloc;

pub mod feistel;
mod constants;

use alloc::vec::Vec;

pub use feistel::*;
use constants::*;

use p3_field::{Algebra, PrimeField};
use p3_mersenne_31::Mersenne31;
use p3_symmetric::Permutation;
use rand::distr::{Distribution, StandardUniform};
use rand::{Rng, RngExt};

const SUPPORTED_WIDTHS: [usize; 2] = [16, 24];


#[derive(Clone, Debug)]
pub struct PSquareHash<F, const WIDTH: usize> {
    num_rounds: usize,
    num_steps: usize,
    feistel_constants: Vec<F>,
}

impl<F, const WIDTH: usize> PSquareHash<F, WIDTH>
where
    F: PrimeField,
{
    pub fn new(num_rounds: usize, num_steps: usize, feistel_constants: Vec<F>) -> Self {
        assert!(SUPPORTED_WIDTHS.contains(&WIDTH));
        Self {
            num_rounds,
            num_steps,
            feistel_constants,
        }
    }

    pub fn new_from_rng<R: Rng>(num_rounds: usize, num_steps: usize, rng: &mut R) -> Self
    where
        StandardUniform: Distribution<F>,
    {
        let num_constants = WIDTH / 2 * num_rounds * num_steps;
        let feistel_constants = rng
            .sample_iter(StandardUniform)
            .take(num_constants)
            .collect::<Vec<_>>();
        Self {
            num_rounds,
            num_steps,
            feistel_constants,
        }
    }

    pub fn permute_double<A: Algebra<F>>(&self, state: &mut [A; WIDTH]) {
        assert!(self.num_rounds % 2 == 0, "Number of rounds must be even for double round evaluation");
        *state = matrix(state);

        for round in (0..(self.num_rounds * self.num_steps)/2).into_iter() {
            match WIDTH {
                16 => {
                    double_feistel_16(
                        state,
                        &self.feistel_constants[16 * round..16 * (round+1)],
                    );
                }

                24 => {
                    double_feistel_24(
                        state,
                        &self.feistel_constants[24 * round..24 * (round + 1)],
                    );
                }

                _ => {
                    panic!("Unsupported width");
                }
            }
        }

        *state = matrix(state);
    }
}

impl<F, A, const WIDTH: usize> Permutation<[A; WIDTH]>
    for PSquareHash<F, WIDTH>
where
    F: PrimeField,
    A: Algebra<F>,
{
    fn permute_mut(&self, state: &mut [A; WIDTH]) {
        *state = matrix(state);

        for round in (0..(self.num_rounds * self.num_steps)).into_iter() {
            match WIDTH {
                16 => {
                    *state = feistel_16(
                        state,
                        &self.feistel_constants[WIDTH / 2 * round..WIDTH / 2 * (round + 1)],
                    );
                }

                24 => {
                    *state = feistel_24(
                        state,
                        &self.feistel_constants[WIDTH / 2 * round..WIDTH / 2 * (round + 1)],
                    );
                }

                _ => {
                    panic!("Unsupported width");
                }
            }
        }

        *state = matrix(state);
    }
}

pub fn default_m31_psquarehash_16() -> PSquareHash<Mersenne31, 16> {
    PSquareHash::new(4, 13, CONSTANTS_M31_16_4_13.to_vec())
}

pub fn default_m31_psquarehash_24() -> PSquareHash<Mersenne31, 24> {
    PSquareHash::new(4, 13, CONSTANTS_M31_24_4_13.to_vec())
}

#[cfg(test)]
mod psquarehash_tests{
    use p3_baby_bear::BabyBear;
    use p3_field::PrimeCharacteristicRing;
    use p3_mersenne_31::Mersenne31;
    use p3_symmetric::Permutation;
    use rand::{RngExt, SeedableRng};
    use rand::rngs::SmallRng;
    use rand::distr::{Distribution, StandardUniform};

    use crate::{default_m31_psquarehash_16,
                default_m31_psquarehash_24,
                PSquareHash};


    #[test]
    fn test_feistel_16()
        where
        StandardUniform: Distribution<BabyBear>,
    {

        let mut rng = SmallRng::seed_from_u64(42);
        const WIDTH: usize = 16;
        const NUM_ROUNDS: usize = 4;
        const NUM_STEPS: usize = 10;


        let psquarehash = PSquareHash::<BabyBear, WIDTH>::new_from_rng(NUM_ROUNDS, NUM_STEPS, &mut rng);
        let mut input_double = [<BabyBear as PrimeCharacteristicRing>::ZERO; WIDTH];
        for target in input_double.iter_mut() {
            let val: BabyBear = rng.sample(StandardUniform);
            *target = BabyBear::from(val);
        }
        let mut input_single = input_double.clone();

        psquarehash.permute_double(&mut input_double);
        psquarehash.permute_mut(&mut input_single);

        for (a, b) in input_double.iter().zip(input_single.iter()) {
            assert_eq!(a, b, "The double round and single round outputs do not match!");
        }
    }

    #[test]
    fn test_feistel_24()
        where
        StandardUniform: Distribution<BabyBear>,
    {

        let mut rng = SmallRng::seed_from_u64(42);
        const WIDTH: usize = 24;
        const NUM_ROUNDS: usize = 4;
        const NUM_STEPS: usize = 10;



        let psquarehash = PSquareHash::<BabyBear, WIDTH,>::new_from_rng(NUM_ROUNDS, NUM_STEPS, &mut rng);
        let mut input_double = [<BabyBear as PrimeCharacteristicRing>::ZERO; WIDTH];
        for target in input_double.iter_mut() {
            let val: BabyBear = rng.sample(StandardUniform);
            *target = BabyBear::from(val);
        }
        let mut input_single = input_double.clone();

        psquarehash.permute_double(&mut input_double);
        psquarehash.permute_mut(&mut input_single);

        for (a, b) in input_double.iter().zip(input_single.iter()) {
            assert_eq!(a, b, "The double round and single round outputs do not match!\n{:?}\n{:?}", input_double, input_single);
        }
    }

    #[test]
    fn test_vector_16()
    {
        let psquarehash = default_m31_psquarehash_16();
        let input = Mersenne31::new_array([0x78066d6b, 0x68a24eb4, 0x2d12aacd, 0x42bb7df4,
                                                                    0x3a85ecf4, 0x010084b5, 0x28e3f4fb, 0x41514a49,
                                                                    0x0e904f42, 0x0981bfd9, 0x3309b9ac, 0x19f408ff,
                                                                    0x1f3202d0, 0x2ebbcc8c, 0x261b659f, 0x22171a32]);
        let output = Mersenne31::new_array([0x0d31650f, 0x5b324b40, 0x02fb8ac7, 0x555c9139,
                                                                     0x00a60cba, 0x1b61b003, 0x33e1c0ad, 0x48d970a2,
                                                                     0x0876e39d, 0x3a6f9513, 0x36afab87, 0x4d85ef87,
                                                                     0x277b1cee, 0x70debee3, 0x1337395b, 0x35ea5bad]);
        assert_eq!(psquarehash.permute(input.clone()), output);
    }

    #[test]
    fn test_vector_24()
    {
        let psquarehash = default_m31_psquarehash_24();
        let input = Mersenne31::new_array([0x78066d6b, 0x68a24eb4, 0x2d12aacd, 0x42bb7df4,
                                                                    0x3a85ecf4, 0x010084b5, 0x28e3f4fb, 0x41514a49,
                                                                    0x0e904f42, 0x0981bfd9, 0x3309b9ac, 0x19f408ff,
                                                                    0x1f3202d0, 0x2ebbcc8c, 0x261b659f, 0x22171a32,
                                                                    0x2b77fbfb, 0x57d3e692, 0x47dbb2c4, 0x5f803d52,
                                                                    0x7791f988, 0x6988c314, 0x283918dd, 0x32a8ab7b]);
        let output = Mersenne31::new_array([0x2e50ad42, 0x117b015c, 0x0c4610fb, 0x636c99be,
                                                                    0x3b2635cc, 0x15323786, 0x36ba41ac, 0x788cb4d8,
                                                                    0x0f7b751a, 0x7608c969, 0x6ff8eeda, 0x6ed27e30,
                                                                    0x55e7b993, 0x63506d14, 0x42032061, 0x31bbe5f5,
                                                                    0x7179d761, 0x5871965a, 0x16497d76, 0x2237878d,
                                                                    0x637ca7d2, 0x13752294, 0x0831c440, 0x18bf0647]);
        assert_eq!(psquarehash.permute(input.clone()), output);
    }
}
