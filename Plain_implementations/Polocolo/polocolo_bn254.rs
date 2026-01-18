use super::polocolo_params::PolocoloParams;
use crate::fields::bn254::Bn254;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Bn254;

const LABEL: &str = "BN254";

lazy_static! {
    pub static ref POLOCOLO_BN254_3_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(3, LABEL));
    pub static ref POLOCOLO_BN254_4_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(4, LABEL));
    pub static ref POLOCOLO_BN254_6_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(6, LABEL));
    pub static ref POLOCOLO_BN254_8_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(8, LABEL));
}
