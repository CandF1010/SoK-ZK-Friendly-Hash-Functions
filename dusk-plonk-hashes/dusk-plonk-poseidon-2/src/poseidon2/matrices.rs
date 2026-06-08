// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use dusk_curves::bls12_381::BlsScalar;

use crate::poseidon2::WIDTH;



/// Represents a `static reference` to the
/// `Maximum Distance Separable Matrix -> MDS_MATRIX`
/// of `(WIDTH x WIDTH)`.
///
/// Matrix is taken from https://github.com/HorizenLabs/poseidon2.git
pub const MATRIX_EXTERNAL: [[BlsScalar; WIDTH]; WIDTH] = {

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

/// Diagonal entries are taken from https://github.com/HorizenLabs/poseidon2.git
pub const MATRIX_INTERNAL: [[BlsScalar; WIDTH]; WIDTH] = {

    let diag_raw = [
            [9504325156600634173, 5692817194839602091, 6925806655738617122, 528692291691282888],
            [3790653436390206116, 14699154428968799256, 599204178697582302, 7026991724733177724],
            [11040537809306477778, 7297697885296586774, 3163487935215441259, 4724989628689984884],
            [904020457471457159, 12398633180389008006, 7322330905204831789, 3622246414198469028],
        ];

    let mut mat = [[BlsScalar::zero(); WIDTH]; WIDTH];
    let mut i = 0;
    while i < WIDTH  {
        let mut j = 0;
        while j < WIDTH {
            if i == j {
                mat[i][j] = BlsScalar::from_raw([diag_raw[i][j], 0, 0, 0]);
            }
            else {
                mat[i][j] = BlsScalar::from_raw([1, 0, 0, 0]);
            }
            
            j += 1;
        }
        i += 1;
    }
    mat
};
