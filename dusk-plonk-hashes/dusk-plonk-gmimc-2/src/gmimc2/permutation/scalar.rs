use dusk_curves::bls12_381::BlsScalar;
use dusk_safe::Safe;

use super::GMiMC2;
use crate::gmimc2::{CONSTANTS, WIDTH};

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

impl GMiMC2<BlsScalar> for ScalarPermutation {
    fn octic_s_box(&mut self, value: &BlsScalar) -> BlsScalar {
        value.clone().square().square().square()
    }

    fn first_round_function(&mut self, state: &mut [BlsScalar; WIDTH]) {
        let sum = state[0] + CONSTANTS[0];
        let pow = self.octic_s_box(&sum);
        let mut result = [BlsScalar::zero(); WIDTH];
        result[0] += state[1] + pow + CONSTANTS[1];
        for i in (1..(WIDTH - 1)).into_iter() {
            result[i] += state[i + 1] + pow;
        }
        result[WIDTH - 1] += sum;
        state.copy_from_slice(&result);
    }

    fn round_function(&mut self, round: usize, state: &mut [BlsScalar; WIDTH]) {
        let pow = self.octic_s_box(&state[0]);
        let mut result = [BlsScalar::zero(); WIDTH];
        result[0] += state[1] + pow + CONSTANTS[round + 1];
        for i in (1..(WIDTH - 1)).into_iter() {
            result[i] += state[i + 1] + pow;
        }
        result[WIDTH - 1] += state[0];
        state.copy_from_slice(&result);
    }

    fn final_round_function(&mut self, state: &mut [BlsScalar; WIDTH]) {
        let pow = self.octic_s_box(&state[0]);
        let mut result = [BlsScalar::zero(); WIDTH];
        for i in (0..(WIDTH - 1)).into_iter() {
            result[i] += state[i + 1] + pow;
        }
        result[WIDTH - 1] += state[0];
        state.copy_from_slice(&result);
    }
    
    /*
    fn perm_opt(&mut self, state: &mut [BlsScalar; WIDTH]) {
        let mut current_state = state.clone();
        let mut acc = BlsScalar::zero();
        let mut acc_queue = [BlsScalar::zero(); WIDTH - 1];
        current_state[0] += CONSTANTS[0];
        for round in (0..(ROUNDS - 1)).into_iter() {
            let pow  = self.quintic_s_box(&current_state[0]);
            acc_queue.rotate_left(1);
            acc -= acc_queue[0];
            acc_queue[0] = pow.clone();
            acc += pow;

            current_state.rotate_left(1);
            current_state[0] += &acc + CONSTANTS[round + 1];
        }
        // Final round
        let pow  = self.quintic_s_box(&current_state[0]);
        acc_queue.rotate_left(1);
        acc -= acc_queue[0];
        acc_queue[0] = pow.clone();
        acc += pow;

        current_state.rotate_left(1);
        current_state[0] += &acc;

        // final adds
        for i in (1..(WIDTH - 1)).into_iter() {
            acc_queue.rotate_left(1);
            acc -= acc_queue[0];
            current_state[i] += acc;
        }
        
        state.copy_from_slice(&current_state);
    }
    */
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
    fn gmimc_det() {
        let mut x = [BlsScalar::from(17u64); WIDTH];
        let mut y = [BlsScalar::from(17u64); WIDTH];
        let mut z = [BlsScalar::from(19u64); WIDTH];

        ScalarPermutation::new().permute(&mut x);
        ScalarPermutation::new().permute(&mut y);
        ScalarPermutation::new().permute(&mut z);

        assert_eq!(x, y);
        assert_ne!(x, z);
    }

    /*
    #[test]
    fn gmimc_opt() {
        let mut x = [BlsScalar::from(17u64); WIDTH];
        let mut y = [BlsScalar::from(17u64); WIDTH];

        ScalarPermutation::new().perm(&mut x);
        ScalarPermutation::new().perm_opt(&mut y);

        assert_eq!(x, y);
    }
    */
}
