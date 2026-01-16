use super::anemoi_params::AnemoiParams;
use crate::fields::goldilocks::Goldilocks;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Goldilocks;

lazy_static! {
    pub static ref ANEMOI_GOLDILOCKS_2_PARAMS: Arc<AnemoiParams<Scalar>> =
        Arc::new(AnemoiParams::from_spec(1, 128));
    pub static ref ANEMOI_GOLDILOCKS_4_PARAMS: Arc<AnemoiParams<Scalar>> =
        Arc::new(AnemoiParams::from_spec(2, 128));
}
