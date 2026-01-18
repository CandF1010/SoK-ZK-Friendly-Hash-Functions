use super::tip4_params::Tip4Params;
use crate::fields::goldilocks::Goldilocks;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Goldilocks;

lazy_static! {
    pub static ref TIP4_GOLDILOCKS_PARAMS: Arc<Tip4Params<Scalar>> =
        Arc::new(Tip4Params::new_tip4());
    pub static ref TIP4P_GOLDILOCKS_PARAMS: Arc<Tip4Params<Scalar>> =
        Arc::new(Tip4Params::new_tip4p());
}
