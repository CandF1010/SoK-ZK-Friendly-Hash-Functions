use super::neptune_params::NeptuneParams;
use crate::fields::goldilocks::Goldilocks;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Goldilocks;

lazy_static! {
    pub static ref NEPTUNE_GOLDILOCKS_8_PARAMS: Arc<NeptuneParams<Scalar>> =
        Arc::new(NeptuneParams::from_spec(8, 128));
    pub static ref NEPTUNE_GOLDILOCKS_12_PARAMS: Arc<NeptuneParams<Scalar>> =
        Arc::new(NeptuneParams::from_spec(12, 128));
}
