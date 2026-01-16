use sok_zk_friendly_hash_functions::anemoi::anemoi::Anemoi;
use sok_zk_friendly_hash_functions::anemoi::anemoi_babybear::{
    ANEMOI_BABYBEAR_2_PARAMS, ANEMOI_BABYBEAR_4_PARAMS,
};
use sok_zk_friendly_hash_functions::anemoi::anemoi_bls12_381::{
    ANEMOI_BLS12_381_2_PARAMS, ANEMOI_BLS12_381_4_PARAMS,
};
use sok_zk_friendly_hash_functions::anemoi::anemoi_bn254::{
    ANEMOI_BN254_2_PARAMS, ANEMOI_BN254_4_PARAMS,
};
use sok_zk_friendly_hash_functions::anemoi::anemoi_goldilocks::{
    ANEMOI_GOLDILOCKS_2_PARAMS, ANEMOI_GOLDILOCKS_4_PARAMS,
};
use sok_zk_friendly_hash_functions::anemoi::anemoi_koalabear::{
    ANEMOI_KOALABEAR_2_PARAMS, ANEMOI_KOALABEAR_4_PARAMS,
};
use sok_zk_friendly_hash_functions::fields::FieldElement;
use sok_zk_friendly_hash_functions::gmimc_erf::gmimc_erf::GmimcErf;
use sok_zk_friendly_hash_functions::gmimc_erf::gmimc_erf_babybear::{
    GMIMC_ERF_BABYBEAR_16_PARAMS, GMIMC_ERF_BABYBEAR_24_PARAMS,
};
use sok_zk_friendly_hash_functions::gmimc_erf::gmimc_erf_bls12_381::{
    GMIMC_ERF_BLS12_381_2_PARAMS, GMIMC_ERF_BLS12_381_3_PARAMS,
};
use sok_zk_friendly_hash_functions::gmimc_erf::gmimc_erf_bn254::{
    GMIMC_ERF_BN254_2_PARAMS, GMIMC_ERF_BN254_3_PARAMS,
};
use sok_zk_friendly_hash_functions::gmimc_erf::gmimc_erf_felt252::{
    GMIMC_ERF_FELT252_2_PARAMS, GMIMC_ERF_FELT252_3_PARAMS,
};
use sok_zk_friendly_hash_functions::gmimc_erf::gmimc_erf_goldilocks::{
    GMIMC_ERF_GOLDILOCKS_8_PARAMS, GMIMC_ERF_GOLDILOCKS_12_PARAMS,
};
use sok_zk_friendly_hash_functions::gmimc_erf::gmimc_erf_koalabear::{
    GMIMC_ERF_KOALABEAR_16_PARAMS, GMIMC_ERF_KOALABEAR_24_PARAMS,
};
use sok_zk_friendly_hash_functions::poseidon::poseidon::Poseidon;
use sok_zk_friendly_hash_functions::poseidon::poseidon_babybear::{
    POSEIDON_BABYBEAR_16_PARAMS, POSEIDON_BABYBEAR_24_PARAMS,
};
use sok_zk_friendly_hash_functions::poseidon::poseidon_bls12_381::{
    POSEIDON_BLS12_381_2_PARAMS, POSEIDON_BLS12_381_3_PARAMS,
};
use sok_zk_friendly_hash_functions::poseidon::poseidon_bn254::{
    POSEIDON_BN254_2_PARAMS, POSEIDON_BN254_3_PARAMS,
};
use sok_zk_friendly_hash_functions::poseidon::poseidon_felt252::{
    POSEIDON_FELT252_2_PARAMS, POSEIDON_FELT252_3_PARAMS,
};
use sok_zk_friendly_hash_functions::poseidon::poseidon_goldilocks::{
    POSEIDON_GOLDILOCKS_8_PARAMS, POSEIDON_GOLDILOCKS_12_PARAMS,
};
use sok_zk_friendly_hash_functions::poseidon::poseidon_koalabear::{
    POSEIDON_KOALABEAR_16_PARAMS, POSEIDON_KOALABEAR_24_PARAMS,
};
use sok_zk_friendly_hash_functions::poseidon2::poseidon2::Poseidon2;
use sok_zk_friendly_hash_functions::poseidon2::poseidon2_babybear::{
    POSEIDON2_BABYBEAR_16_PARAMS, POSEIDON2_BABYBEAR_24_PARAMS,
};
use sok_zk_friendly_hash_functions::poseidon2::poseidon2_bls12_381::{
    POSEIDON2_BLS12_381_2_PARAMS, POSEIDON2_BLS12_381_3_PARAMS,
};
use sok_zk_friendly_hash_functions::poseidon2::poseidon2_bn254::{
    POSEIDON2_BN254_2_PARAMS, POSEIDON2_BN254_3_PARAMS,
};
use sok_zk_friendly_hash_functions::poseidon2::poseidon2_felt252::{
    POSEIDON2_FELT252_2_PARAMS, POSEIDON2_FELT252_3_PARAMS,
};
use sok_zk_friendly_hash_functions::poseidon2::poseidon2_goldilocks::{
    POSEIDON2_GOLDILOCKS_8_PARAMS, POSEIDON2_GOLDILOCKS_12_PARAMS,
};
use sok_zk_friendly_hash_functions::poseidon2::poseidon2_koalabear::{
    POSEIDON2_KOALABEAR_16_PARAMS, POSEIDON2_KOALABEAR_24_PARAMS,
};
use sok_zk_friendly_hash_functions::rescue::rescue::Rescue;
use sok_zk_friendly_hash_functions::rescue::rescue_babybear::{
    RESCUE_BABYBEAR_16_PARAMS, RESCUE_BABYBEAR_24_PARAMS,
};
use sok_zk_friendly_hash_functions::rescue::rescue_bls12_381::{
    RESCUE_BLS12_381_2_PARAMS, RESCUE_BLS12_381_3_PARAMS,
};
use sok_zk_friendly_hash_functions::rescue::rescue_bn254::{
    RESCUE_BN254_2_PARAMS, RESCUE_BN254_3_PARAMS,
};
use sok_zk_friendly_hash_functions::rescue::rescue_felt252::{
    RESCUE_FELT252_2_PARAMS, RESCUE_FELT252_3_PARAMS,
};
use sok_zk_friendly_hash_functions::rescue::rescue_goldilocks::{
    RESCUE_GOLDILOCKS_8_PARAMS, RESCUE_GOLDILOCKS_12_PARAMS,
};
use sok_zk_friendly_hash_functions::rescue::rescue_koalabear::{
    RESCUE_KOALABEAR_16_PARAMS, RESCUE_KOALABEAR_24_PARAMS,
};
use std::hint::black_box;
use std::time::Instant;

