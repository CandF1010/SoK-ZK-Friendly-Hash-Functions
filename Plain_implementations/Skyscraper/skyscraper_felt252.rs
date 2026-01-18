use super::skyscraper_params::SkyscraperParams;
use crate::fields::felt252::Felt252;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Felt252;

lazy_static! {
    pub static ref SKYSCRAPER_FELT252_1_PARAMS: Arc<SkyscraperParams<Scalar>> =
        Arc::new(SkyscraperParams::new(1, 5));
}
