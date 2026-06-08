mod matrix;
mod permutation;
mod constants;

use constants::{CONSTANTS_G, CONSTANTS_H, CONSTANTS_AFFINE};

use matrix::MATRIX;

pub const ROUNDS: usize = 17;

/// The amount of field elements that fit into the arion permutation container
pub const WIDTH: usize = 4;

// d_2 = 257
pub const D2_INV: [u64; 4] = [
                                8469711284772863745,
                                1214928404647555091,
                                15849274830579433833,
                                3867970841541904563
                            ];

#[cfg(feature = "zk")]
pub(crate) use permutation::gadget::GadgetPermutation;
pub(crate) use permutation::scalar::ScalarPermutation;
