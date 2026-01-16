use super::gmimc_erf_params::GmimcErfParams;
use crate::fields::felt252::Felt252;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Felt252;

lazy_static! {
    pub static ref GMIMC_ERF_FELT252_2_PARAMS: Arc<GmimcErfParams<Scalar>> =
        Arc::new(GmimcErfParams::from_spec(2));
    pub static ref GMIMC_ERF_FELT252_3_PARAMS: Arc<GmimcErfParams<Scalar>> =
        Arc::new(GmimcErfParams::from_spec(3));
}
