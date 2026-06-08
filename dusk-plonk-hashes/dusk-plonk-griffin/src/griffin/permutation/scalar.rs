use dusk_curves::bls12_381::BlsScalar;
use dusk_safe::Safe;

use super::Griffin;
use crate::griffin::{CONSTANTS_AFFINE, CONSTANTS_ALPHA_BETA, FIVE_INV, MATRIX, WIDTH};

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

impl Griffin<BlsScalar> for ScalarPermutation {
    fn add_round_constants(
        &mut self,
        round: usize,
        state: &mut [BlsScalar; WIDTH],
    ) {
        state
            .iter_mut()
            .enumerate()
            .for_each(|(i, s)| *s += CONSTANTS_AFFINE[round][i]);
    }

    fn quintic_s_box(&mut self, value: &mut BlsScalar) {
        *value = value.square().square() * *value;
    }

    fn quintic_s_box_inv(&mut self, value: &mut BlsScalar) {
        *value = value.pow_vartime(&FIVE_INV);
    }

    fn mul_matrix(&mut self, state: &mut [BlsScalar; WIDTH]) {
        let mut result = [BlsScalar::zero(); WIDTH];

        for i in 0..WIDTH {
            for j in 0..WIDTH {
                result[i] += MATRIX[i][j] * state[j];
            }
        }

        state.copy_from_slice(&result);
    }

    fn affine(&mut self, round: usize, state: &mut [BlsScalar; WIDTH]) {
        self.mul_matrix(state);
        self.add_round_constants(round, state);
    }

    fn horst(&mut self, round: usize, state: &mut [BlsScalar; WIDTH]) {
        self.quintic_s_box_inv(&mut state[0]);
        
        self.quintic_s_box(&mut state[1]);

        let l_2 = state[0] + state[1];
        let squ_2 = l_2.square() + CONSTANTS_ALPHA_BETA[round][0][0] * l_2 + CONSTANTS_ALPHA_BETA[round][0][1];
        state[2] *= squ_2;

        for i in 3..WIDTH {
            let l_i = BlsScalar::from_raw([(i - 1) as u64, 0, 0, 0]) * state[0] + state[1] + state[i - 1];
            let squ_i = l_i.square() + CONSTANTS_ALPHA_BETA[round][i - 2][0] * l_i + CONSTANTS_ALPHA_BETA[round][i - 2][1];
            state[i] *= squ_i;
        }
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
    fn griffin_det() {
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
