use super::rescue_params::RescueParams;
use crate::fields::bn254::Bn254;
use crate::fields::PrimeField;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Bn254;

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
    pub static ref RESCUE_BN254_2_PARAMS: Arc<RescueParams<Scalar>> = {
        let capacity = capacity_for_security(2);
        Arc::new(RescueParams::from_spec(2, capacity, SECURITY_LEVEL))
    };
    pub static ref RESCUE_BN254_3_PARAMS: Arc<RescueParams<Scalar>> = {
        let capacity = capacity_for_security(3);
        Arc::new(RescueParams::from_spec(3, capacity, SECURITY_LEVEL))
    };
}
