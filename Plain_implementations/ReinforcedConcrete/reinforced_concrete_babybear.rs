use super::reinforced_concrete_params::ReinforcedConcreteParams;
use crate::fields::babybear::BabyBear;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = BabyBear;

lazy_static! {
    pub static ref RC_BABYBEAR_PARAMS: Arc<ReinforcedConcreteParams<Scalar>> =
        Arc::new(ReinforcedConcreteParams::new_auto());
}
