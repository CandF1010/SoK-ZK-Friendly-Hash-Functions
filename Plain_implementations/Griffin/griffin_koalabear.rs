use super::griffin_params::GriffinParams;
use crate::fields::koalabear::KoalaBear;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = KoalaBear;

lazy_static! {
    pub static ref GRIFFIN_KOALABEAR_16_PARAMS: Arc<GriffinParams<Scalar>> =
        Arc::new(GriffinParams::from_spec(16, 128));
    pub static ref GRIFFIN_KOALABEAR_24_PARAMS: Arc<GriffinParams<Scalar>> =
        Arc::new(GriffinParams::from_spec(24, 128));
}
