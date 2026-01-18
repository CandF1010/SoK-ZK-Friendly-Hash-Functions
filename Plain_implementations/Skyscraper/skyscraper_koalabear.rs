use super::skyscraper_params::SkyscraperParams;
use crate::fields::koalabear::KoalaBear;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = KoalaBear;

lazy_static! {
    pub static ref SKYSCRAPER_KOALABEAR_1_PARAMS: Arc<SkyscraperParams<Scalar>> =
        Arc::new(SkyscraperParams::new(1, 5));
}
