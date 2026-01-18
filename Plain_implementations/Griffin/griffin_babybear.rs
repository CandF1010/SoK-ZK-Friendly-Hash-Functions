use super::griffin_params::GriffinParams;
use crate::fields::babybear::BabyBear;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = BabyBear;

lazy_static! {
    pub static ref GRIFFIN_BABYBEAR_16_PARAMS: Arc<GriffinParams<Scalar>> =
        Arc::new(GriffinParams::from_spec(16, 128));
    pub static ref GRIFFIN_BABYBEAR_24_PARAMS: Arc<GriffinParams<Scalar>> =
        Arc::new(GriffinParams::from_spec(24, 128));
}
