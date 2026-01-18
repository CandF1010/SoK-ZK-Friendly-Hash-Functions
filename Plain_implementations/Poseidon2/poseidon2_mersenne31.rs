use super::poseidon2_params::Poseidon2Params;
use crate::fields::mersenne31::Mersenne31;
use crate::fields::FieldElement;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Mersenne31;

const D: u64 = 5;
const ROUNDS_F: usize = 8;
const ROUNDS_P_16: usize = 14;
const ROUNDS_P_24: usize = 22;

fn internal_diag_16() -> Vec<Scalar> {
    vec![
        Scalar::from_u64(2).negate(),
        Scalar::from_u64(1),
        Scalar::from_u64(2),
        Scalar::from_u64(4),
        Scalar::from_u64(8),
        Scalar::from_u64(16),
        Scalar::from_u64(32),
        Scalar::from_u64(64),
        Scalar::from_u64(1u64 << 7),
        Scalar::from_u64(1u64 << 8),
        Scalar::from_u64(1u64 << 10),
        Scalar::from_u64(1u64 << 12),
        Scalar::from_u64(1u64 << 13),
        Scalar::from_u64(1u64 << 14),
        Scalar::from_u64(1u64 << 15),
        Scalar::from_u64(1u64 << 16),
    ]
}

fn internal_diag_24() -> Vec<Scalar> {
    vec![
        Scalar::from_u64(2).negate(),
        Scalar::from_u64(1),
        Scalar::from_u64(2),
        Scalar::from_u64(4),
        Scalar::from_u64(8),
        Scalar::from_u64(16),
        Scalar::from_u64(32),
        Scalar::from_u64(64),
        Scalar::from_u64(1u64 << 7),
        Scalar::from_u64(1u64 << 8),
        Scalar::from_u64(1u64 << 9),
        Scalar::from_u64(1u64 << 10),
        Scalar::from_u64(1u64 << 11),
        Scalar::from_u64(1u64 << 12),
        Scalar::from_u64(1u64 << 13),
        Scalar::from_u64(1u64 << 14),
        Scalar::from_u64(1u64 << 15),
        Scalar::from_u64(1u64 << 16),
        Scalar::from_u64(1u64 << 17),
        Scalar::from_u64(1u64 << 18),
        Scalar::from_u64(1u64 << 19),
        Scalar::from_u64(1u64 << 20),
        Scalar::from_u64(1u64 << 21),
        Scalar::from_u64(1u64 << 22),
    ]
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
    pub static ref POSEIDON2_MERSENNE31_16_PARAMS: Arc<Poseidon2Params<Scalar>> = {
        let diag = internal_diag_16();
        let mat_internal = build_internal_matrix(&diag);
        Arc::new(Poseidon2Params::from_grain(
            16,
            D,
            ROUNDS_F,
            ROUNDS_P_16,
            &diag,
            &mat_internal,
        ))
    };

    pub static ref POSEIDON2_MERSENNE31_24_PARAMS: Arc<Poseidon2Params<Scalar>> = {
        let diag = internal_diag_24();
        let mat_internal = build_internal_matrix(&diag);
        Arc::new(Poseidon2Params::from_grain(
            24,
            D,
            ROUNDS_F,
            ROUNDS_P_24,
            &diag,
            &mat_internal,
        ))
    };
}
