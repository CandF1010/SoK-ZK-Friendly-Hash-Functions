use super::anemoi_params::AnemoiParams;
use crate::fields::bn254::Bn254;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Bn254;

lazy_static! {
    pub static ref ANEMOI_BN254_2_PARAMS: Arc<AnemoiParams<Scalar>> =
        Arc::new(AnemoiParams::from_spec(1, 128));
    pub static ref ANEMOI_BN254_4_PARAMS: Arc<AnemoiParams<Scalar>> =
        Arc::new(AnemoiParams::from_spec(2, 128));
}
