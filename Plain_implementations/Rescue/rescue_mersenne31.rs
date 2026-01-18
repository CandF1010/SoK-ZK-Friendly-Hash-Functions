use super::rescue_params::RescueParams;
use crate::fields::mersenne31::Mersenne31;
use crate::fields::PrimeField;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Mersenne31;

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
    pub static ref RESCUE_MERSENNE31_16_PARAMS: Arc<RescueParams<Scalar>> = {
        let capacity = capacity_for_security(16);
        Arc::new(RescueParams::from_spec(16, capacity, SECURITY_LEVEL))
    };
    pub static ref RESCUE_MERSENNE31_24_PARAMS: Arc<RescueParams<Scalar>> = {
        let capacity = capacity_for_security(24);
        Arc::new(RescueParams::from_spec(24, capacity, SECURITY_LEVEL))
    };
}
