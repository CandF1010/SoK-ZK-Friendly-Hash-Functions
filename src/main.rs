// SoK paper benchmark: Type-1, Type-2, Type-3 hash permutations + baselines.
//
// Target hashes (from the March-4 meeting):
//   Type1: S-GMiMC / Poseidon2 / Neptune
//   Type2: Rescue  / Arion    / Anemoi / Griffin
//   Type3: Monolith / Polocolo / Tip5
//
// Baselines: SHA-256, Keccak-f[1600], Blake2b, Blake3.

use sok_zk_friendly_hash_functions::anemoi::anemoi::Anemoi;
use sok_zk_friendly_hash_functions::anemoi::instances::{
    ANEMOI_BABYBEAR_16_PARAMS, ANEMOI_BABYBEAR_24_PARAMS, ANEMOI_BLS12_381_2_PARAMS,
    ANEMOI_BN254_2_PARAMS, ANEMOI_GOLDILOCKS_12_PARAMS, ANEMOI_GOLDILOCKS_8_PARAMS,
    ANEMOI_KOALABEAR_16_PARAMS, ANEMOI_KOALABEAR_24_PARAMS, ANEMOI_MERSENNE31_16_PARAMS,
    ANEMOI_MERSENNE31_24_PARAMS,
};
use sok_zk_friendly_hash_functions::arion::arion::Arion;
use sok_zk_friendly_hash_functions::arion::instances::{
    ARION_BLS12_381_3_PARAMS, ARION_BN254_3_PARAMS,
};
use sok_zk_friendly_hash_functions::fields::{FieldElement, PrimeFieldWords};
use sok_zk_friendly_hash_functions::griffin::griffin::Griffin;
use sok_zk_friendly_hash_functions::griffin::instances::{
    GRIFFIN_BLS12_381_3_PARAMS, GRIFFIN_BN254_3_PARAMS, GRIFFIN_GOLDILOCKS_12_PARAMS,
    GRIFFIN_GOLDILOCKS_8_PARAMS,
};
use sok_zk_friendly_hash_functions::monolith::instances::{
    MONOLITH_BABYBEAR_16_PARAMS, MONOLITH_BABYBEAR_24_PARAMS, MONOLITH_GOLDILOCKS_12_PARAMS,
    MONOLITH_GOLDILOCKS_8_PARAMS, MONOLITH_KOALABEAR_16_PARAMS, MONOLITH_KOALABEAR_24_PARAMS,
    MONOLITH_MERSENNE31_16_PARAMS, MONOLITH_MERSENNE31_24_PARAMS,
};
use sok_zk_friendly_hash_functions::monolith::monolith::{Monolith31, Monolith64};
use sok_zk_friendly_hash_functions::monolith::monolith_params::{MonolithField32, MonolithField64};
use sok_zk_friendly_hash_functions::neptune::neptune::Neptune;
use sok_zk_friendly_hash_functions::neptune::instances::{
    NEPTUNE_BABYBEAR_16_PARAMS, NEPTUNE_BABYBEAR_24_PARAMS, NEPTUNE_BLS12_381_2_PARAMS,
    NEPTUNE_BN254_2_PARAMS, NEPTUNE_GOLDILOCKS_12_PARAMS, NEPTUNE_GOLDILOCKS_8_PARAMS,
    NEPTUNE_KOALABEAR_16_PARAMS, NEPTUNE_KOALABEAR_24_PARAMS, NEPTUNE_MERSENNE31_16_PARAMS,
    NEPTUNE_MERSENNE31_24_PARAMS,
};
use sok_zk_friendly_hash_functions::polocolo::instances::{
    POLOCOLO_BLS12_381_3_PARAMS, POLOCOLO_BN254_3_PARAMS,
};
use sok_zk_friendly_hash_functions::polocolo::polocolo::Polocolo;
use sok_zk_friendly_hash_functions::poseidon2::poseidon2::Poseidon2;
use sok_zk_friendly_hash_functions::poseidon2::instances::{
    POSEIDON2_BABYBEAR_16_PARAMS, POSEIDON2_BABYBEAR_24_PARAMS, POSEIDON2_BLS12_381_2_PARAMS,
    POSEIDON2_BLS12_381_3_PARAMS, POSEIDON2_BN254_2_PARAMS, POSEIDON2_BN254_3_PARAMS,
    POSEIDON2_GOLDILOCKS_12_PARAMS, POSEIDON2_GOLDILOCKS_8_PARAMS,
    POSEIDON2_KOALABEAR_16_PARAMS, POSEIDON2_KOALABEAR_24_PARAMS,
    POSEIDON2_MERSENNE31_16_PARAMS, POSEIDON2_MERSENNE31_24_PARAMS,
};
use sok_zk_friendly_hash_functions::rescueprime::instances::{
    RESCUE_PRIME_BLS12_381_2_PARAMS, RESCUE_PRIME_BLS12_381_3_PARAMS,
    RESCUE_PRIME_BN254_2_PARAMS, RESCUE_PRIME_BN254_3_PARAMS,
    RESCUE_PRIME_GOLDILOCKS_12_PARAMS, RESCUE_PRIME_GOLDILOCKS_8_PARAMS,
};
use sok_zk_friendly_hash_functions::rescueprime::rescue_prime::RescuePrime;
use sok_zk_friendly_hash_functions::sgmimc::instances::{
    SGMIMC_ALPHA8_BLS12_381_2_PARAMS, SGMIMC_ALPHA8_BLS12_381_3_PARAMS,
    SGMIMC_ALPHA8_BN254_2_PARAMS, SGMIMC_ALPHA8_BN254_3_PARAMS,
    SGMIMC_BABYBEAR_16_PARAMS, SGMIMC_BABYBEAR_24_PARAMS,
    SGMIMC_BLS12_381_2_PARAMS, SGMIMC_BLS12_381_3_PARAMS,
    SGMIMC_BN254_2_PARAMS, SGMIMC_BN254_3_PARAMS,
    SGMIMC_GOLDILOCKS_12_PARAMS, SGMIMC_GOLDILOCKS_8_PARAMS,
    SGMIMC_KOALABEAR_16_PARAMS, SGMIMC_KOALABEAR_24_PARAMS,
    SGMIMC_MERSENNE31_16_PARAMS, SGMIMC_MERSENNE31_24_PARAMS,
};
use sok_zk_friendly_hash_functions::sgmimc::sgmimc::SgmiMc;
use sok_zk_friendly_hash_functions::tip5::instances::TIP5_GOLDILOCKS_PARAMS;
use sok_zk_friendly_hash_functions::tip5::tip5::{Tip5, Tip5Field};
use sok_zk_friendly_hash_functions::plain_hashes;
use std::hint::black_box;
use std::time::Instant;

