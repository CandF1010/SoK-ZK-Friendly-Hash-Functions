mod matrices;
mod permutation;
mod constants;

use matrices::{MATRIX_EXTERNAL_1, MATRIX_EXTERNAL_2, MATRIX_INTERNAL};
use constants::{ALPHA, GAMMA, ROUND_CONSTANTS};

const FULL_ROUNDS_INITIAL: usize = 4;
const FULL_ROUNDS_FINAL: usize = 2;

const PARTIAL_ROUNDS: usize = 68;

/// The amount of field elements that fit into the poseidon2 permutation container
pub const WIDTH: usize = 4;

#[cfg(feature = "zk")]
pub(crate) use permutation::gadget::GadgetPermutation;
pub(crate) use permutation::scalar::ScalarPermutation;
