// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

//! Implementation of [Hades252](https://eprint.iacr.org/2019/458.pdf)
//! permutation algorithm over the Bls12-381 Scalar field.
//!
//! ## Parameters
//!
//! - `p = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001`
//! - Permutation container `WIDTH` is 5 field elements
//! - 8 full rounds: 4 full rounds at the beginning and 4 full rounds at the
//!   end, and each full round has `WIDTH` quintic S-Boxes.
//! - 60 partial rounds: each partial round has `WIDTH - 1` identity function
//!   and one quintic S-Box.
//! - 340 round constants which are generated using [this algorithm](https://extgit.iaik.tugraz.at/krypto/hadesmimc/blob/master/code/calc_round_numbers.py)
//! - The MDS matrix is a cauchy matrix, the method used to generate it, is
//!   noted in section "Concrete Instantiations Poseidon and Starkad"

mod mds_matrix;
mod permutation;
mod round_constants;

use mds_matrix::MDS_MATRIX;
use round_constants::ROUND_CONSTANTS;

const FULL_ROUNDS: usize = 8;

const PARTIAL_ROUNDS: usize = 56;

/// The amount of field elements that fit into the hades permutation container
pub const WIDTH: usize = 4;

#[cfg(feature = "zk")]
pub(crate) use permutation::gadget::GadgetPermutation;
pub(crate) use permutation::scalar::ScalarPermutation;
