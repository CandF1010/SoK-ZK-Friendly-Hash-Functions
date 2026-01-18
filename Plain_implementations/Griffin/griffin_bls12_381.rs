use super::griffin_params::GriffinParams;
use crate::fields::bls12_381::Bls12_381;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Bls12_381;

lazy_static! {
    pub static ref GRIFFIN_BLS12_381_3_PARAMS: Arc<GriffinParams<Scalar>> =
        Arc::new(GriffinParams::from_spec(3, 128));
    pub static ref GRIFFIN_BLS12_381_4_PARAMS: Arc<GriffinParams<Scalar>> =
        Arc::new(GriffinParams::from_spec(4, 128));
}
