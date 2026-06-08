use dusk_curves::bls12_381::BlsScalar;

use crate::griffin::WIDTH;

pub const MATRIX: [[BlsScalar; WIDTH]; WIDTH] = {

    let mat_raw = [
        [5, 7, 1, 3],
        [4, 6, 1, 1],
        [1, 3, 5, 7],
        [1, 1, 4, 6],
    ];  

    let mut mat = [[BlsScalar::zero(); WIDTH]; WIDTH];
    let mut i = 0;
    while i < WIDTH  {
        let mut j = 0;
        while j < WIDTH {
            mat[i][j] = BlsScalar::from_raw([mat_raw[i][j], 0, 0, 0]);
            j += 1;
        }
        i += 1;
    }
    mat
};