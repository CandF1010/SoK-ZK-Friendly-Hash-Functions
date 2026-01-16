use super::gmimc_erf_params::GmimcErfParams;
use crate::fields::babybear::BabyBear;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = BabyBear;

lazy_static! {
    pub static ref GMIMC_ERF_BABYBEAR_16_PARAMS: Arc<GmimcErfParams<Scalar>> =
        Arc::new(GmimcErfParams::from_spec(16));
    pub static ref GMIMC_ERF_BABYBEAR_24_PARAMS: Arc<GmimcErfParams<Scalar>> =
        Arc::new(GmimcErfParams::from_spec(24));
}
