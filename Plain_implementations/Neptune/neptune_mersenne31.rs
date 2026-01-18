use super::neptune_params::NeptuneParams;
use crate::fields::mersenne31::Mersenne31;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Mersenne31;

lazy_static! {
    pub static ref NEPTUNE_MERSENNE31_16_PARAMS: Arc<NeptuneParams<Scalar>> =
        Arc::new(NeptuneParams::from_spec(16, 128));
    pub static ref NEPTUNE_MERSENNE31_24_PARAMS: Arc<NeptuneParams<Scalar>> =
        Arc::new(NeptuneParams::from_spec(24, 128));
}
