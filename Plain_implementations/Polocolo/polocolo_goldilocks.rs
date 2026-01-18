use super::polocolo_params::PolocoloParams;
use crate::fields::goldilocks::Goldilocks;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Goldilocks;

const LABEL: &str = "Goldilocks";

lazy_static! {
    pub static ref POLOCOLO_GOLDILOCKS_3_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(3, LABEL));
    pub static ref POLOCOLO_GOLDILOCKS_4_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(4, LABEL));
    pub static ref POLOCOLO_GOLDILOCKS_6_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(6, LABEL));
    pub static ref POLOCOLO_GOLDILOCKS_8_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(8, LABEL));
}
