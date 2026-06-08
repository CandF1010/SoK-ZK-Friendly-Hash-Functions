// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

//! This module contains an implementation of the `Grendel` permutation
//! algorithm specifically designed to work outside of Rank 1 Constraint Systems
//! (R1CS) or other custom Constraint Systems such as Add/Mul/Custom plonk
//! gate-circuits.
//!
//! The inputs of the permutation function have to be explicitly over the
//! scalar Field of the bls12_381 curve so over a modulus
//! `p = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001`.

use crate::grendel::{ROUNDS, WIDTH};

/// Grendel permutation struct operating in a plonk-circuit.
#[cfg(feature = "zk")]
pub(crate) mod gadget;

/// Grendel permutation struct operating on [`BlsScalar`].
pub(crate) mod scalar;

/// Defines the Grendel permutation algorithm.
///
/// This permutation is a 3-step process that:
/// - Applies half of the `FULL_ROUNDS` (which can be understood as linear ops).
/// - Applies the `PARTIAL_ROUNDS` (which can be understood as non-linear ops).
/// - Applies the other half of the `FULL_ROUNDS`.
///
/// This structure allows to minimize the number of non-linear ops while
/// maintaining the security.
pub(crate) trait Grendel<T> {
    #[allow(unused)]
    const ROUNDS: usize = ROUNDS;

    fn add_round_constants(&mut self, round: usize, state: &mut [T; WIDTH]);

    fn s_box(&mut self, value: &mut T);

    fn mul_matrix(&mut self, state: &mut [T; WIDTH]);

    fn affine(&mut self, round: usize, state: &mut [T; WIDTH]);

    fn apply_round(&mut self, round: usize, state: &mut [T; WIDTH]) {
        state.iter_mut().for_each(|w| self.s_box(w));

        self.affine(2 * round, state);
    }

    fn perm(&mut self, state: &mut [T; WIDTH]) {
        for round in 0..ROUNDS {
            self.apply_round(round, state);
        }
    }
}
