use super::neptune_params::NeptuneParams;
use crate::fields::babybear::BabyBear;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = BabyBear;

lazy_static! {
    pub static ref NEPTUNE_BABYBEAR_16_PARAMS: Arc<NeptuneParams<Scalar>> =
        Arc::new(NeptuneParams::from_spec(16, 128));
    pub static ref NEPTUNE_BABYBEAR_24_PARAMS: Arc<NeptuneParams<Scalar>> =
        Arc::new(NeptuneParams::from_spec(24, 128));
}
