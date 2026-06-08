mod matrix;
mod permutation;
mod constants;

use constants::{CONSTANTS_ALPHA_BETA, CONSTANTS_AFFINE};

use matrix::MATRIX;

pub const ROUNDS: usize = 17;

/// The amount of field elements that fit into the griffin permutation container
pub const WIDTH: usize = 4;

pub const FIVE_INV: [u64; 4] = [
                                3689348813023923405,
                                2413663763415232921,
                                16233882818423549954,
                                3341406743785779740,
                            ];

#[cfg(feature = "zk")]
pub(crate) use permutation::gadget::GadgetPermutation;
pub(crate) use permutation::scalar::ScalarPermutation;