const ITERS: usize = 1 << 14;
const ITERS_ANEMOI_W4: usize = ITERS * 3 / 4;

fn main() {
    println!("iters = {ITERS}, anemoi_w4_iters = {ITERS_ANEMOI_W4}");

    println!("\n== Poseidon (state ~512) ==");
    bench_poseidon("Poseidon BN254 t=2", &Poseidon::new(&POSEIDON_BN254_2_PARAMS), ITERS);
    bench_poseidon(
        "Poseidon BLS12-381 t=2",
        &Poseidon::new(&POSEIDON_BLS12_381_2_PARAMS),
        ITERS,
    );
    bench_poseidon(
        "Poseidon felt252 t=2",
        &Poseidon::new(&POSEIDON_FELT252_2_PARAMS),
        ITERS,
    );
    bench_poseidon(
        "Poseidon Goldilocks t=8",
        &Poseidon::new(&POSEIDON_GOLDILOCKS_8_PARAMS),
        ITERS,
    );
    bench_poseidon(
        "Poseidon BabyBear t=16",
        &Poseidon::new(&POSEIDON_BABYBEAR_16_PARAMS),
        ITERS,
    );
    bench_poseidon(
        "Poseidon KoalaBear t=16",
        &Poseidon::new(&POSEIDON_KOALABEAR_16_PARAMS),
        ITERS,
    );

    println!("\n== Poseidon (state ~768) ==");
    bench_poseidon("Poseidon BN254 t=3", &Poseidon::new(&POSEIDON_BN254_3_PARAMS), ITERS);
    bench_poseidon(
        "Poseidon BLS12-381 t=3",
        &Poseidon::new(&POSEIDON_BLS12_381_3_PARAMS),
        ITERS,
    );
    bench_poseidon(
        "Poseidon felt252 t=3",
        &Poseidon::new(&POSEIDON_FELT252_3_PARAMS),
        ITERS,
    );
    bench_poseidon(
        "Poseidon Goldilocks t=12",
        &Poseidon::new(&POSEIDON_GOLDILOCKS_12_PARAMS),
        ITERS,
    );
    bench_poseidon(
        "Poseidon BabyBear t=24",
        &Poseidon::new(&POSEIDON_BABYBEAR_24_PARAMS),
        ITERS,
    );
    bench_poseidon(
        "Poseidon KoalaBear t=24",
        &Poseidon::new(&POSEIDON_KOALABEAR_24_PARAMS),
        ITERS,
    );

    println!("\n== Poseidon2 (state ~512) ==");
    bench_poseidon2(
        "Poseidon2 BN254 t=2",
        &Poseidon2::new(&POSEIDON2_BN254_2_PARAMS),
        ITERS,
    );
    bench_poseidon2(
        "Poseidon2 BLS12-381 t=2",
        &Poseidon2::new(&POSEIDON2_BLS12_381_2_PARAMS),
        ITERS,
    );
    bench_poseidon2(
        "Poseidon2 felt252 t=2",
        &Poseidon2::new(&POSEIDON2_FELT252_2_PARAMS),
        ITERS,
    );
    bench_poseidon2(
        "Poseidon2 Goldilocks t=8",
        &Poseidon2::new(&POSEIDON2_GOLDILOCKS_8_PARAMS),
        ITERS,
    );
    bench_poseidon2(
        "Poseidon2 BabyBear t=16",
        &Poseidon2::new(&POSEIDON2_BABYBEAR_16_PARAMS),
        ITERS,
    );
    bench_poseidon2(
        "Poseidon2 KoalaBear t=16",
        &Poseidon2::new(&POSEIDON2_KOALABEAR_16_PARAMS),
        ITERS,
    );

    println!("\n== Poseidon2 (state ~768) ==");
    bench_poseidon2(
        "Poseidon2 BN254 t=3",
        &Poseidon2::new(&POSEIDON2_BN254_3_PARAMS),
        ITERS,
    );
    bench_poseidon2(
        "Poseidon2 BLS12-381 t=3",
        &Poseidon2::new(&POSEIDON2_BLS12_381_3_PARAMS),
        ITERS,
    );
    bench_poseidon2(
        "Poseidon2 felt252 t=3",
        &Poseidon2::new(&POSEIDON2_FELT252_3_PARAMS),
        ITERS,
    );
    bench_poseidon2(
        "Poseidon2 Goldilocks t=12",
        &Poseidon2::new(&POSEIDON2_GOLDILOCKS_12_PARAMS),
        ITERS,
    );
    bench_poseidon2(
        "Poseidon2 BabyBear t=24",
        &Poseidon2::new(&POSEIDON2_BABYBEAR_24_PARAMS),
        ITERS,
    );
    bench_poseidon2(
        "Poseidon2 KoalaBear t=24",
        &Poseidon2::new(&POSEIDON2_KOALABEAR_24_PARAMS),
        ITERS,
    );

    println!("\n== Rescue (state ~512) ==");
    bench_rescue("Rescue BN254 t=2", &Rescue::new(&RESCUE_BN254_2_PARAMS), ITERS);
    bench_rescue(
        "Rescue BLS12-381 t=2",
        &Rescue::new(&RESCUE_BLS12_381_2_PARAMS),
        ITERS,
    );
    bench_rescue(
        "Rescue felt252 t=2",
        &Rescue::new(&RESCUE_FELT252_2_PARAMS),
        ITERS,
    );
    bench_rescue(
        "Rescue Goldilocks t=8",
        &Rescue::new(&RESCUE_GOLDILOCKS_8_PARAMS),
        ITERS,
    );
    bench_rescue(
        "Rescue BabyBear t=16",
        &Rescue::new(&RESCUE_BABYBEAR_16_PARAMS),
        ITERS,
    );
    bench_rescue(
        "Rescue KoalaBear t=16",
        &Rescue::new(&RESCUE_KOALABEAR_16_PARAMS),
        ITERS,
    );

    println!("\n== Rescue (state ~768) ==");
    bench_rescue("Rescue BN254 t=3", &Rescue::new(&RESCUE_BN254_3_PARAMS), ITERS);
    bench_rescue(
        "Rescue BLS12-381 t=3",
        &Rescue::new(&RESCUE_BLS12_381_3_PARAMS),
        ITERS,
    );
    bench_rescue(
        "Rescue felt252 t=3",
        &Rescue::new(&RESCUE_FELT252_3_PARAMS),
        ITERS,
    );
    bench_rescue(
        "Rescue Goldilocks t=12",
        &Rescue::new(&RESCUE_GOLDILOCKS_12_PARAMS),
        ITERS,
    );
    bench_rescue(
        "Rescue BabyBear t=24",
        &Rescue::new(&RESCUE_BABYBEAR_24_PARAMS),
        ITERS,
    );
    bench_rescue(
        "Rescue KoalaBear t=24",
        &Rescue::new(&RESCUE_KOALABEAR_24_PARAMS),
        ITERS,
    );

    println!("\n== Anemoi (width 2) ==");
    bench_anemoi("Anemoi BN254 w=2", &Anemoi::new(&ANEMOI_BN254_2_PARAMS), ITERS);
    bench_anemoi(
        "Anemoi BLS12-381 w=2",
        &Anemoi::new(&ANEMOI_BLS12_381_2_PARAMS),
        ITERS,
    );
    bench_anemoi(
        "Anemoi Goldilocks w=2",
        &Anemoi::new(&ANEMOI_GOLDILOCKS_2_PARAMS),
        ITERS,
    );
    bench_anemoi(
        "Anemoi BabyBear w=2",
        &Anemoi::new(&ANEMOI_BABYBEAR_2_PARAMS),
        ITERS,
    );
    bench_anemoi(
        "Anemoi KoalaBear w=2",
        &Anemoi::new(&ANEMOI_KOALABEAR_2_PARAMS),
        ITERS,
    );

    println!("\n== Anemoi (width 4, 0.75x iters) ==");
    bench_anemoi(
        "Anemoi BN254 w=4",
        &Anemoi::new(&ANEMOI_BN254_4_PARAMS),
        ITERS_ANEMOI_W4,
    );
    bench_anemoi(
        "Anemoi BLS12-381 w=4",
        &Anemoi::new(&ANEMOI_BLS12_381_4_PARAMS),
        ITERS_ANEMOI_W4,
    );
    bench_anemoi(
        "Anemoi Goldilocks w=4",
        &Anemoi::new(&ANEMOI_GOLDILOCKS_4_PARAMS),
        ITERS_ANEMOI_W4,
    );
    bench_anemoi(
        "Anemoi BabyBear w=4",
        &Anemoi::new(&ANEMOI_BABYBEAR_4_PARAMS),
        ITERS_ANEMOI_W4,
    );
    bench_anemoi(
        "Anemoi KoalaBear w=4",
        &Anemoi::new(&ANEMOI_KOALABEAR_4_PARAMS),
        ITERS_ANEMOI_W4,
    );

    println!("\n== GMiMC-ERF (state ~512) ==");
    bench_gmimc(
        "GMiMC-ERF BN254 t=2",
        &GmimcErf::new(&GMIMC_ERF_BN254_2_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF BLS12-381 t=2",
        &GmimcErf::new(&GMIMC_ERF_BLS12_381_2_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF felt252 t=2",
        &GmimcErf::new(&GMIMC_ERF_FELT252_2_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF Goldilocks t=8",
        &GmimcErf::new(&GMIMC_ERF_GOLDILOCKS_8_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF BabyBear t=16",
        &GmimcErf::new(&GMIMC_ERF_BABYBEAR_16_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF KoalaBear t=16",
        &GmimcErf::new(&GMIMC_ERF_KOALABEAR_16_PARAMS),
        ITERS,
    );

    println!("\n== GMiMC-ERF (state ~768) ==");
    bench_gmimc(
        "GMiMC-ERF BN254 t=3",
        &GmimcErf::new(&GMIMC_ERF_BN254_3_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF BLS12-381 t=3",
        &GmimcErf::new(&GMIMC_ERF_BLS12_381_3_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF felt252 t=3",
        &GmimcErf::new(&GMIMC_ERF_FELT252_3_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF Goldilocks t=12",
        &GmimcErf::new(&GMIMC_ERF_GOLDILOCKS_12_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF BabyBear t=24",
        &GmimcErf::new(&GMIMC_ERF_BABYBEAR_24_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF KoalaBear t=24",
        &GmimcErf::new(&GMIMC_ERF_KOALABEAR_24_PARAMS),
        ITERS,
    );
}

fn bench_poseidon<F: FieldElement>(label: &str, perm: &Poseidon<F>, iters: usize) {
    let input = make_input::<F>(perm.get_t());
    bench_with_input(label, iters, &input, |inp| perm.permutation(inp));
}

fn bench_poseidon2<F: FieldElement>(label: &str, perm: &Poseidon2<F>, iters: usize) {
    let input = make_input::<F>(perm.get_t());
    bench_with_input(label, iters, &input, |inp| perm.permutation(inp));
}

fn bench_anemoi<F: FieldElement>(label: &str, perm: &Anemoi<F>, iters: usize) {
    let input = make_input::<F>(perm.get_t());
    bench_with_input(label, iters, &input, |inp| perm.permutation(inp));
}

fn bench_gmimc<F: FieldElement>(label: &str, perm: &GmimcErf<F>, iters: usize) {
    let input = make_input::<F>(perm.get_t());
    bench_with_input(label, iters, &input, |inp| perm.permutation(inp));
}

fn bench_rescue<F: FieldElement>(label: &str, perm: &Rescue<F>, iters: usize) {
    let input = make_input::<F>(perm.get_t());
    bench_with_input(label, iters, &input, |inp| perm.permutation(inp));
}

fn make_input<F: FieldElement>(t: usize) -> Vec<F> {
    (0..t).map(|i| F::from_u64((i + 1) as u64)).collect()
}

fn bench_with_input<F: FieldElement, R, FFn: FnMut(&[F]) -> R>(
    label: &str,
    iters: usize,
    input: &[F],
    mut f: FFn,
) {
    let start = Instant::now();
    let mut out = None;
    for _ in 0..iters {
        out = Some(f(input));
        black_box(&out);
    }
    let elapsed = start.elapsed();
    let per_ns = elapsed.as_nanos() / iters as u128;
    println!("{label}: {iters} iters in {elapsed:?} ({per_ns} ns/iter)");
    black_box(out);
}
