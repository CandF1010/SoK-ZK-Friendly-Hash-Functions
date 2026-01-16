use super::poseidon2_params::Poseidon2Params;
use crate::fields::bn254::Bn254;
use crate::fields::FieldElement;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Bn254;

const D: u64 = 5;
const ROUNDS_F: usize = 8;
const ROUNDS_P: usize = 56;

fn from_hex(s: &str) -> Option<Scalar> {
    Bn254::from_hex(s)
}

fn internal_diag_2() -> Vec<Scalar> {
    vec![Scalar::from_u64(1), Scalar::from_u64(2)]
}

fn build_internal_matrix(diag_m_1: &[Scalar]) -> Vec<Vec<Scalar>> {
    let t = diag_m_1.len();
    let mut mat = vec![vec![Scalar::one(); t]; t];
    for i in 0..t {
        mat[i][i].add_assign(&diag_m_1[i]);
    }
    mat
}

lazy_static! {
    pub static ref MAT_DIAG3_M_1: Vec<Scalar> = vec![
        from_hex("0x0000000000000000000000000000000000000000000000000000000000000001").unwrap(),
        from_hex("0x0000000000000000000000000000000000000000000000000000000000000001").unwrap(),
        from_hex("0x0000000000000000000000000000000000000000000000000000000000000002").unwrap(),
    ];

    pub static ref POSEIDON2_BN254_2_PARAMS: Arc<Poseidon2Params<Scalar>> = {
        let diag = internal_diag_2();
        let mat_internal = build_internal_matrix(&diag);
        Arc::new(Poseidon2Params::from_grain(
            2,
            D,
            ROUNDS_F,
            ROUNDS_P,
            &diag,
            &mat_internal,
        ))
    };

    pub static ref POSEIDON2_BN254_3_PARAMS: Arc<Poseidon2Params<Scalar>> = {
        let mat_internal = build_internal_matrix(&MAT_DIAG3_M_1);
        Arc::new(Poseidon2Params::from_grain(
            3,
            D,
            ROUNDS_F,
            ROUNDS_P,
            &MAT_DIAG3_M_1,
            &mat_internal,
        ))
    };
}
