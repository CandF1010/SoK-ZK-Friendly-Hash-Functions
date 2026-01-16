use super::poseidon_params::PoseidonParams;
use crate::fields::koalabear::KoalaBear;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = KoalaBear;

lazy_static! {
    pub static ref POSEIDON_KOALABEAR_16_PARAMS: Arc<PoseidonParams<Scalar>> =
        Arc::new(PoseidonParams::from_grain(16, 3, 8, 20));
    pub static ref POSEIDON_KOALABEAR_24_PARAMS: Arc<PoseidonParams<Scalar>> =
        Arc::new(PoseidonParams::from_grain(24, 3, 8, 23));
}
