use super::gmimc_erf_params::GmimcErfParams;
use crate::fields::bn254::Bn254;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Bn254;

lazy_static! {
    pub static ref GMIMC_ERF_BN254_2_PARAMS: Arc<GmimcErfParams<Scalar>> =
        Arc::new(GmimcErfParams::from_spec(2));
    pub static ref GMIMC_ERF_BN254_3_PARAMS: Arc<GmimcErfParams<Scalar>> =
        Arc::new(GmimcErfParams::from_spec(3));
}