const ITERS: usize = 1 << 14;

fn main() {
    println!("iters = {ITERS}");

    // ============================================================
    // Type-1: S-GMiMC / Poseidon2 / Neptune
    // ============================================================

    // --- S-GMiMC (α=2, square S-box) ---

    println!("\n== S-GMiMC(alpha=2) state ~512 ==");
    bench_sgmimc("S-GMiMC(alpha=2) BN254 t=2", &SgmiMc::new(&SGMIMC_BN254_2_PARAMS), ITERS);
    bench_sgmimc("S-GMiMC(alpha=2) BLS12-381 t=2", &SgmiMc::new(&SGMIMC_BLS12_381_2_PARAMS), ITERS);
    bench_sgmimc("S-GMiMC(alpha=2) Goldilocks t=8", &SgmiMc::new(&SGMIMC_GOLDILOCKS_8_PARAMS), ITERS);
    bench_sgmimc("S-GMiMC(alpha=2) BabyBear t=16", &SgmiMc::new(&SGMIMC_BABYBEAR_16_PARAMS), ITERS);
    bench_sgmimc("S-GMiMC(alpha=2) KoalaBear t=16", &SgmiMc::new(&SGMIMC_KOALABEAR_16_PARAMS), ITERS);
    bench_sgmimc("S-GMiMC(alpha=2) Mersenne31 t=16", &SgmiMc::new(&SGMIMC_MERSENNE31_16_PARAMS), ITERS);

    println!("\n== S-GMiMC(alpha=2) state ~768 ==");
    bench_sgmimc("S-GMiMC(alpha=2) BN254 t=3", &SgmiMc::new(&SGMIMC_BN254_3_PARAMS), ITERS);
    bench_sgmimc("S-GMiMC(alpha=2) BLS12-381 t=3", &SgmiMc::new(&SGMIMC_BLS12_381_3_PARAMS), ITERS);
    bench_sgmimc("S-GMiMC(alpha=2) Goldilocks t=12", &SgmiMc::new(&SGMIMC_GOLDILOCKS_12_PARAMS), ITERS);
    bench_sgmimc("S-GMiMC(alpha=2) BabyBear t=24", &SgmiMc::new(&SGMIMC_BABYBEAR_24_PARAMS), ITERS);
    bench_sgmimc("S-GMiMC(alpha=2) KoalaBear t=24", &SgmiMc::new(&SGMIMC_KOALABEAR_24_PARAMS), ITERS);
    bench_sgmimc("S-GMiMC(alpha=2) Mersenne31 t=24", &SgmiMc::new(&SGMIMC_MERSENNE31_24_PARAMS), ITERS);

    // --- S-GMiMC (α=8, 256-bit only — optimal for large fields per Matthias) ---

    println!("\n== S-GMiMC(alpha=8) (~256-bit fields) ==");
    bench_sgmimc("S-GMiMC(alpha=8) BN254 t=2", &SgmiMc::new(&SGMIMC_ALPHA8_BN254_2_PARAMS), ITERS);
    bench_sgmimc("S-GMiMC(alpha=8) BN254 t=3", &SgmiMc::new(&SGMIMC_ALPHA8_BN254_3_PARAMS), ITERS);
    bench_sgmimc("S-GMiMC(alpha=8) BLS12-381 t=2", &SgmiMc::new(&SGMIMC_ALPHA8_BLS12_381_2_PARAMS), ITERS);
    bench_sgmimc("S-GMiMC(alpha=8) BLS12-381 t=3", &SgmiMc::new(&SGMIMC_ALPHA8_BLS12_381_3_PARAMS), ITERS);

    // --- Poseidon2 ---

    println!("\n== Poseidon2 (~256-bit fields) ==");
    bench_poseidon2("Poseidon2 BN254 t=2", &Poseidon2::new(&POSEIDON2_BN254_2_PARAMS), ITERS);
    bench_poseidon2("Poseidon2 BN254 t=3", &Poseidon2::new(&POSEIDON2_BN254_3_PARAMS), ITERS);
    bench_poseidon2("Poseidon2 BLS12-381 t=2", &Poseidon2::new(&POSEIDON2_BLS12_381_2_PARAMS), ITERS);
    bench_poseidon2("Poseidon2 BLS12-381 t=3", &Poseidon2::new(&POSEIDON2_BLS12_381_3_PARAMS), ITERS);

    println!("\n== Poseidon2 (~64-bit field) ==");
    bench_poseidon2("Poseidon2 Goldilocks t=8", &Poseidon2::new(&POSEIDON2_GOLDILOCKS_8_PARAMS), ITERS);
    bench_poseidon2("Poseidon2 Goldilocks t=12", &Poseidon2::new(&POSEIDON2_GOLDILOCKS_12_PARAMS), ITERS);

    println!("\n== Poseidon2 (~31-bit fields) ==");
    bench_poseidon2("Poseidon2 BabyBear t=16", &Poseidon2::new(&POSEIDON2_BABYBEAR_16_PARAMS), ITERS);
    bench_poseidon2("Poseidon2 BabyBear t=24", &Poseidon2::new(&POSEIDON2_BABYBEAR_24_PARAMS), ITERS);
    bench_poseidon2("Poseidon2 KoalaBear t=16", &Poseidon2::new(&POSEIDON2_KOALABEAR_16_PARAMS), ITERS);
    bench_poseidon2("Poseidon2 KoalaBear t=24", &Poseidon2::new(&POSEIDON2_KOALABEAR_24_PARAMS), ITERS);
    bench_poseidon2("Poseidon2 Mersenne31 t=16", &Poseidon2::new(&POSEIDON2_MERSENNE31_16_PARAMS), ITERS);
    bench_poseidon2("Poseidon2 Mersenne31 t=24", &Poseidon2::new(&POSEIDON2_MERSENNE31_24_PARAMS), ITERS);

    // --- Neptune ---

    println!("\n== Neptune (~256-bit fields) ==");
    bench_neptune("Neptune BN254 t=2", &Neptune::new(&NEPTUNE_BN254_2_PARAMS), ITERS);
    bench_neptune("Neptune BLS12-381 t=2", &Neptune::new(&NEPTUNE_BLS12_381_2_PARAMS), ITERS);

    println!("\n== Neptune (~64-bit field) ==");
    bench_neptune("Neptune Goldilocks t=8", &Neptune::new(&NEPTUNE_GOLDILOCKS_8_PARAMS), ITERS);
    bench_neptune("Neptune Goldilocks t=12", &Neptune::new(&NEPTUNE_GOLDILOCKS_12_PARAMS), ITERS);

    println!("\n== Neptune (~31-bit fields) ==");
    bench_neptune("Neptune BabyBear t=16", &Neptune::new(&NEPTUNE_BABYBEAR_16_PARAMS), ITERS);
    bench_neptune("Neptune KoalaBear t=16", &Neptune::new(&NEPTUNE_KOALABEAR_16_PARAMS), ITERS);
    bench_neptune("Neptune Mersenne31 t=16", &Neptune::new(&NEPTUNE_MERSENNE31_16_PARAMS), ITERS);
    bench_neptune("Neptune BabyBear t=24", &Neptune::new(&NEPTUNE_BABYBEAR_24_PARAMS), ITERS);
    bench_neptune("Neptune KoalaBear t=24", &Neptune::new(&NEPTUNE_KOALABEAR_24_PARAMS), ITERS);
    bench_neptune("Neptune Mersenne31 t=24", &Neptune::new(&NEPTUNE_MERSENNE31_24_PARAMS), ITERS);

    // ============================================================
    // Type-2: Rescue / Arion / Anemoi / Griffin
    // ============================================================

    println!("\n== RescuePrime (state ~512) ==");
    bench_rescue("Rescue BN254 t=2", &RescuePrime::new(&RESCUE_PRIME_BN254_2_PARAMS), ITERS);
    bench_rescue("Rescue BLS12-381 t=2", &RescuePrime::new(&RESCUE_PRIME_BLS12_381_2_PARAMS), ITERS);
    bench_rescue("Rescue Goldilocks t=8", &RescuePrime::new(&RESCUE_PRIME_GOLDILOCKS_8_PARAMS), ITERS);

    println!("\n== RescuePrime (state ~768) ==");
    bench_rescue("Rescue BN254 t=3", &RescuePrime::new(&RESCUE_PRIME_BN254_3_PARAMS), ITERS);
    bench_rescue("Rescue BLS12-381 t=3", &RescuePrime::new(&RESCUE_PRIME_BLS12_381_3_PARAMS), ITERS);
    bench_rescue("Rescue Goldilocks t=12", &RescuePrime::new(&RESCUE_PRIME_GOLDILOCKS_12_PARAMS), ITERS);

    println!("\n== Anemoi (~256-bit fields) ==");
    bench_anemoi("Anemoi BN254 t=2", &Anemoi::new(&ANEMOI_BN254_2_PARAMS), ITERS);
    bench_anemoi("Anemoi BLS12-381 t=2", &Anemoi::new(&ANEMOI_BLS12_381_2_PARAMS), ITERS);

    println!("\n== Anemoi (~64-bit field) ==");
    bench_anemoi("Anemoi Goldilocks t=8", &Anemoi::new(&ANEMOI_GOLDILOCKS_8_PARAMS), ITERS);
    bench_anemoi("Anemoi Goldilocks t=12", &Anemoi::new(&ANEMOI_GOLDILOCKS_12_PARAMS), ITERS);

    println!("\n== Anemoi (~31-bit fields) ==");
    bench_anemoi("Anemoi BabyBear t=16", &Anemoi::new(&ANEMOI_BABYBEAR_16_PARAMS), ITERS);
    bench_anemoi("Anemoi BabyBear t=24", &Anemoi::new(&ANEMOI_BABYBEAR_24_PARAMS), ITERS);
    bench_anemoi("Anemoi KoalaBear t=16", &Anemoi::new(&ANEMOI_KOALABEAR_16_PARAMS), ITERS);
    bench_anemoi("Anemoi KoalaBear t=24", &Anemoi::new(&ANEMOI_KOALABEAR_24_PARAMS), ITERS);
    bench_anemoi("Anemoi Mersenne31 t=16", &Anemoi::new(&ANEMOI_MERSENNE31_16_PARAMS), ITERS);
    bench_anemoi("Anemoi Mersenne31 t=24", &Anemoi::new(&ANEMOI_MERSENNE31_24_PARAMS), ITERS);

    println!("\n== Griffin (~256-bit fields) ==");
    bench_griffin("Griffin BN254 t=3", &Griffin::new(&GRIFFIN_BN254_3_PARAMS), ITERS);
    bench_griffin("Griffin BLS12-381 t=3", &Griffin::new(&GRIFFIN_BLS12_381_3_PARAMS), ITERS);

    println!("\n== Griffin (~64-bit field) ==");
    bench_griffin("Griffin Goldilocks t=8", &Griffin::new(&GRIFFIN_GOLDILOCKS_8_PARAMS), ITERS);
    bench_griffin("Griffin Goldilocks t=12", &Griffin::new(&GRIFFIN_GOLDILOCKS_12_PARAMS), ITERS);

    println!("\n== Arion (~256-bit fields) ==");
    bench_arion("Arion BN254 t=3", &Arion::new(&ARION_BN254_3_PARAMS), ITERS);
    bench_arion("Arion BLS12-381 t=3", &Arion::new(&ARION_BLS12_381_3_PARAMS), ITERS);

    // ============================================================
    // Type-3: Monolith / Polocolo / Tip5
    // ============================================================

    println!("\n== Monolith (state ~512) ==");
    bench_monolith64("Monolith Goldilocks t=8", &Monolith64::new(&MONOLITH_GOLDILOCKS_8_PARAMS), ITERS);
    bench_monolith31("Monolith Mersenne31 t=16", &Monolith31::new(&MONOLITH_MERSENNE31_16_PARAMS), ITERS);
    bench_monolith31("Monolith BabyBear t=16", &Monolith31::new(&MONOLITH_BABYBEAR_16_PARAMS), ITERS);
    bench_monolith31("Monolith KoalaBear t=16", &Monolith31::new(&MONOLITH_KOALABEAR_16_PARAMS), ITERS);

    println!("\n== Monolith (state ~768) ==");
    bench_monolith64("Monolith Goldilocks t=12", &Monolith64::new(&MONOLITH_GOLDILOCKS_12_PARAMS), ITERS);
    bench_monolith31("Monolith Mersenne31 t=24", &Monolith31::new(&MONOLITH_MERSENNE31_24_PARAMS), ITERS);
    bench_monolith31("Monolith BabyBear t=24", &Monolith31::new(&MONOLITH_BABYBEAR_24_PARAMS), ITERS);
    bench_monolith31("Monolith KoalaBear t=24", &Monolith31::new(&MONOLITH_KOALABEAR_24_PARAMS), ITERS);

    println!("\n== Polocolo (~256-bit fields) ==");
    bench_polocolo("Polocolo BN254 t=3", &Polocolo::new(&POLOCOLO_BN254_3_PARAMS), ITERS);
    bench_polocolo("Polocolo BLS12-381 t=3", &Polocolo::new(&POLOCOLO_BLS12_381_3_PARAMS), ITERS);

    println!("\n== Tip5 (Goldilocks) ==");
    bench_tip5("Tip5 Goldilocks", &Tip5::new(&TIP5_GOLDILOCKS_PARAMS), ITERS);

    // ============================================================
    // Baselines: SHA-256 / Keccak-f / Blake2b / Blake3
    // ============================================================

    println!("\n== SHA-256 / Keccak-f / Blake2b / Blake3 (permutations) ==");
    bench_sha256_perm("SHA-256 compress (1 block)", ITERS);
    bench_keccak_f1600("Keccak-f[1600] (24 rounds)", ITERS);
    bench_blake2b_perm("Blake2b compress (1 block)", ITERS);
    bench_blake3_perm("Blake3 compress (1 block)", ITERS);
}

