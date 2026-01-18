use super::skyscraper_params::SkyscraperParams;
use crate::fields::goldilocks::Goldilocks;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Goldilocks;

lazy_static! {
    pub static ref SKYSCRAPER_GOLDILOCKS_1_PARAMS: Arc<SkyscraperParams<Scalar>> =
        Arc::new(SkyscraperParams::new(1, 5));
}
