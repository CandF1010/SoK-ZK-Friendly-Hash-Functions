use super::anemoi_params::AnemoiParams;
use crate::fields::babybear::BabyBear;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = BabyBear;

lazy_static! {
    pub static ref ANEMOI_BABYBEAR_2_PARAMS: Arc<AnemoiParams<Scalar>> =
        Arc::new(AnemoiParams::from_spec(1, 128));
    pub static ref ANEMOI_BABYBEAR_4_PARAMS: Arc<AnemoiParams<Scalar>> =
        Arc::new(AnemoiParams::from_spec(2, 128));
}
