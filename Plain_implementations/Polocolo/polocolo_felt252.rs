use super::polocolo_params::PolocoloParams;
use crate::fields::felt252::Felt252;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Felt252;

const LABEL: &str = "Felt252";

lazy_static! {
    pub static ref POLOCOLO_FELT252_3_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(3, LABEL));
    pub static ref POLOCOLO_FELT252_4_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(4, LABEL));
    pub static ref POLOCOLO_FELT252_6_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(6, LABEL));
    pub static ref POLOCOLO_FELT252_8_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(8, LABEL));
}
