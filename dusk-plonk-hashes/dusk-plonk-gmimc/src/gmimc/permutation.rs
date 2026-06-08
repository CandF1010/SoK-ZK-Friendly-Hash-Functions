//! This module contains an implementation of the `GMiMC` permutation
//! algorithm specifically designed to work outside of Rank 1 Constraint Systems
//! (R1CS) or other custom Constraint Systems such as Add/Mul/Custom plonk
//! gate-circuits.
//!
//! The inputs of the permutation function have to be explicitly over the
//! scalar Field of the bls12_381 curve so over a modulus
//! `p = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001`.

use crate::gmimc::{ROUNDS, WIDTH};

/// GMiMC permutation struct operating in a plonk-circuit.
#[cfg(feature = "zk")]
pub(crate) mod gadget;

/// GMiMC permutation struct operating on [`BlsScalar`].
pub(crate) mod scalar;

/// Defines the GMiMC permutation algorithm.
pub(crate) trait GMiMC<T> {
    /// Computes `(input + c) ^ 5 (mod p)`
    ///
    /// The modulo depends on the input you use. In our case the modulo is done
    /// in respect of the scalar field of the bls12_381 curve
    /// `p = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001`.
    fn quintic_s_box(&mut self, round: usize, value: &T) -> T;

    fn round_function(&mut self, round: usize, state: &mut [T; WIDTH]);

    /// Applies one GMiMC permutation.
    fn perm(&mut self, state: &mut [T; WIDTH]) {
        for round in 0..ROUNDS {
            self.round_function(round, state);
        }
    }

    /*
    /// Applies one GMiMC permutation.
    #[allow(dead_code)]
    fn perm_opt(&mut self, state: &mut [T; WIDTH]);
    */
}
