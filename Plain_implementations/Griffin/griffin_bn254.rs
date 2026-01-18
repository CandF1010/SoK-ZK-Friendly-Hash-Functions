use super::griffin_params::GriffinParams;
use crate::fields::bn254::Bn254;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Bn254;

lazy_static! {
    pub static ref GRIFFIN_BN254_3_PARAMS: Arc<GriffinParams<Scalar>> =
        Arc::new(GriffinParams::from_spec(3, 128));
    pub static ref GRIFFIN_BN254_4_PARAMS: Arc<GriffinParams<Scalar>> =
        Arc::new(GriffinParams::from_spec(4, 128));
}
