use super::griffin_params::GriffinParams;
use crate::fields::goldilocks::Goldilocks;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Goldilocks;

lazy_static! {
    pub static ref GRIFFIN_GOLDILOCKS_8_PARAMS: Arc<GriffinParams<Scalar>> =
        Arc::new(GriffinParams::from_spec(8, 128));
    pub static ref GRIFFIN_GOLDILOCKS_12_PARAMS: Arc<GriffinParams<Scalar>> =
        Arc::new(GriffinParams::from_spec(12, 128));
}
