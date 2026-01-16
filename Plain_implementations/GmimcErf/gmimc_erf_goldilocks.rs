use super::gmimc_erf_params::GmimcErfParams;
use crate::fields::goldilocks::Goldilocks;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Goldilocks;

lazy_static! {
    pub static ref GMIMC_ERF_GOLDILOCKS_8_PARAMS: Arc<GmimcErfParams<Scalar>> =
        Arc::new(GmimcErfParams::from_spec(8));
    pub static ref GMIMC_ERF_GOLDILOCKS_12_PARAMS: Arc<GmimcErfParams<Scalar>> =
        Arc::new(GmimcErfParams::from_spec(12));
}
