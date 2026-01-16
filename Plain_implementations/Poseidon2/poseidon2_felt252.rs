use super::poseidon2_params::Poseidon2Params;
use crate::fields::felt252::Felt252;
use crate::fields::FieldElement;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Felt252;

fn internal_diag_2() -> Vec<Scalar> {
    vec![Scalar::from_u64(1), Scalar::from_u64(2)]
}

fn internal_diag_3() -> Vec<Scalar> {
    vec![
        Scalar::from_u64(1),
        Scalar::from_u64(1),
        Scalar::from_u64(2),
    ]
}

fn build_internal_matrix(diag: &[Scalar]) -> Vec<Vec<Scalar>> {
    let t = diag.len();
    let mut mat = vec![vec![Scalar::from_u64(1); t]; t];
    for i in 0..t {
        mat[i][i].add_assign(&diag[i]);
    }
    mat
}

lazy_static! {
    pub static ref POSEIDON2_FELT252_2_PARAMS: Arc<Poseidon2Params<Scalar>> = {
        let diag = internal_diag_2();
        let mat_internal = build_internal_matrix(&diag);
        Arc::new(Poseidon2Params::from_grain(
            2,
            3,
            8,
            56,
            &diag,
            &mat_internal,
        ))
    };
    pub static ref POSEIDON2_FELT252_3_PARAMS: Arc<Poseidon2Params<Scalar>> = {
        let diag = internal_diag_3();
        let mat_internal = build_internal_matrix(&diag);
        Arc::new(Poseidon2Params::from_grain(
            3,
            3,
            8,
            56,
            &diag,
            &mat_internal,
        ))
    };
}
