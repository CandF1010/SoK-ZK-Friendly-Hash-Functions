use super::rescue_params::RescueParams;
use crate::fields::goldilocks::Goldilocks;
use crate::fields::PrimeField;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Goldilocks;

const SECURITY_LEVEL: usize = 128;

fn capacity_for_security(t: usize) -> usize {
    let bits = Scalar::modulus().bits() as usize;
    let target = 2 * SECURITY_LEVEL;
    let mut capacity = (target + bits - 1) / bits;
    if capacity >= t {
        capacity = t - 1;
    }
    capacity
}

lazy_static! {
    pub static ref RESCUE_GOLDILOCKS_8_PARAMS: Arc<RescueParams<Scalar>> = {
        let capacity = capacity_for_security(8);
        Arc::new(RescueParams::from_spec(8, capacity, SECURITY_LEVEL))
    };
    pub static ref RESCUE_GOLDILOCKS_12_PARAMS: Arc<RescueParams<Scalar>> = {
        let capacity = capacity_for_security(12);
        Arc::new(RescueParams::from_spec(12, capacity, SECURITY_LEVEL))
    };
}
