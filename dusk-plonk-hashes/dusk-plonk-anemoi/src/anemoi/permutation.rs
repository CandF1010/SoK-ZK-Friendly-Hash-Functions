//! This module contains an implementation of the `Anemoi` permutation
//! algorithm specifically designed to work outside of Rank 1 Constraint Systems
//! (R1CS) or other custom Constraint Systems such as Add/Mul/Custom plonk
//! gate-circuits.
//!
//! The inputs of the permutation function have to be explicitly over the
//! scalar Field of the bls12_381 curve so over a modulus
//! `p = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001`.

use dusk_curves::bls12_381::BlsScalar;
use crate::anemoi::{ROUNDS, WIDTH};

/// Anemoi permutation struct operating in a plonk-circuit.
#[cfg(feature = "zk")]
pub(crate) mod gadget;

/// Anemoi permutation struct operating on [`BlsScalar`].
pub(crate) mod scalar;

/// Defines the Anemoi permutation algorithm.
pub(crate) trait Anemoi<T> {
    fn add_round_constants(&mut self, round: usize, state: &mut [T; WIDTH]);

    fn open_flystel(&mut self, state: &mut [BlsScalar]);

    #[allow(unused)]
    fn closed_flystel(&mut self, state_y_v: &[T], state_x_u: &[T]);

    fn linear_layer(&mut self, state: &mut [T; WIDTH]);

    // In Anemoi specification, constants are added before matrix.
    // We swap the to reduce Plonk constraints.
    fn affine_layer(&mut self, round: usize, state: &mut [T; WIDTH]);

    fn round_function(&mut self, round: usize, state: &mut [T; WIDTH]);

    /// Applies one Anemoi permutation.
    fn perm(&mut self, state: &mut [T; WIDTH]) {
        for round in 0..ROUNDS {
            self.round_function(round, state);
        }
        self.linear_layer(state);
    }
}
