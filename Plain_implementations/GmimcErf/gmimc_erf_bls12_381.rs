use super::gmimc_erf_params::GmimcErfParams;
use crate::fields::bls12_381::Bls12_381;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Bls12_381;

lazy_static! {
    pub static ref GMIMC_ERF_BLS12_381_2_PARAMS: Arc<GmimcErfParams<Scalar>> =
        Arc::new(GmimcErfParams::from_spec(2));
    pub static ref GMIMC_ERF_BLS12_381_3_PARAMS: Arc<GmimcErfParams<Scalar>> =
        Arc::new(GmimcErfParams::from_spec(3));
}
