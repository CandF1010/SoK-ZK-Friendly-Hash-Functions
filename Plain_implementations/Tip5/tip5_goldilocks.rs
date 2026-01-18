use super::tip5_params::Tip5Params;
use crate::fields::goldilocks::Goldilocks;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Goldilocks;

lazy_static! {
    pub static ref TIP5_GOLDILOCKS_PARAMS: Arc<Tip5Params<Scalar>> =
        Arc::new(Tip5Params::new());
}
