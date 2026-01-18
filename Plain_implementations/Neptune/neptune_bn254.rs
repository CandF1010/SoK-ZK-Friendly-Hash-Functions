use super::neptune_params::NeptuneParams;
use crate::fields::bn254::Bn254;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Bn254;

lazy_static! {
    pub static ref NEPTUNE_BN254_2_PARAMS: Arc<NeptuneParams<Scalar>> =
        Arc::new(NeptuneParams::from_spec(2, 128));
    pub static ref NEPTUNE_BN254_4_PARAMS: Arc<NeptuneParams<Scalar>> =
        Arc::new(NeptuneParams::from_spec(4, 128));
}