// --- Benchmark helpers ---

fn bench_sgmimc<F: FieldElement>(label: &str, perm: &SgmiMc<F>, iters: usize) {
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

fn bench_griffin<F: FieldElement>(label: &str, perm: &Griffin<F>, iters: usize) {
    let input = make_input::<F>(perm.get_t());
    bench_with_input(label, iters, &input, |inp| perm.permutation(inp));
}

fn bench_neptune<F: FieldElement>(label: &str, perm: &Neptune<F>, iters: usize) {
    let input = make_input::<F>(perm.get_t());
    bench_with_input(label, iters, &input, |inp| perm.permutation(inp));
}

fn bench_polocolo<F: PrimeFieldWords>(label: &str, perm: &Polocolo<F>, iters: usize) {
    let input = make_input::<F>(perm.get_t());
    bench_with_input(label, iters, &input, |inp| perm.permutation(inp));
}

fn bench_monolith64<F: MonolithField64>(label: &str, perm: &Monolith64<F>, iters: usize) {
    let input = make_input::<F>(perm.get_t());
    bench_with_input(label, iters, &input, |inp| perm.permutation(inp));
}

fn bench_monolith31<F: MonolithField32>(label: &str, perm: &Monolith31<F>, iters: usize) {
    let input = make_input::<F>(perm.get_t());
    bench_with_input(label, iters, &input, |inp| perm.permutation(inp));
}

fn bench_tip5<F: Tip5Field>(label: &str, perm: &Tip5<F>, iters: usize) {
    let input = make_input::<F>(perm.get_t());
    bench_with_input(label, iters, &input, |inp| perm.permutation(inp));
}

fn bench_rescue<F: FieldElement>(label: &str, perm: &RescuePrime<F>, iters: usize) {
    let input = make_input::<F>(perm.get_t());
    bench_with_input(label, iters, &input, |inp| perm.permutation(inp));
}

fn bench_arion<F: FieldElement>(label: &str, perm: &Arion<F>, iters: usize) {
    let input = make_input::<F>(perm.get_t());
    bench_with_input(label, iters, &input, |inp| perm.permutation(inp));
}

fn bench_sha256_perm(label: &str, iters: usize) {
    let mut state: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    use sha2::digest::generic_array::GenericArray;
    let block_arr: [u8; 64] = core::array::from_fn(|i| (i as u8).wrapping_add(1));
    let block: GenericArray<u8, sha2::digest::typenum::U64> = GenericArray::clone_from_slice(&block_arr);

    let start = Instant::now();
    for _ in 0..iters {
        plain_hashes::sha256_compress(&mut state, core::slice::from_ref(&block));
        black_box(&state);
    }
    let elapsed = start.elapsed();
    let per_ns = elapsed.as_nanos() / iters as u128;
    println!("{label}: {iters} iters in {elapsed:?} ({per_ns} ns/iter)");
    black_box(state);
}

fn bench_keccak_f1600(label: &str, iters: usize) {
    let mut state: [u64; 25] = core::array::from_fn(|i| i as u64);

    let start = Instant::now();
    for _ in 0..iters {
        plain_hashes::keccak_f1600(&mut state);
        black_box(&state);
    }
    let elapsed = start.elapsed();
    let per_ns = elapsed.as_nanos() / iters as u128;
    println!("{label}: {iters} iters in {elapsed:?} ({per_ns} ns/iter)");
    black_box(state);
}

fn bench_blake2b_perm(label: &str, iters: usize) {
    let mut h: [u64; 8] = [
        0x6a09e667f3bcc908, 0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
        0x510e527fade682d1, 0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
    ];
    let m: [u64; 16] = core::array::from_fn(|i| i as u64);

    let start = Instant::now();
    for _ in 0..iters {
        plain_hashes::blake2b_compress(&mut h, &m, 0, 0, 0, 0);
        black_box(&h);
    }
    let elapsed = start.elapsed();
    let per_ns = elapsed.as_nanos() / iters as u128;
    println!("{label}: {iters} iters in {elapsed:?} ({per_ns} ns/iter)");
    black_box(h);
}

fn bench_blake3_perm(label: &str, iters: usize) {
    let cv: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    let block: [u8; 64] = core::array::from_fn(|i| i as u8);

    let start = Instant::now();
    for _ in 0..iters {
        let out = plain_hashes::blake3_compress(&cv, &block, 64, 0, 0);
        black_box(&out);
    }
    let elapsed = start.elapsed();
    let per_ns = elapsed.as_nanos() / iters as u128;
    println!("{label}: {iters} iters in {elapsed:?} ({per_ns} ns/iter)");
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
