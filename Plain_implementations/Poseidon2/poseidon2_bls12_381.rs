use super::poseidon2_params::Poseidon2Params;
use crate::fields::bls12_381::Bls12_381;
use crate::fields::FieldElement;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Bls12_381;

const D: u64 = 5;
const ROUNDS_F: usize = 8;
const ROUNDS_P_2_3_4: usize = 56;
const ROUNDS_P_8: usize = 57;

fn from_hex(s: &str) -> Option<Scalar> {
    Bls12_381::from_hex(s)
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
    pub static ref MAT_DIAG2_M_1: Vec<Scalar> = vec![
        from_hex("0x0000000000000000000000000000000000000000000000000000000000000001").unwrap(),
        from_hex("0x0000000000000000000000000000000000000000000000000000000000000002").unwrap(),
    ];
    pub static ref MAT_DIAG3_M_1: Vec<Scalar> = vec![
        from_hex("0x0000000000000000000000000000000000000000000000000000000000000001").unwrap(),
        from_hex("0x0000000000000000000000000000000000000000000000000000000000000001").unwrap(),
        from_hex("0x0000000000000000000000000000000000000000000000000000000000000002").unwrap(),
    ];
    pub static ref MAT_DIAG4_M_1: Vec<Scalar> = vec![
        from_hex("0x03a8f448b6dfdbab048b5f193acd1d89b3c0d8777449a7c9e82836d71fce09ca").unwrap(),
        from_hex("0x6f82e6cb42fa93680f1058179be9ff2463a16bc2af5cfe85bba7b6f78b057b6e").unwrap(),
        from_hex("0x0528f173c24228cebfa1e5be28ddfe0b0d4dc579adc8b11868b741a8333a53f6").unwrap(),
        from_hex("0x6fdc6c0c05fba5fc05e81673a730002393e23f5d6ac931dcae63255550b941a1").unwrap(),
    ];
    pub static ref MAT_DIAG8_M_1: Vec<Scalar> = vec![
        from_hex("0x4289d156a8360e325b5b2aeb2889bf09627524e52168ccba9f6fb30cd3a631fc").unwrap(),
        from_hex("0x03b147b740e9e445f6e52855834847e7b99e6e5dd0b93962200d39294dcda046").unwrap(),
        from_hex("0x67231fe5a0ec8ed7e8a039c3a1602573b3e7280b9814c6be9ccb9bca3906c91b").unwrap(),
        from_hex("0x4e937e0df2287d9de5202a37039a232daa5a919c354329708f0c0a63ced25048").unwrap(),
        from_hex("0x0cab0fb17237f92f733d906104fe8996ac0f563bb31d50b1c57a087817ff05fa").unwrap(),
        from_hex("0x6f88c59917feab568ab485369e632d114dce43a7a7880ca4807a4a7f1ec14e4f").unwrap(),
        from_hex("0x260dfda28a3734c4d43b1707f481aa5a32329eceb23091628031cdc311a5dfd2").unwrap(),
        from_hex("0x4cbb316075e9c827d2688a8607a0efd237b899eda1f67739f9dac13e70061925").unwrap(),
    ];

    pub static ref POSEIDON2_BLS12_381_2_PARAMS: Arc<Poseidon2Params<Scalar>> = {
        let mat_internal = build_internal_matrix(&MAT_DIAG2_M_1);
        Arc::new(Poseidon2Params::from_grain(
            2,
            D,
            ROUNDS_F,
            ROUNDS_P_2_3_4,
            &MAT_DIAG2_M_1,
            &mat_internal,
        ))
    };
    pub static ref POSEIDON2_BLS12_381_3_PARAMS: Arc<Poseidon2Params<Scalar>> = {
        let mat_internal = build_internal_matrix(&MAT_DIAG3_M_1);
        Arc::new(Poseidon2Params::from_grain(
            3,
            D,
            ROUNDS_F,
            ROUNDS_P_2_3_4,
            &MAT_DIAG3_M_1,
            &mat_internal,
        ))
    };
    pub static ref POSEIDON2_BLS12_381_4_PARAMS: Arc<Poseidon2Params<Scalar>> = {
        let mat_internal = build_internal_matrix(&MAT_DIAG4_M_1);
        Arc::new(Poseidon2Params::from_grain(
            4,
            D,
            ROUNDS_F,
            ROUNDS_P_2_3_4,
            &MAT_DIAG4_M_1,
            &mat_internal,
        ))
    };
    pub static ref POSEIDON2_BLS12_381_8_PARAMS: Arc<Poseidon2Params<Scalar>> = {
        let mat_internal = build_internal_matrix(&MAT_DIAG8_M_1);
        Arc::new(Poseidon2Params::from_grain(
            8,
            D,
            ROUNDS_F,
            ROUNDS_P_8,
            &MAT_DIAG8_M_1,
            &mat_internal,
        ))
    };
}
