use super::poseidon_params::PoseidonParams;
use crate::fields::bn254::Bn254;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Bn254;

lazy_static! {
    pub static ref POSEIDON_BN254_2_PARAMS: Arc<PoseidonParams<Scalar>> =
        Arc::new(PoseidonParams::from_grain(2, 5, 8, 56));
    pub static ref POSEIDON_BN254_3_PARAMS: Arc<PoseidonParams<Scalar>> =
        Arc::new(PoseidonParams::from_grain(3, 5, 8, 57));
}
