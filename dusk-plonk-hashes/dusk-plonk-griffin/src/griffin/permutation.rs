//! This module contains an implementation of the `Griffin` permutation
//! algorithm specifically designed to work outside of Rank 1 Constraint Systems
//! (R1CS) or other custom Constraint Systems such as Add/Mul/Custom plonk
//! gate-circuits.
//!
//! The inputs of the permutation function have to be explicitly over the
//! scalar Field of the bls12_381 curve so over a modulus
//! `p = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001`.

use crate::griffin::{ROUNDS, WIDTH};

/// Griffin permutation struct operating in a plonk-circuit.
#[cfg(feature = "zk")]
pub(crate) mod gadget;

/// Griffin permutation struct operating on [`BlsScalar`].
pub(crate) mod scalar;

/// Defines the Griffin permutation algorithm.
pub(crate) trait Griffin<T> {
    fn add_round_constants(&mut self, round: usize, state: &mut [T; WIDTH]);

    fn quintic_s_box(&mut self, value: &mut T);

    fn quintic_s_box_inv(&mut self, value: &mut T);

    fn horst(&mut self, round: usize, state: &mut [T; WIDTH]);

    fn mul_matrix(&mut self, state: &mut [T; WIDTH]);

    fn affine(&mut self, round: usize, state: &mut [T; WIDTH]);

    fn round_function(&mut self, round: usize, state: &mut [T; WIDTH]) {
        self.horst(round, state);

        self.affine(round, state);
    }

    /// Applies one Griffin permutation.
    fn perm(&mut self, state: &mut [T; WIDTH]) {
        self.mul_matrix(state);
        for round in 0..ROUNDS {
            self.round_function(round, state);
        }
    }
}
