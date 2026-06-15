use p3_field::{Algebra, Field, PrimeCharacteristicRing, PrimeField};

#[inline(always)]
pub fn add_tweak<F, A, const WIDTH: usize, const WIDTH_TWEAK: usize>(state: &mut [A; WIDTH], tweak: &[F; WIDTH_TWEAK])
where
    F: Field,
    A: Algebra<F>,
{
    for i in (0..WIDTH_TWEAK).into_iter() {
        state[i] += tweak[i].clone();
    }
}

#[inline(always)]
pub fn matrix<F, A, const WIDTH: usize>(state: &[A; WIDTH]) -> [A; WIDTH]
where
    F: Field,
    A: Algebra<F>,
{
    let mut result = [A::ZERO; WIDTH];
    (0..(WIDTH / 2)).into_iter().for_each(|i| result[i] += state[i].clone() + state[WIDTH / 2 + i].clone());
    ((WIDTH / 2)..WIDTH).into_iter().for_each(|i| result[i] += state[i - WIDTH / 2].clone().double() + state[i].clone());
    result
}

#[inline(always)]
pub fn non_linear_layer<A, F>(state: &[A], feistel_constants: &[F]) -> [A; 2]
where
    F: PrimeCharacteristicRing,
    A: Algebra<F>,
{
    let mut y_1 = state[1].clone() + feistel_constants[0].clone();
    let mut y_3 = y_1.clone() + feistel_constants[1].clone();

    let mut y_2 = state[0].clone();
    y_1 = y_1.square();
    y_2 += y_1.clone();

    y_3 += y_2.clone();

    let mut y_5 = y_3.clone();

    let mut y_4 = y_2.clone();
    y_3 = y_3.square();
    y_4 += y_3.clone();

    y_5 += y_4.clone();

    [y_4.clone(), y_5.clone()]
}

#[inline(always)]
pub fn linear_combination<A, F>(state: &[A]) -> [A; 2]
where
    F: PrimeCharacteristicRing,
    A: Algebra<F>,
{
    let z_1 = state[0].clone() + state[1].clone();
    let z_0 = z_1.clone() + state[0].clone();

    [z_0.clone(), z_1.clone()]
}


#[inline(always)]
pub fn double_feistel_16<F, A, const WIDTH: usize>(
    state: &mut [A; WIDTH],
    feistel_constants: &[F],
)
where
    F: PrimeCharacteristicRing,
    A: Algebra<F>,
{
    //
    let z1 = linear_combination(&state[6..8]);
    // Filling elements in result in order is more efficient than out of order.
    let y = non_linear_layer(&state[6..8], &feistel_constants[0..2]);
    state[8] += y[0].clone();
    state[9] += y[1].clone();
    //
    let y = non_linear_layer(&state[4..6], &feistel_constants[2..4]);
    state[10] += z1[0].clone() + y[0].clone();
    state[11] += z1[1].clone() + y[1].clone();
    //
    let y = non_linear_layer(&state[2..4], &feistel_constants[4..6]);
    state[12] += z1[0].clone() + y[0].clone();
    state[13] += z1[1].clone() + y[1].clone();
    //
    let y = non_linear_layer(&state[0..2], &feistel_constants[6..8]);
    state[14] += state[2].clone() + state[4].clone() + z1[0].clone() + y[0].clone();
    state[15] += state[3].clone() + state[5].clone() + z1[1].clone() + y[1].clone();
    // println!("result after first round: {:?}", result);
    //
    let z2 = linear_combination(&state[14..16]);
    //
    let y = non_linear_layer(&state[14..16], &feistel_constants[8..10]);
    state[0] += y[0].clone();
    state[1] += y[1].clone();
    //
    let y = non_linear_layer(&state[12..14], &feistel_constants[10..12]);
    state[2] += z2[0].clone() + y[0].clone();
    state[3] += z2[1].clone() + y[1].clone();
    //
    let y = non_linear_layer(&state[10..12], &feistel_constants[12..14]);
    state[4] += z2[0].clone() + y[0].clone();
    state[5] += z2[1].clone() + y[1].clone();
    //
    let y = non_linear_layer(&state[8..10], &feistel_constants[14..16]);
    state[6] += state[10].clone() + state[12].clone() + z2[0].clone() + y[0].clone();
    state[7] += state[11].clone() + state[13].clone() + z2[1].clone() + y[1].clone();
}

