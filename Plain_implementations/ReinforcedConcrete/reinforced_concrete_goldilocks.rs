use super::reinforced_concrete_params::ReinforcedConcreteParams;
use crate::fields::goldilocks::Goldilocks;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Goldilocks;

lazy_static! {
    pub static ref RC_GOLDILOCKS_PARAMS: Arc<ReinforcedConcreteParams<Scalar>> =
        Arc::new(ReinforcedConcreteParams::new_auto());
}
