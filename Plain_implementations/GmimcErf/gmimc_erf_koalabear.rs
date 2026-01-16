use super::gmimc_erf_params::GmimcErfParams;
use crate::fields::koalabear::KoalaBear;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = KoalaBear;

lazy_static! {
    pub static ref GMIMC_ERF_KOALABEAR_16_PARAMS: Arc<GmimcErfParams<Scalar>> =
        Arc::new(GmimcErfParams::from_spec(16));
    pub static ref GMIMC_ERF_KOALABEAR_24_PARAMS: Arc<GmimcErfParams<Scalar>> =
        Arc::new(GmimcErfParams::from_spec(24));
}
