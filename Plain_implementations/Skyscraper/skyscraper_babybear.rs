use super::skyscraper_params::SkyscraperParams;
use crate::fields::babybear::BabyBear;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = BabyBear;

lazy_static! {
    pub static ref SKYSCRAPER_BABYBEAR_1_PARAMS: Arc<SkyscraperParams<Scalar>> =
        Arc::new(SkyscraperParams::new(1, 5));
}
