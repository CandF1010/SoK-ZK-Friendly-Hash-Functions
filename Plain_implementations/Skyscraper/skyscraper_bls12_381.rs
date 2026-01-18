use super::skyscraper_params::SkyscraperParams;
use crate::fields::bls12_381::Bls12_381;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Bls12_381;

lazy_static! {
    pub static ref SKYSCRAPER_BLS12_381_1_PARAMS: Arc<SkyscraperParams<Scalar>> =
        Arc::new(SkyscraperParams::new(1, 5));
    pub static ref SKYSCRAPER_BLS12_381_2_PARAMS: Arc<SkyscraperParams<Scalar>> =
        Arc::new(SkyscraperParams::new(2, 5));
    pub static ref SKYSCRAPER_BLS12_381_3_PARAMS: Arc<SkyscraperParams<Scalar>> =
        Arc::new(SkyscraperParams::new(3, 2));
}
