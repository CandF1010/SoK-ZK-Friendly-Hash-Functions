use super::reinforced_concrete_params::ReinforcedConcreteParams;
use crate::fields::koalabear::KoalaBear;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = KoalaBear;

lazy_static! {
    pub static ref RC_KOALABEAR_PARAMS: Arc<ReinforcedConcreteParams<Scalar>> =
        Arc::new(ReinforcedConcreteParams::new_auto());
}
