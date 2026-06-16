use dusk_curves::bls12_381::BlsScalar;
use dusk_safe::Safe;

use super::Neptune;
use crate::neptune::{ALPHA, GAMMA, MATRIX_EXTERNAL_1, MATRIX_EXTERNAL_2, MATRIX_INTERNAL, ROUND_CONSTANTS, WIDTH};

/// An implementation of the [`Permutation`] for `BlsScalar` as input values.
#[derive(Default)]
pub(crate) struct ScalarPermutation();

impl ScalarPermutation {
    /// Constructs a new `ScalarPermutation`.
    pub fn new() -> Self {
        Self()
    }
}

impl Safe<BlsScalar, WIDTH> for ScalarPermutation {
    fn permute(&mut self, state: &mut [BlsScalar; WIDTH]) {
        self.perm(state);
    }

    fn tag(&mut self, input: &[u8]) -> BlsScalar {
        BlsScalar::hash_to_scalar(input)
    }

    fn add(&mut self, right: &BlsScalar, left: &BlsScalar) -> BlsScalar {
        right + left
    }
}

impl Neptune<BlsScalar> for ScalarPermutation {
    fn add_round_constants(
        &mut self,
        round: usize,
        state: &mut [BlsScalar; WIDTH],
    ) {
        state
            .iter_mut()
            .enumerate()
            .for_each(|(i, s)| *s += ROUND_CONSTANTS[round][i]);
    }

    fn quintic_s_box(&mut self, value: &mut BlsScalar) {
        *value = value.square().square() * *value;
    }

    fn lm_s_box(&mut self, state: &mut [BlsScalar]) {
        // x - y
        let diff_1 = state[0] - state[1];
        // x - 2 * y
        let diff_2 = diff_1 - state[1];
        // (x - y)^2
        let squ_1 = diff_1.square();
        // GAMMA + ALPHA * (x - 2 * y) - (x - y)^2
        let diff_3 = GAMMA + ALPHA * diff_2 - squ_1;
        // (GAMMA + ALPHA * (x - 2 * y) + (x - y)^2)^2
        let squ_2 = diff_3.square();
        // 2 * x + y
        let sum_1 = state[0].double() + state[1];
        // x + 3 * y
        let sum_2 = state[0] + state[1].double() + state[1];
        
        let mut result = [BlsScalar::zero(); 2];
        result[0] += ALPHA.square() * sum_1 + (ALPHA.double() + ALPHA) * squ_1 + squ_2;
        result[1] += ALPHA.square() * sum_2 + ALPHA.double().double() * squ_1 + squ_2;

        state.copy_from_slice(&result);
    }

    fn mul_matrix_external(&mut self, state: &mut [BlsScalar; WIDTH]) {
        let mut result = [BlsScalar::zero(); WIDTH];

        for i in 0..(WIDTH / 2) {
            for j in 0..(WIDTH / 2) {
                result[2 * i] += MATRIX_EXTERNAL_1[i][j] * state[2 * j];
                result[2 * i + 1] += MATRIX_EXTERNAL_2[i][j] * state[2 * j + 1];
            }
        }

        state.copy_from_slice(&result);
    }

    fn affine_external(&mut self, round: usize, state: &mut [BlsScalar; WIDTH]) {
        self.mul_matrix_external(state);

        self.add_round_constants(round, state);
    }

    fn affine_internal(&mut self, round: usize, state: &mut [BlsScalar; WIDTH]) {
        let mut result = [BlsScalar::zero(); WIDTH];

        for i in 0..WIDTH {
            for j in 0..WIDTH {
                result[i] += MATRIX_INTERNAL[i][j] * state[j];
            }
        }

        self.add_round_constants(round, &mut result);

        state.copy_from_slice(&result);
    }
}

#[cfg(feature = "encryption")]
impl dusk_safe::Encryption<BlsScalar, WIDTH> for ScalarPermutation {
    fn subtract(
        &mut self,
        minuend: &BlsScalar,
        subtrahend: &BlsScalar,
    ) -> BlsScalar {
        minuend - subtrahend
    }

    fn is_equal(&mut self, lhs: &BlsScalar, rhs: &BlsScalar) -> bool {
        lhs == rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neptune_det() {
        let mut x = [BlsScalar::from(17u64); WIDTH];
        let mut y = [BlsScalar::from(17u64); WIDTH];
        let mut z = [BlsScalar::from(19u64); WIDTH];

        ScalarPermutation::new().permute(&mut x);
        ScalarPermutation::new().permute(&mut y);
        ScalarPermutation::new().permute(&mut z);

        assert_eq!(x, y);
        assert_ne!(x, z);
    }
}
