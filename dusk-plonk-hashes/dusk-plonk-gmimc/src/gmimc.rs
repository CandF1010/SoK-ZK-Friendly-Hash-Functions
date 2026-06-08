mod permutation;
mod round_constants;

use round_constants::CONSTANTS;

pub const ROUNDS: usize = 228;

/// The amount of field elements that fit into the gmimc permutation container
pub const WIDTH: usize = 4;

#[cfg(feature = "zk")]
pub(crate) use permutation::gadget::GadgetPermutation;
pub(crate) use permutation::scalar::ScalarPermutation;
