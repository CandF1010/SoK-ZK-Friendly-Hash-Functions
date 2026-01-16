use super::poseidon2_params::Poseidon2Params;
use crate::fields::goldilocks::Goldilocks;
use crate::fields::FieldElement;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = Goldilocks;

const D: u64 = 7;
const ROUNDS_F: usize = 8;
const ROUNDS_P: usize = 22;

fn from_hex(s: &str) -> Option<Scalar> {
    Goldilocks::from_hex(s)
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
    pub static ref MAT_DIAG8_M_1: Vec<Scalar> = vec![
        from_hex("0xd57b33d215cc4805").unwrap(),
        from_hex("0xaa2238eb3ac17b62").unwrap(),
        from_hex("0x28925fe2f3895c0d").unwrap(),
        from_hex("0x3dab9370a67db22e").unwrap(),
        from_hex("0xe5cafe41ef4eac62").unwrap(),
        from_hex("0x4c633d43f2260c06").unwrap(),
        from_hex("0x1fa5fb8a31d6369d").unwrap(),
        from_hex("0x999a460e4a706453").unwrap(),
    ];
    pub static ref MAT_DIAG12_M_1: Vec<Scalar> = vec![
        from_hex("0xcf6f77ac16722af9").unwrap(),
        from_hex("0x3fd4c0d74672aebc").unwrap(),
        from_hex("0x9b72bf1c1c3d08a8").unwrap(),
        from_hex("0xe4940f84b71e4ac2").unwrap(),
        from_hex("0x61b27b077118bc72").unwrap(),
        from_hex("0x2efd8379b8e661e2").unwrap(),
        from_hex("0x858edcf353df0341").unwrap(),
        from_hex("0x2d9c20affb5c4516").unwrap(),
        from_hex("0x5120143f0695defb").unwrap(),
        from_hex("0x62fc898ae34a5c5b").unwrap(),
        from_hex("0xa3d9560c99123ed2").unwrap(),
        from_hex("0x98fd739d8e7fc933").unwrap(),
    ];
    pub static ref MAT_DIAG16_M_1: Vec<Scalar> = vec![
        from_hex("0x6467e51bf3d4a158").unwrap(),
        from_hex("0xbcd86f8ed95c8028").unwrap(),
        from_hex("0x865395802d29826f").unwrap(),
        from_hex("0xe4b1763459e41e21").unwrap(),
        from_hex("0x2da4f5a76bbef23a").unwrap(),
        from_hex("0x42702ba7e249fe75").unwrap(),
        from_hex("0x7b6353f0c4d29928").unwrap(),
        from_hex("0xd7f5676c38479db7").unwrap(),
        from_hex("0x4dc314866efb67cd").unwrap(),
        from_hex("0xe85869cfa847af8e").unwrap(),
        from_hex("0xef9f7c17e4a27a78").unwrap(),
        from_hex("0xabaf0627a707a822").unwrap(),
        from_hex("0x09455b5c8396308e").unwrap(),
        from_hex("0x428ba8cbc6f9cf05").unwrap(),
        from_hex("0xbe2c1090485097c9").unwrap(),
        from_hex("0x480a0d60cbaf82e9").unwrap(),
    ];
    pub static ref MAT_DIAG20_M_1: Vec<Scalar> = vec![
        from_hex("0xfb20ea446e196a5c").unwrap(),
        from_hex("0x28359de3f0a75592").unwrap(),
        from_hex("0x361c96febdbb2992").unwrap(),
        from_hex("0xde088f5cc8b99f3a").unwrap(),
        from_hex("0x2f58c248a386e52e").unwrap(),
        from_hex("0x271df056207e4b44").unwrap(),
        from_hex("0xf432599ba2016942").unwrap(),
        from_hex("0xb277767a1dbcd62e").unwrap(),
        from_hex("0x5d2ad97cae255d59").unwrap(),
        from_hex("0x52f108a3496726db").unwrap(),
        from_hex("0x56ea6dac1101bdaa").unwrap(),
        from_hex("0x03c7072cff7416a8").unwrap(),
        from_hex("0x2690ea28bcfe170d").unwrap(),
        from_hex("0xf1e4f90b056065fe").unwrap(),
        from_hex("0xb8c4343114021ebe").unwrap(),
        from_hex("0x53cd768d0a2469c8").unwrap(),
        from_hex("0xea2a4cfeebcfcf28").unwrap(),
        from_hex("0x85092b96df737b7b").unwrap(),
        from_hex("0x5706a9b2f87b8e80").unwrap(),
        from_hex("0xbc29720c8ad2e4fd").unwrap(),
    ];

    pub static ref POSEIDON2_GOLDILOCKS_8_PARAMS: Arc<Poseidon2Params<Scalar>> = {
        let mat_internal = build_internal_matrix(&MAT_DIAG8_M_1);
        Arc::new(Poseidon2Params::from_grain(
            8,
            D,
            ROUNDS_F,
            ROUNDS_P,
            &MAT_DIAG8_M_1,
            &mat_internal,
        ))
    };
    pub static ref POSEIDON2_GOLDILOCKS_12_PARAMS: Arc<Poseidon2Params<Scalar>> = {
        let mat_internal = build_internal_matrix(&MAT_DIAG12_M_1);
        Arc::new(Poseidon2Params::from_grain(
            12,
            D,
            ROUNDS_F,
            ROUNDS_P,
            &MAT_DIAG12_M_1,
            &mat_internal,
        ))
    };
    pub static ref POSEIDON2_GOLDILOCKS_16_PARAMS: Arc<Poseidon2Params<Scalar>> = {
        let mat_internal = build_internal_matrix(&MAT_DIAG16_M_1);
        Arc::new(Poseidon2Params::from_grain(
            16,
            D,
            ROUNDS_F,
            ROUNDS_P,
            &MAT_DIAG16_M_1,
            &mat_internal,
        ))
    };
    pub static ref POSEIDON2_GOLDILOCKS_20_PARAMS: Arc<Poseidon2Params<Scalar>> = {
        let mat_internal = build_internal_matrix(&MAT_DIAG20_M_1);
        Arc::new(Poseidon2Params::from_grain(
            20,
            D,
            ROUNDS_F,
            ROUNDS_P,
            &MAT_DIAG20_M_1,
            &mat_internal,
        ))
    };
}
