use dusk_curves::bls12_381::BlsScalar;

use crate::arion::WIDTH;

pub const MATRIX: [[BlsScalar; WIDTH]; WIDTH] = {

    let mat_raw = [
        [1, 2, 3, 4],
        [4, 1, 2, 3],
        [3, 4, 1, 2],
        [2, 3, 4, 1],
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
