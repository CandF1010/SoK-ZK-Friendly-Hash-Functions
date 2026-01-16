use super::anemoi_params::AnemoiParams;
use crate::fields::bls12_381::Bls12_381;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Bls12_381;

lazy_static! {
    pub static ref ANEMOI_BLS12_381_2_PARAMS: Arc<AnemoiParams<Scalar>> =
        Arc::new(AnemoiParams::from_spec(1, 128));
    pub static ref ANEMOI_BLS12_381_4_PARAMS: Arc<AnemoiParams<Scalar>> =
        Arc::new(AnemoiParams::from_spec(2, 128));
}
