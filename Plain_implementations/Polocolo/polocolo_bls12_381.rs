use super::polocolo_params::PolocoloParams;
use crate::fields::bls12_381::Bls12_381;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Bls12_381;

const LABEL: &str = "BLS12-381";

lazy_static! {
    pub static ref POLOCOLO_BLS12_381_3_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(3, LABEL));
    pub static ref POLOCOLO_BLS12_381_4_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(4, LABEL));
    pub static ref POLOCOLO_BLS12_381_6_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(6, LABEL));
    pub static ref POLOCOLO_BLS12_381_8_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(8, LABEL));
}
