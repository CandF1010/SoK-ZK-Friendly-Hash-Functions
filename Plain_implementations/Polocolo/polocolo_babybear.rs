use super::polocolo_params::PolocoloParams;
use crate::fields::babybear::BabyBear;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = BabyBear;

const LABEL: &str = "BabyBear";

lazy_static! {
    pub static ref POLOCOLO_BABYBEAR_3_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(3, LABEL));
    pub static ref POLOCOLO_BABYBEAR_4_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(4, LABEL));
    pub static ref POLOCOLO_BABYBEAR_6_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(6, LABEL));
    pub static ref POLOCOLO_BABYBEAR_8_PARAMS: Arc<PolocoloParams<Scalar>> =
        Arc::new(PolocoloParams::from_table(8, LABEL));
}
