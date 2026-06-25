use dusk_curves::bls12_381::BlsScalar;
use dusk_safe::Safe;

use super::Anemoi;
use crate::anemoi::{FIVE_INV, G, G_1, G_2, G_INV, G_SQU_1, G_SQU_2G_1, G_SQU_G_1, ROUND_CONSTANTS, WIDTH};

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

impl Anemoi<BlsScalar> for ScalarPermutation {
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

    fn open_flystel(&mut self, state: &mut [BlsScalar]) {
        let mut feistel_1 = state[1].square();
        feistel_1 *= G;
        feistel_1 += G_INV;
        state[0] -= feistel_1;
        //
        let feistel_2 = state[0].pow_vartime(&FIVE_INV);
        state[1] -= feistel_2; 
        //
        let mut feistel_3 = state[1].square();
        feistel_3 *= G;
        state[0] += feistel_3;
    }

    #[allow(unused)]
    fn closed_flystel(&mut self, state_x: &[BlsScalar], state_y: &[BlsScalar]) {
        // Do nothing
    }

    fn linear_layer(&mut self, state: &mut [BlsScalar; WIDTH]) {
        // Anemoi matrix for WITDH = 4 is given by
        // 2 + g,        1 + g,       0,            0
        // 0,            0,           g^2 + 2g + 1, g^2 + g + 1
        // g^2 + 2g + 1, g^2 + g + 1, 0,          , 0
        // 0,            0,         , 2 + g       , 1 + g
        let mut result = [BlsScalar::zero(); WIDTH];
        result[0] += G_2 * state[0] + G_1 * state[1];
        result[1] += G_SQU_2G_1 * state[2] + G_SQU_G_1 * state[3];
        result[2] += G_SQU_2G_1 * state[0] + G_SQU_G_1 * state[1];
        result[3] += G_2 * state[2] + G_1 * state[3];

        state.copy_from_slice(&result);
    }

    fn linear_layer_final(&mut self, state: &mut [BlsScalar; WIDTH]) {
        // M_x
        // 1,   g
        // g,   g^2 + 1
        //
        // M_y
        // g,   g^2 + 1
        // 1,   g
        let mut result = [BlsScalar::zero(); WIDTH];
        result[0] += state[0] + G * state[1];
        result[1] += G * state[0] + G_SQU_1 * state[1];
        result[2] += G * state[2] + G_SQU_1 * state[3];
        result[3] += state[2] + G * state[3];

        state.copy_from_slice(&result);
    }

    fn affine_layer(&mut self, round: usize, state: &mut [BlsScalar; WIDTH]) {
        self.linear_layer(state);
        self.add_round_constants(round, state);
    }
    
    fn round_function(&mut self, round: usize, state: &mut [BlsScalar; WIDTH]) {
        self.affine_layer(round, state);
        for i in 0..(WIDTH / 2){
            self.open_flystel(&mut state[(2 * i)..(2 * i + 2)]);
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
    fn anemoi_det() {
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
