use super::poseidon_params::PoseidonParams;
use crate::fields::felt252::Felt252;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Felt252;

lazy_static! {
    pub static ref POSEIDON_FELT252_2_PARAMS: Arc<PoseidonParams<Scalar>> =
        Arc::new(PoseidonParams::from_grain(2, 3, 8, 56));
    pub static ref POSEIDON_FELT252_3_PARAMS: Arc<PoseidonParams<Scalar>> =
        Arc::new(PoseidonParams::from_grain(3, 3, 8, 57));
}
