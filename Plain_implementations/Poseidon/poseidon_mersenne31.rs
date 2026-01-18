use super::poseidon_params::PoseidonParams;
use crate::fields::mersenne31::Mersenne31;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Mersenne31;

lazy_static! {
    pub static ref POSEIDON_MERSENNE31_16_PARAMS: Arc<PoseidonParams<Scalar>> =
        Arc::new(PoseidonParams::from_grain(16, 5, 8, 14));
    pub static ref POSEIDON_MERSENNE31_24_PARAMS: Arc<PoseidonParams<Scalar>> =
        Arc::new(PoseidonParams::from_grain(24, 5, 8, 22));
}
