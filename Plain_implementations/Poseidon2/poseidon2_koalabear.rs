use super::poseidon2_params::Poseidon2Params;
use crate::fields::koalabear::KoalaBear;
use crate::fields::FieldElement;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = KoalaBear;

const D: u64 = 3;
const ROUNDS_F: usize = 8;
const ROUNDS_P_16: usize = 20;
const ROUNDS_P_24: usize = 23;

fn inv_pow2(exp: u64) -> Scalar {
    let modulus = KoalaBear::MODULUS as u64;
    Scalar::from_u64(2).pow_u64(modulus - 1 - exp)
}

fn internal_diag_16() -> Vec<Scalar> {
    let inv2 = inv_pow2(1);
    let inv2_8 = inv_pow2(3);
    let inv2_16 = inv_pow2(4);
    let inv2_8pow = inv_pow2(8);
    let inv2_24 = inv_pow2(24);

    vec![
        Scalar::from_u64(2).negate(),
        Scalar::from_u64(1),
        Scalar::from_u64(2),
        inv2.clone(),
        Scalar::from_u64(3),
        Scalar::from_u64(4),
        inv2.negate(),
        Scalar::from_u64(3).negate(),
        Scalar::from_u64(4).negate(),
        inv2_8pow.clone(),
        inv2_8.clone(),
        inv2_24.clone(),
        inv2_8pow.negate(),
        inv2_8.negate(),
        inv2_16.negate(),
        inv2_24.negate(),
    ]
}

fn internal_diag_24() -> Vec<Scalar> {
    let inv2 = inv_pow2(1);
    let inv2_4 = inv_pow2(2);
    let inv2_8 = inv_pow2(3);
    let inv2_16 = inv_pow2(4);
    let inv2_32 = inv_pow2(5);
    let inv2_64 = inv_pow2(6);
    let inv2_7 = inv_pow2(7);
    let inv2_9 = inv_pow2(9);
    let inv2_8pow = inv_pow2(8);
    let inv2_24 = inv_pow2(24);

    vec![
        Scalar::from_u64(2).negate(),
        Scalar::from_u64(1),
        Scalar::from_u64(2),
        inv2.clone(),
        Scalar::from_u64(3),
        Scalar::from_u64(4),
        inv2.negate(),
        Scalar::from_u64(3).negate(),
        Scalar::from_u64(4).negate(),
        inv2_8pow.clone(),
        inv2_4.clone(),
        inv2_8.clone(),
        inv2_16.clone(),
        inv2_32.clone(),
        inv2_64.clone(),
        inv2_24.clone(),
        inv2_8pow.negate(),
        inv2_8.negate(),
        inv2_16.negate(),
        inv2_32.negate(),
        inv2_64.negate(),
        inv2_7.negate(),
        inv2_9.negate(),
        inv2_24.negate(),
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
    pub static ref POSEIDON2_KOALABEAR_16_PARAMS: Arc<Poseidon2Params<Scalar>> = {
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

    pub static ref POSEIDON2_KOALABEAR_24_PARAMS: Arc<Poseidon2Params<Scalar>> = {
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
