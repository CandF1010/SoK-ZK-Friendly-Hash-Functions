//! This module contains an implementation of the `GMiMC2` permutation
//! algorithm specifically designed to work outside of Rank 1 Constraint Systems
//! (R1CS) or other custom Constraint Systems such as Add/Mul/Custom plonk
//! gate-circuits.
//!
//! The inputs of the permutation function have to be explicitly over the
//! scalar Field of the bls12_381 curve so over a modulus
//! `p = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001`.

use crate::gmimc2::{ROUNDS, WIDTH};

/// GMiMC2 permutation struct operating in a plonk-circuit.
#[cfg(feature = "zk")]
pub(crate) mod gadget;

/// GMiMC2 permutation struct operating on [`BlsScalar`].
pub(crate) mod scalar;

/// Defines the GMiMC2 permutation algorithm.
pub(crate) trait GMiMC2<T> {
    /// Computes `(input + c) ^ 5 (mod p)`
    ///
    /// The modulo depends on the input you use. In our case the modulo is done
    /// in respect of the scalar field of the bls12_381 curve
    /// `p = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001`.
    fn octic_s_box(&mut self, value: &T) -> T;

    fn first_round_function(&mut self, state: &mut [T; WIDTH]);

    fn round_function(&mut self, round: usize, state: &mut [T; WIDTH]);

    fn final_round_function(&mut self, state: &mut [T; WIDTH]);

    /// Applies one GMiMC2 permutation.
    fn perm(&mut self, state: &mut [T; WIDTH]) {
        self.first_round_function(state);
        for round in 1..(ROUNDS - 1) {
            self.round_function(round, state);
        }
        self.final_round_function(state);
    }

    /*
    /// Applies one GMiMC2 permutation.
    #[allow(dead_code)]
    fn perm_opt(&mut self, state: &mut [T; WIDTH]);
    */
}