#[inline(always)]
pub fn double_feistel_24<F, A, const WIDTH: usize>(
    state: &mut [A; WIDTH],
    feistel_constants: &[F],
)
where
    F: PrimeField,
    A: Algebra<F>,
{
    //
    let z = linear_combination(&state[10..12]);
    // Filling elements in result in order is more efficient than out of order.
    let y = non_linear_layer(&state[10..12], &feistel_constants[0..2]);
    state[12] += y[0].clone();
    state[13] += y[1].clone();
    //
    let y = non_linear_layer(&state[8..10], &feistel_constants[2..4]);
    state[14] += z[0].clone() + y[0].clone();
    state[15] += z[1].clone() + y[1].clone();
    //
    let y = non_linear_layer(&state[6..8], &feistel_constants[4..6]);
    state[16] += z[0].clone() + y[0].clone();
    state[17] += z[1].clone() + y[1].clone();
    //
    let y = non_linear_layer(&state[4..6], &feistel_constants[6..8]);
    state[18] += z[0].clone() + y[0].clone();
    state[19] += z[1].clone() + y[1].clone();
    //
    let y = non_linear_layer(&state[2..4], &feistel_constants[8..10]);
    state[20] += z[0].clone() + y[0].clone();
    state[21] += z[1].clone() + y[1].clone();
    //
    let y = non_linear_layer(&state[0..2], &feistel_constants[10..12]);
    state[22] += state[2].clone()
        + state[4].clone()
        + state[6].clone()
        + state[8].clone()
        + z[0].clone()
        + y[0].clone();
    state[23] += state[3].clone()
        + state[5].clone()
        + state[7].clone()
        + state[9].clone()
        + z[1].clone()
        + y[1].clone();

    // Second half-round linear comb:
    let z = linear_combination(&state[22..24]);
    //r' 0,1
    let y =  non_linear_layer(&state[22..24], &feistel_constants[12..14]);
    state[0] += y[0].clone();
    state[1] += y[1].clone();

    //r' 2,3
    let y =  non_linear_layer(&state[20..22], &feistel_constants[14..16]);
    state[2] += z[0].clone() + y[0].clone();
    state[3] += z[1].clone() + y[1].clone();

    //r' 4,5
    let y =  non_linear_layer(&state[18..20], &feistel_constants[16..18]);
    state[4] += z[0].clone() + y[0].clone();
    state[5] += z[1].clone() + y[1].clone();

    //r' 6,7
    let y =  non_linear_layer(&state[16..18], &feistel_constants[18..20]);
    state[6] += z[0].clone() + y[0].clone();
    state[7] += z[1].clone() + y[1].clone();

    //r' 8,9
    let y = non_linear_layer(&state[14..16], &feistel_constants[20..22]);
    state[8] += z[0].clone() + y[0].clone();
    state[9] += z[1].clone() + y[1].clone();

    //r' 10
    let y = non_linear_layer(&state[12..14], &feistel_constants[22..24]);
    state[10] += state[14].clone()
        + state[16].clone()
        + state[18].clone()
        + state[20].clone()
        + z[0].clone()
        + y[0].clone();
    //r' 11
    state[11] += state[15].clone()
        + state[17].clone()
        + state[19].clone()
        + state[21].clone()
        + z[1].clone()
        + y[1].clone();
}

