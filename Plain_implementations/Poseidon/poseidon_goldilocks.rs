use super::poseidon_params::PoseidonParams;
use crate::fields::goldilocks::Goldilocks;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Goldilocks;

lazy_static! {
    pub static ref POSEIDON_GOLDILOCKS_8_PARAMS: Arc<PoseidonParams<Scalar>> =
        Arc::new(PoseidonParams::from_grain(8, 7, 8, 22));
    pub static ref POSEIDON_GOLDILOCKS_12_PARAMS: Arc<PoseidonParams<Scalar>> =
        Arc::new(PoseidonParams::from_grain(12, 7, 8, 22));
}
