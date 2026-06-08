// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

//! This module contains an implementation of the `Poseidon2` permutation
//! algorithm specifically designed to work outside of Rank 1 Constraint Systems
//! (R1CS) or other custom Constraint Systems such as Add/Mul/Custom plonk
//! gate-circuits.
//!
//! The inputs of the permutation function have to be explicitly over the
//! scalar Field of the bls12_381 curve so over a modulus
//! `p = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001`.

use crate::poseidon2::{FULL_ROUNDS, PARTIAL_ROUNDS, WIDTH};

/// Poseidon2 permutation struct operating in a plonk-circuit.
#[cfg(feature = "zk")]
pub(crate) mod gadget;

/// Poseidon2 permutation struct operating on [`BlsScalar`].
pub(crate) mod scalar;

/// Defines the Poseidon2 permutation algorithm.
///
/// This permutation is a 3-step process that:
/// - Applies half of the `FULL_ROUNDS` (which can be understood as linear ops).
/// - Applies the `PARTIAL_ROUNDS` (which can be understood as non-linear ops).
/// - Applies the other half of the `FULL_ROUNDS`.
///
/// This structure allows to minimize the number of non-linear ops while
/// maintaining the security.
pub(crate) trait Poseidon2<T> {
    #[allow(unused)]
    const ROUNDS: usize = FULL_ROUNDS + PARTIAL_ROUNDS;

    /// Add round constants to the state.
    ///
    /// This constants addition, also known as `ARC`, is used to reach
    /// `Confusion and Diffusion` properties for the algorithm.
    ///
    /// Basically it allows to destroy any connection between the inputs and the
    /// outputs of the function.
    fn add_round_constants(&mut self, round: usize, state: &mut [T; WIDTH]);

    /// Computes `input ^ 5 (mod p)`
    ///
    /// The modulo depends on the input you use. In our case the modulo is done
    /// in respect of the scalar field of the bls12_381 curve
    /// `p = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001`.
    fn quintic_s_box(&mut self, value: &mut T);

    /// Multiply the external MDS matrix with the state.
    fn mul_matrix_external(&mut self, state: &mut [T; WIDTH]);

    fn affine_external(&mut self, round: usize, state: &mut [T; WIDTH]);

    fn affine_internal(&mut self, round: usize, state: &mut [T; WIDTH]); 

    fn apply_partial_round(&mut self, round: usize, state: &mut [T; WIDTH]) {
        self.quintic_s_box(&mut state[0]);

        self.affine_internal(round, state);
    }

    fn apply_full_round(&mut self, round: usize, state: &mut [T; WIDTH]) {
        state.iter_mut().for_each(|w| self.quintic_s_box(w));

        self.affine_external(round, state);
    }

    fn apply_full_round_final(&mut self, state: &mut [T; WIDTH]) {
        state.iter_mut().for_each(|w| self.quintic_s_box(w));

        self.mul_matrix_external(state);
    }

    fn perm(&mut self, state: &mut [T; WIDTH]) {
        let mut ctr = 0;

        self.affine_external(ctr, state);
        ctr += 1;
        // Apply R_f full rounds
        for _ in 0..(FULL_ROUNDS / 2) {
            self.apply_full_round(ctr, state);
            ctr += 1;
        }

        // Apply R_P partial rounds
        for _ in 0..PARTIAL_ROUNDS {
            self.apply_partial_round(ctr, state);
            ctr += 1;
        }

        // Apply R_f full rounds
        for _ in 0..(FULL_ROUNDS / 2 - 1) {
            self.apply_full_round(ctr, state);
            ctr += 1;
        }
        self.apply_full_round_final(state);
    }
}
