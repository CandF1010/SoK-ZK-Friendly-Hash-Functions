mod mds_matrix;
mod permutation;
mod round_constants;

use mds_matrix::MDS_MATRIX;
use round_constants::ROUND_CONSTANTS;

const ROUNDS: usize = 11;

/// The amount of field elements that fit into the poseidon2 permutation container
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
