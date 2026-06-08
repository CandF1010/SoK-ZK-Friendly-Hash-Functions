mod mds_matrix;
mod permutation;
mod round_constants;

use mds_matrix::MDS_MATRIX;
use round_constants::ROUND_CONSTANTS;

const ROUNDS: usize = 23;

/// The amount of field elements that fit into the poseidon2 permutation container
pub const WIDTH: usize = 4;

pub const Q_1_2: [u64; 4] = [
                                9223372034707292160,
                                12240451741123816959,
                                1845609449319885826,
                                4176758429732224676,
                            ];

#[cfg(feature = "zk")]
pub(crate) use permutation::gadget::GadgetPermutation;
pub(crate) use permutation::scalar::ScalarPermutation;
