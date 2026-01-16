use super::poseidon_params::PoseidonParams;
use crate::fields::babybear::BabyBear;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = BabyBear;

lazy_static! {
    pub static ref POSEIDON_BABYBEAR_16_PARAMS: Arc<PoseidonParams<Scalar>> =
        Arc::new(PoseidonParams::from_grain(16, 7, 8, 13));
    pub static ref POSEIDON_BABYBEAR_24_PARAMS: Arc<PoseidonParams<Scalar>> =
        Arc::new(PoseidonParams::from_grain(24, 7, 8, 21));
}
