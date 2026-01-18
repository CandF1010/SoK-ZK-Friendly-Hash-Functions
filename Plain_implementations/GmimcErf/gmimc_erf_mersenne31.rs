use super::gmimc_erf_params::GmimcErfParams;
use crate::fields::mersenne31::Mersenne31;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Mersenne31;

lazy_static! {
    pub static ref GMIMC_ERF_MERSENNE31_16_PARAMS: Arc<GmimcErfParams<Scalar>> =
        Arc::new(GmimcErfParams::from_spec(16));
    pub static ref GMIMC_ERF_MERSENNE31_24_PARAMS: Arc<GmimcErfParams<Scalar>> =
        Arc::new(GmimcErfParams::from_spec(24));
}
