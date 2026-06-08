mod permutation;
mod constants;

use constants::{G, G_INV, G_1, G_2, G_SQU_G_1, G_SQU_2G_1, ROUND_CONSTANTS};

pub const ROUNDS: usize = 16;

/// The amount of field elements that fit into the anemoi permutation container
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
