use super::reinforced_concrete_params::ReinforcedConcreteParams;
use crate::fields::mersenne31::Mersenne31;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Mersenne31;

lazy_static! {
    pub static ref RC_MERSENNE31_PARAMS: Arc<ReinforcedConcreteParams<Scalar>> =
        Arc::new(ReinforcedConcreteParams::new_auto());
}
