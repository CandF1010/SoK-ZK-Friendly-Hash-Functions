use dusk_curves::bls12_381::BlsScalar;
use dusk_safe::Safe;

use super::Arion;
use crate::arion::{CONSTANTS_G, CONSTANTS_H, CONSTANTS_AFFINE, D2_INV, MATRIX, WIDTH};

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

impl Arion<BlsScalar> for ScalarPermutation {
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

    fn d_2_inv_s_box(&mut self, value: &mut BlsScalar) {
        *value = value.pow_vartime(&D2_INV);
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

    fn gtds(&mut self, round: usize, state: &mut [BlsScalar; WIDTH]) {
        let state_cpy = state.clone();
        let mut sigma = BlsScalar::zero();

        self.d_2_inv_s_box(&mut state[WIDTH - 1]);
        
        sigma += state_cpy[WIDTH - 1];
        sigma += state[WIDTH - 1];
        for i in (0..(WIDTH - 2)).rev() {
            self.quintic_s_box(&mut state[i]);
            let sigma_squ = sigma.square();
            let g = sigma_squ + CONSTANTS_G[round][i][0] * sigma + CONSTANTS_G[round][i][1];
            let h = sigma_squ + CONSTANTS_H[round][i] * sigma;
            state[i] *= g;
            state[i] += h;
            if i > 0 {
                sigma += state_cpy[i] + state[i];
            }
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
    fn arion_det() {
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
