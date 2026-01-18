use super::neptune_params::NeptuneParams;
use crate::fields::bls12_381::Bls12_381;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Bls12_381;

lazy_static! {
    pub static ref NEPTUNE_BLS12_381_2_PARAMS: Arc<NeptuneParams<Scalar>> =
        Arc::new(NeptuneParams::from_spec(2, 128));
    pub static ref NEPTUNE_BLS12_381_4_PARAMS: Arc<NeptuneParams<Scalar>> =
        Arc::new(NeptuneParams::from_spec(4, 128));
}
