use super::poseidon_params::PoseidonParams;
use crate::fields::bls12_381::Bls12_381;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Bls12_381;

lazy_static! {
    pub static ref POSEIDON_BLS12_381_2_PARAMS: Arc<PoseidonParams<Scalar>> =
        Arc::new(PoseidonParams::from_grain(2, 5, 8, 56));
    pub static ref POSEIDON_BLS12_381_3_PARAMS: Arc<PoseidonParams<Scalar>> =
        Arc::new(PoseidonParams::from_grain(3, 5, 8, 57));
}