#[inline(always)]
pub fn feistel_16<F, A, const WIDTH: usize>(
    state: &[A; WIDTH],
    feistel_constants: &[F],
) -> [A; WIDTH]
where
    F: PrimeField,
    A: Algebra<F>,
{
    let mut result = [A::ZERO; WIDTH];
    //
    let z = linear_combination(&state[6..8]);
    // Filling elements in result in order is more efficient than out of order.
    let y = non_linear_layer(&state[6..8], &feistel_constants[0..2]);
    result[0] = state[8].clone() + y[0].clone();
    result[1] = state[9].clone() + y[1].clone();
    //
    let y = non_linear_layer(&state[4..6], &feistel_constants[2..4]);
    result[2] = state[10].clone() + z[0].clone() + y[0].clone();
    result[3] = state[11].clone() + z[1].clone() + y[1].clone();
    //
    let y = non_linear_layer(&state[2..4], &feistel_constants[4..6]);
    result[4] = state[12].clone() + z[0].clone() + y[0].clone();
    result[5] = state[13].clone() + z[1].clone() + y[1].clone();
    //
    let y = non_linear_layer(&state[0..2], &feistel_constants[6..8]);
    result[6] = state[14].clone() + state[2].clone() + state[4].clone() + z[0].clone() + y[0].clone();
    result[7] = state[15].clone() + state[3].clone() + state[5].clone() + z[1].clone() + y[1].clone();
    //
    result[8] = state[0].clone();
    result[9] = state[1].clone();
    result[10] = state[2].clone();
    result[11] = state[3].clone();
    result[12] = state[4].clone();
    result[13] = state[5].clone();
    result[14] = state[6].clone();
    result[15] = state[7].clone();
    result
}

#[inline(always)]
pub fn feistel_24<F, A, const WIDTH: usize>(
    state: &[A; WIDTH],
    feistel_constants: &[F],
) -> [A; WIDTH]
where
    F: PrimeField,
    A: Algebra<F>,
{
    let mut result = [A::ZERO; WIDTH];
    //
    let z = linear_combination(&state[10..12]);
    // Filling elements in result in order is more efficient than out of order.
    let y = non_linear_layer(&state[10..12], &feistel_constants[0..2]);
    result[0] = state[12].clone() + y[0].clone();
    result[1] = state[13].clone() + y[1].clone();
    //
    let y = non_linear_layer(&state[8..10], &feistel_constants[2..4]);
    result[2] = state[14].clone() + z[0].clone() + y[0].clone();
    result[3] = state[15].clone() + z[1].clone() + y[1].clone();
    //
    let y = non_linear_layer(&state[6..8], &feistel_constants[4..6]);
    result[4] = state[16].clone() + z[0].clone() + y[0].clone();
    result[5] = state[17].clone() + z[1].clone() + y[1].clone();
    //
    let y = non_linear_layer(&state[4..6], &feistel_constants[6..8]);
    result[6] = state[18].clone() + z[0].clone() + y[0].clone();
    result[7] = state[19].clone() + z[1].clone() + y[1].clone();
    //
    let y = non_linear_layer(&state[2..4], &feistel_constants[8..10]);
    result[8] = state[20].clone() + z[0].clone() + y[0].clone();
    result[9] = state[21].clone() + z[1].clone() + y[1].clone();
    //
    let y = non_linear_layer(&state[0..2], &feistel_constants[10..12]);
    result[10] = state[22].clone()
        + state[2].clone()
        + state[4].clone()
        + state[6].clone()
        + state[8].clone()
        + z[0].clone()
        + y[0].clone();
    result[11] = state[23].clone()
        + state[3].clone()
        + state[5].clone()
        + state[7].clone()
        + state[9].clone()
        + z[1].clone()
        + y[1].clone();
    //
    result[12] = state[0].clone();
    result[13] = state[1].clone();
    result[14] = state[2].clone();
    result[15] = state[3].clone();
    result[16] = state[4].clone();
    result[17] = state[5].clone();
    result[18] = state[6].clone();
    result[19] = state[7].clone();
    result[20] = state[8].clone();
    result[21] = state[9].clone();
    result[22] = state[10].clone();
    result[23] = state[11].clone();
    result
}
