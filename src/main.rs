use sok_zk_friendly_hash_functions::anemoi::anemoi::Anemoi;
use sok_zk_friendly_hash_functions::anemoi::instances::{
    ANEMOI_BABYBEAR_16_PARAMS, ANEMOI_BABYBEAR_24_PARAMS, ANEMOI_BLS12_381_2_PARAMS,
    ANEMOI_BN254_2_PARAMS, ANEMOI_GOLDILOCKS_8_PARAMS, ANEMOI_GOLDILOCKS_12_PARAMS,
    ANEMOI_KOALABEAR_16_PARAMS, ANEMOI_KOALABEAR_24_PARAMS, ANEMOI_MERSENNE31_16_PARAMS,
    ANEMOI_MERSENNE31_24_PARAMS,
};
use sok_zk_friendly_hash_functions::fields::{FieldElement, PrimeFieldWords};
use sok_zk_friendly_hash_functions::griffin::griffin::Griffin;
use sok_zk_friendly_hash_functions::griffin::instances::{
    GRIFFIN_BLS12_381_3_PARAMS, GRIFFIN_BN254_3_PARAMS, GRIFFIN_GOLDILOCKS_8_PARAMS,
    GRIFFIN_GOLDILOCKS_12_PARAMS,
};
use sok_zk_friendly_hash_functions::gmimc_erf::gmimc_erf::GmimcErf;
use sok_zk_friendly_hash_functions::gmimc_erf::instances::{
    GMIMC_ERF_ALPHA3_BABYBEAR_16_PARAMS, GMIMC_ERF_ALPHA3_BABYBEAR_24_PARAMS,
    GMIMC_ERF_ALPHA3_BLS12_381_2_PARAMS, GMIMC_ERF_ALPHA3_BLS12_381_3_PARAMS,
    GMIMC_ERF_ALPHA3_BN254_2_PARAMS, GMIMC_ERF_ALPHA3_BN254_3_PARAMS,
    GMIMC_ERF_ALPHA3_GOLDILOCKS_8_PARAMS, GMIMC_ERF_ALPHA3_GOLDILOCKS_12_PARAMS,
    GMIMC_ERF_ALPHA3_KOALABEAR_16_PARAMS, GMIMC_ERF_ALPHA3_KOALABEAR_24_PARAMS,
    GMIMC_ERF_ALPHA3_MERSENNE31_16_PARAMS, GMIMC_ERF_ALPHA3_MERSENNE31_24_PARAMS,
    GMIMC_ERF_BABYBEAR_16_PARAMS, GMIMC_ERF_BABYBEAR_24_PARAMS,
    GMIMC_ERF_BLS12_381_2_PARAMS, GMIMC_ERF_BLS12_381_3_PARAMS,
    GMIMC_ERF_BN254_2_PARAMS, GMIMC_ERF_BN254_3_PARAMS,
    GMIMC_ERF_GOLDILOCKS_8_PARAMS, GMIMC_ERF_GOLDILOCKS_12_PARAMS,
    GMIMC_ERF_KOALABEAR_16_PARAMS, GMIMC_ERF_KOALABEAR_24_PARAMS,
    GMIMC_ERF_MERSENNE31_16_PARAMS, GMIMC_ERF_MERSENNE31_24_PARAMS,
};
use sok_zk_friendly_hash_functions::monolith::instances::{
    MONOLITH_BABYBEAR_16_PARAMS, MONOLITH_BABYBEAR_24_PARAMS,
    MONOLITH_GOLDILOCKS_12_PARAMS, MONOLITH_GOLDILOCKS_8_PARAMS,
    MONOLITH_KOALABEAR_16_PARAMS, MONOLITH_KOALABEAR_24_PARAMS,
    MONOLITH_MERSENNE31_16_PARAMS, MONOLITH_MERSENNE31_24_PARAMS,
};
use sok_zk_friendly_hash_functions::monolith::monolith::{Monolith31, Monolith64};
use sok_zk_friendly_hash_functions::monolith::monolith_params::{MonolithField32, MonolithField64};
use sok_zk_friendly_hash_functions::neptune::neptune::Neptune;
use sok_zk_friendly_hash_functions::neptune::instances::{
    NEPTUNE_BABYBEAR_16_PARAMS, NEPTUNE_BABYBEAR_24_PARAMS, NEPTUNE_BLS12_381_2_PARAMS,
    NEPTUNE_BN254_2_PARAMS, NEPTUNE_GOLDILOCKS_8_PARAMS, NEPTUNE_GOLDILOCKS_12_PARAMS,
    NEPTUNE_KOALABEAR_16_PARAMS, NEPTUNE_KOALABEAR_24_PARAMS, NEPTUNE_MERSENNE31_16_PARAMS,
    NEPTUNE_MERSENNE31_24_PARAMS,
};
use sok_zk_friendly_hash_functions::polocolo::instances::{
    POLOCOLO_BLS12_381_3_PARAMS, POLOCOLO_BN254_3_PARAMS,
};
use sok_zk_friendly_hash_functions::polocolo::polocolo::Polocolo;
use sok_zk_friendly_hash_functions::poseidon::poseidon::Poseidon;
use sok_zk_friendly_hash_functions::poseidon::instances::{
    POSEIDON_BABYBEAR_16_PARAMS, POSEIDON_BABYBEAR_24_PARAMS, POSEIDON_BLS12_381_2_PARAMS,
    POSEIDON_BLS12_381_3_PARAMS, POSEIDON_BN254_2_PARAMS, POSEIDON_BN254_3_PARAMS,
    POSEIDON_GOLDILOCKS_8_PARAMS, POSEIDON_GOLDILOCKS_12_PARAMS, POSEIDON_KOALABEAR_16_PARAMS,
    POSEIDON_KOALABEAR_24_PARAMS, POSEIDON_MERSENNE31_16_PARAMS,
    POSEIDON_MERSENNE31_24_PARAMS,
};
use sok_zk_friendly_hash_functions::poseidon2::poseidon2::Poseidon2;
use sok_zk_friendly_hash_functions::poseidon2::instances::{
    POSEIDON2_BABYBEAR_16_PARAMS, POSEIDON2_BABYBEAR_24_PARAMS, POSEIDON2_BLS12_381_2_PARAMS,
    POSEIDON2_BLS12_381_3_PARAMS, POSEIDON2_BN254_2_PARAMS, POSEIDON2_BN254_3_PARAMS,
    POSEIDON2_GOLDILOCKS_8_PARAMS, POSEIDON2_GOLDILOCKS_12_PARAMS,
    POSEIDON2_KOALABEAR_16_PARAMS, POSEIDON2_KOALABEAR_24_PARAMS,
    POSEIDON2_MERSENNE31_16_PARAMS, POSEIDON2_MERSENNE31_24_PARAMS,
};
use sok_zk_friendly_hash_functions::reinforced_concrete::reinforced_concrete::ReinforcedConcrete;
use sok_zk_friendly_hash_functions::reinforced_concrete::instances::{
    REINFORCED_CONCRETE_BLS12_381_3_PARAMS, REINFORCED_CONCRETE_BN254_3_PARAMS,
};
use sok_zk_friendly_hash_functions::rescueprime::instances::{
    RESCUE_PRIME_BLS12_381_2_PARAMS, RESCUE_PRIME_BLS12_381_3_PARAMS,
    RESCUE_PRIME_BN254_3_PARAMS, RESCUE_PRIME_GOLDILOCKS_8_PARAMS,
    RESCUE_PRIME_GOLDILOCKS_12_PARAMS,
};
use sok_zk_friendly_hash_functions::rescueprime::rescue_prime::RescuePrime;
use sok_zk_friendly_hash_functions::skyscraper::instances::{
    SKYSCRAPER_BLS12_381_2_PARAMS, SKYSCRAPER_BLS12_381_3_PARAMS, SKYSCRAPER_BN254_2_PARAMS,
    SKYSCRAPER_BN254_3_PARAMS,
};
use sok_zk_friendly_hash_functions::skyscraper::skyscraper::Skyscraper;
use sok_zk_friendly_hash_functions::tip4::tip4::Tip4;
use sok_zk_friendly_hash_functions::tip4::instances::TIP4P_GOLDILOCKS_PARAMS;
use sok_zk_friendly_hash_functions::tip4::tip4::Tip4Field;
use sok_zk_friendly_hash_functions::tip5::tip5::Tip5;
use sok_zk_friendly_hash_functions::tip5::instances::TIP5_GOLDILOCKS_PARAMS;
use sok_zk_friendly_hash_functions::tip5::tip5::Tip5Field;
use sha2::{Digest, Sha256};
use sha3::Keccak256;
use std::hint::black_box;
use std::time::Instant;

const ITERS: usize = 1 << 14;

fn main() {
    println!("iters = {ITERS}");

    println!("\n== Poseidon (~256-bit fields) ==");
    bench_poseidon("Poseidon BN254 t=2", &Poseidon::new(&POSEIDON_BN254_2_PARAMS), ITERS);
    bench_poseidon("Poseidon BN254 t=3", &Poseidon::new(&POSEIDON_BN254_3_PARAMS), ITERS);
    bench_poseidon(
        "Poseidon BLS12-381 t=2",
        &Poseidon::new(&POSEIDON_BLS12_381_2_PARAMS),
        ITERS,
    );
    bench_poseidon(
        "Poseidon BLS12-381 t=3",
        &Poseidon::new(&POSEIDON_BLS12_381_3_PARAMS),
        ITERS,
    );

    println!("\n== Poseidon (~64-bit field) ==");
    bench_poseidon(
        "Poseidon Goldilocks t=8",
        &Poseidon::new(&POSEIDON_GOLDILOCKS_8_PARAMS),
        ITERS,
    );
    bench_poseidon(
        "Poseidon Goldilocks t=12",
        &Poseidon::new(&POSEIDON_GOLDILOCKS_12_PARAMS),
        ITERS,
    );

    println!("\n== Poseidon (~31-bit fields) ==");
    bench_poseidon(
        "Poseidon BabyBear t=16",
        &Poseidon::new(&POSEIDON_BABYBEAR_16_PARAMS),
        ITERS,
    );
    bench_poseidon(
        "Poseidon BabyBear t=24",
        &Poseidon::new(&POSEIDON_BABYBEAR_24_PARAMS),
        ITERS,
    );
    bench_poseidon(
        "Poseidon KoalaBear t=16",
        &Poseidon::new(&POSEIDON_KOALABEAR_16_PARAMS),
        ITERS,
    );
    bench_poseidon(
        "Poseidon KoalaBear t=24",
        &Poseidon::new(&POSEIDON_KOALABEAR_24_PARAMS),
        ITERS,
    );
    bench_poseidon(
        "Poseidon Mersenne31 t=16",
        &Poseidon::new(&POSEIDON_MERSENNE31_16_PARAMS),
        ITERS,
    );
    bench_poseidon(
        "Poseidon Mersenne31 t=24",
        &Poseidon::new(&POSEIDON_MERSENNE31_24_PARAMS),
        ITERS,
    );

    println!("\n== Poseidon2 (~256-bit fields) ==");
    bench_poseidon2(
        "Poseidon2 BN254 t=2",
        &Poseidon2::new(&POSEIDON2_BN254_2_PARAMS),
        ITERS,
    );
    bench_poseidon2(
        "Poseidon2 BN254 t=3",
        &Poseidon2::new(&POSEIDON2_BN254_3_PARAMS),
        ITERS,
    );
    bench_poseidon2(
        "Poseidon2 BLS12-381 t=2",
        &Poseidon2::new(&POSEIDON2_BLS12_381_2_PARAMS),
        ITERS,
    );
    bench_poseidon2(
        "Poseidon2 BLS12-381 t=3",
        &Poseidon2::new(&POSEIDON2_BLS12_381_3_PARAMS),
        ITERS,
    );

    println!("\n== Poseidon2 (~64-bit field) ==");
    bench_poseidon2(
        "Poseidon2 Goldilocks t=8",
        &Poseidon2::new(&POSEIDON2_GOLDILOCKS_8_PARAMS),
        ITERS,
    );
    bench_poseidon2(
        "Poseidon2 Goldilocks t=12",
        &Poseidon2::new(&POSEIDON2_GOLDILOCKS_12_PARAMS),
        ITERS,
    );

    println!("\n== Poseidon2 (~31-bit fields) ==");
    bench_poseidon2(
        "Poseidon2 BabyBear t=16",
        &Poseidon2::new(&POSEIDON2_BABYBEAR_16_PARAMS),
        ITERS,
    );
    bench_poseidon2(
        "Poseidon2 BabyBear t=24",
        &Poseidon2::new(&POSEIDON2_BABYBEAR_24_PARAMS),
        ITERS,
    );
    bench_poseidon2(
        "Poseidon2 KoalaBear t=16",
        &Poseidon2::new(&POSEIDON2_KOALABEAR_16_PARAMS),
        ITERS,
    );
    bench_poseidon2(
        "Poseidon2 KoalaBear t=24",
        &Poseidon2::new(&POSEIDON2_KOALABEAR_24_PARAMS),
        ITERS,
    );
    bench_poseidon2(
        "Poseidon2 Mersenne31 t=16",
        &Poseidon2::new(&POSEIDON2_MERSENNE31_16_PARAMS),
        ITERS,
    );
    bench_poseidon2(
        "Poseidon2 Mersenne31 t=24",
        &Poseidon2::new(&POSEIDON2_MERSENNE31_24_PARAMS),
        ITERS,
    );

    println!("\n== RescuePrime (state ~512) ==");
    bench_rescue(
        "RescuePrime BLS12-381 t=2",
        &RescuePrime::new(&RESCUE_PRIME_BLS12_381_2_PARAMS),
        ITERS,
    );
    bench_rescue(
        "RescuePrime Goldilocks t=8",
        &RescuePrime::new(&RESCUE_PRIME_GOLDILOCKS_8_PARAMS),
        ITERS,
    );

    println!("\n== RescuePrime (state ~768) ==");
    bench_rescue(
        "RescuePrime BN254 t=3",
        &RescuePrime::new(&RESCUE_PRIME_BN254_3_PARAMS),
        ITERS,
    );
    bench_rescue(
        "RescuePrime BLS12-381 t=3",
        &RescuePrime::new(&RESCUE_PRIME_BLS12_381_3_PARAMS),
        ITERS,
    );
    bench_rescue(
        "RescuePrime Goldilocks t=12",
        &RescuePrime::new(&RESCUE_PRIME_GOLDILOCKS_12_PARAMS),
        ITERS,
    );

    println!("\n== Anemoi (~256-bit fields) ==");
    bench_anemoi("Anemoi BN254 t=2", &Anemoi::new(&ANEMOI_BN254_2_PARAMS), ITERS);
    bench_anemoi(
        "Anemoi BLS12-381 t=2",
        &Anemoi::new(&ANEMOI_BLS12_381_2_PARAMS),
        ITERS,
    );

    println!("\n== Anemoi (~64-bit field) ==");
    bench_anemoi(
        "Anemoi Goldilocks t=8",
        &Anemoi::new(&ANEMOI_GOLDILOCKS_8_PARAMS),
        ITERS,
    );
    bench_anemoi(
        "Anemoi Goldilocks t=12",
        &Anemoi::new(&ANEMOI_GOLDILOCKS_12_PARAMS),
        ITERS,
    );

    println!("\n== Anemoi (~31-bit fields) ==");
    bench_anemoi(
        "Anemoi BabyBear t=16",
        &Anemoi::new(&ANEMOI_BABYBEAR_16_PARAMS),
        ITERS,
    );
    bench_anemoi(
        "Anemoi BabyBear t=24",
        &Anemoi::new(&ANEMOI_BABYBEAR_24_PARAMS),
        ITERS,
    );
    bench_anemoi(
        "Anemoi KoalaBear t=16",
        &Anemoi::new(&ANEMOI_KOALABEAR_16_PARAMS),
        ITERS,
    );
    bench_anemoi(
        "Anemoi KoalaBear t=24",
        &Anemoi::new(&ANEMOI_KOALABEAR_24_PARAMS),
        ITERS,
    );
    bench_anemoi(
        "Anemoi Mersenne31 t=16",
        &Anemoi::new(&ANEMOI_MERSENNE31_16_PARAMS),
        ITERS,
    );
    bench_anemoi(
        "Anemoi Mersenne31 t=24",
        &Anemoi::new(&ANEMOI_MERSENNE31_24_PARAMS),
        ITERS,
    );

    println!("\n== GMiMC-ERF (state ~512) ==");
    bench_gmimc(
        "GMiMC-ERF(alpha=2) BN254 t=2",
        &GmimcErf::new(&GMIMC_ERF_BN254_2_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=2) BLS12-381 t=2",
        &GmimcErf::new(&GMIMC_ERF_BLS12_381_2_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=2) Goldilocks t=8",
        &GmimcErf::new(&GMIMC_ERF_GOLDILOCKS_8_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=2) BabyBear t=16",
        &GmimcErf::new(&GMIMC_ERF_BABYBEAR_16_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=2) KoalaBear t=16",
        &GmimcErf::new(&GMIMC_ERF_KOALABEAR_16_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=2) Mersenne31 t=16",
        &GmimcErf::new(&GMIMC_ERF_MERSENNE31_16_PARAMS),
        ITERS,
    );

    println!("\n== GMiMC-ERF (state ~768) ==");
    bench_gmimc(
        "GMiMC-ERF(alpha=2) BN254 t=3",
        &GmimcErf::new(&GMIMC_ERF_BN254_3_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=2) BLS12-381 t=3",
        &GmimcErf::new(&GMIMC_ERF_BLS12_381_3_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=2) Goldilocks t=12",
        &GmimcErf::new(&GMIMC_ERF_GOLDILOCKS_12_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=2) BabyBear t=24",
        &GmimcErf::new(&GMIMC_ERF_BABYBEAR_24_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=2) KoalaBear t=24",
        &GmimcErf::new(&GMIMC_ERF_KOALABEAR_24_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=2) Mersenne31 t=24",
        &GmimcErf::new(&GMIMC_ERF_MERSENNE31_24_PARAMS),
        ITERS,
    );

    println!("\n== GMiMC-ERF(alpha=3) (state ~512) ==");
    bench_gmimc(
        "GMiMC-ERF(alpha=3) BN254 t=2",
        &GmimcErf::new(&GMIMC_ERF_ALPHA3_BN254_2_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=3) BLS12-381 t=2",
        &GmimcErf::new(&GMIMC_ERF_ALPHA3_BLS12_381_2_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=3) Goldilocks t=8",
        &GmimcErf::new(&GMIMC_ERF_ALPHA3_GOLDILOCKS_8_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=3) BabyBear t=16",
        &GmimcErf::new(&GMIMC_ERF_ALPHA3_BABYBEAR_16_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=3) KoalaBear t=16",
        &GmimcErf::new(&GMIMC_ERF_ALPHA3_KOALABEAR_16_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=3) Mersenne31 t=16",
        &GmimcErf::new(&GMIMC_ERF_ALPHA3_MERSENNE31_16_PARAMS),
        ITERS,
    );

    println!("\n== GMiMC-ERF(alpha=3) (state ~768) ==");
    bench_gmimc(
        "GMiMC-ERF(alpha=3) BN254 t=3",
        &GmimcErf::new(&GMIMC_ERF_ALPHA3_BN254_3_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=3) BLS12-381 t=3",
        &GmimcErf::new(&GMIMC_ERF_ALPHA3_BLS12_381_3_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=3) Goldilocks t=12",
        &GmimcErf::new(&GMIMC_ERF_ALPHA3_GOLDILOCKS_12_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=3) BabyBear t=24",
        &GmimcErf::new(&GMIMC_ERF_ALPHA3_BABYBEAR_24_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=3) KoalaBear t=24",
        &GmimcErf::new(&GMIMC_ERF_ALPHA3_KOALABEAR_24_PARAMS),
        ITERS,
    );
    bench_gmimc(
        "GMiMC-ERF(alpha=3) Mersenne31 t=24",
        &GmimcErf::new(&GMIMC_ERF_ALPHA3_MERSENNE31_24_PARAMS),
        ITERS,
    );

    println!("\n== Griffin (~256-bit fields) ==");
    bench_griffin("Griffin BN254 t=3", &Griffin::new(&GRIFFIN_BN254_3_PARAMS), ITERS);
    bench_griffin(
        "Griffin BLS12-381 t=3",
        &Griffin::new(&GRIFFIN_BLS12_381_3_PARAMS),
        ITERS,
    );

    println!("\n== Griffin (~64-bit field) ==");
    bench_griffin(
        "Griffin Goldilocks t=8",
        &Griffin::new(&GRIFFIN_GOLDILOCKS_8_PARAMS),
        ITERS,
    );
    bench_griffin(
        "Griffin Goldilocks t=12",
        &Griffin::new(&GRIFFIN_GOLDILOCKS_12_PARAMS),
        ITERS,
    );

    println!("\n== Neptune (~256-bit fields) ==");
    bench_neptune("Neptune BN254 t=2", &Neptune::new(&NEPTUNE_BN254_2_PARAMS), ITERS);
    bench_neptune(
        "Neptune BLS12-381 t=2",
        &Neptune::new(&NEPTUNE_BLS12_381_2_PARAMS),
        ITERS,
    );

    println!("\n== Neptune (~64-bit field) ==");
    bench_neptune(
        "Neptune Goldilocks t=8",
        &Neptune::new(&NEPTUNE_GOLDILOCKS_8_PARAMS),
        ITERS,
    );
    bench_neptune(
        "Neptune Goldilocks t=12",
        &Neptune::new(&NEPTUNE_GOLDILOCKS_12_PARAMS),
        ITERS,
    );

    println!("\n== Neptune (~31-bit fields) ==");
    bench_neptune(
        "Neptune BabyBear t=16",
        &Neptune::new(&NEPTUNE_BABYBEAR_16_PARAMS),
        ITERS,
    );
    bench_neptune(
        "Neptune KoalaBear t=16",
        &Neptune::new(&NEPTUNE_KOALABEAR_16_PARAMS),
        ITERS,
    );
    bench_neptune(
        "Neptune Mersenne31 t=16",
        &Neptune::new(&NEPTUNE_MERSENNE31_16_PARAMS),
        ITERS,
    );
    bench_neptune(
        "Neptune BabyBear t=24",
        &Neptune::new(&NEPTUNE_BABYBEAR_24_PARAMS),
        ITERS,
    );
    bench_neptune(
        "Neptune KoalaBear t=24",
        &Neptune::new(&NEPTUNE_KOALABEAR_24_PARAMS),
        ITERS,
    );
    bench_neptune(
        "Neptune Mersenne31 t=24",
        &Neptune::new(&NEPTUNE_MERSENNE31_24_PARAMS),
        ITERS,
    );

    println!("\n== Polocolo (~256-bit fields) ==");
    bench_polocolo(
        "Polocolo BN254 t=3",
        &Polocolo::new(&POLOCOLO_BN254_3_PARAMS),
        ITERS,
    );
    bench_polocolo(
        "Polocolo BLS12-381 t=3",
        &Polocolo::new(&POLOCOLO_BLS12_381_3_PARAMS),
        ITERS,
    );

    println!("\n== Skyscraper (~256-bit fields) ==");
    bench_skyscraper(
        "Skyscraper BN254 n=2",
        &Skyscraper::new(&SKYSCRAPER_BN254_2_PARAMS),
        ITERS,
    );
    bench_skyscraper(
        "Skyscraper BN254 n=3",
        &Skyscraper::new(&SKYSCRAPER_BN254_3_PARAMS),
        ITERS,
    );
    bench_skyscraper(
        "Skyscraper BLS12-381 n=2",
        &Skyscraper::new(&SKYSCRAPER_BLS12_381_2_PARAMS),
        ITERS,
    );
    bench_skyscraper(
        "Skyscraper BLS12-381 n=3",
        &Skyscraper::new(&SKYSCRAPER_BLS12_381_3_PARAMS),
        ITERS,
    );

    println!("\n== ReinforcedConcrete (state ~762/~765) ==");
    bench_reinforced_concrete(
        "ReinforcedConcrete BN254 t=3",
        &ReinforcedConcrete::new(&REINFORCED_CONCRETE_BN254_3_PARAMS),
        ITERS,
    );
    bench_reinforced_concrete(
        "ReinforcedConcrete BLS12-381 t=3",
        &ReinforcedConcrete::new(&REINFORCED_CONCRETE_BLS12_381_3_PARAMS),
        ITERS,
    );

    println!("\n== Monolith (state ~512) ==");
    bench_monolith64(
        "Monolith Goldilocks t=8",
        &Monolith64::new(&MONOLITH_GOLDILOCKS_8_PARAMS),
        ITERS,
    );
    bench_monolith31(
        "Monolith Mersenne31 t=16",
        &Monolith31::new(&MONOLITH_MERSENNE31_16_PARAMS),
        ITERS,
    );
    bench_monolith31(
        "Monolith BabyBear t=16",
        &Monolith31::new(&MONOLITH_BABYBEAR_16_PARAMS),
        ITERS,
    );
    bench_monolith31(
        "Monolith KoalaBear t=16",
        &Monolith31::new(&MONOLITH_KOALABEAR_16_PARAMS),
        ITERS,
    );

    println!("\n== Monolith (state ~768) ==");
    bench_monolith64(
        "Monolith Goldilocks t=12",
        &Monolith64::new(&MONOLITH_GOLDILOCKS_12_PARAMS),
        ITERS,
    );
    bench_monolith31(
        "Monolith Mersenne31 t=24",
        &Monolith31::new(&MONOLITH_MERSENNE31_24_PARAMS),
        ITERS,
    );
    bench_monolith31(
        "Monolith BabyBear t=24",
        &Monolith31::new(&MONOLITH_BABYBEAR_24_PARAMS),
        ITERS,
    );
    bench_monolith31(
        "Monolith KoalaBear t=24",
        &Monolith31::new(&MONOLITH_KOALABEAR_24_PARAMS),
        ITERS,
    );

    println!("\n== Tip4' (Goldilocks) ==");
    bench_tip4(
        "Tip4' Goldilocks",
        &Tip4::new(&TIP4P_GOLDILOCKS_PARAMS),
        ITERS,
    );

    println!("\n== Tip5 (Goldilocks) ==");
    bench_tip5(
        "Tip5 Goldilocks",
        &Tip5::new(&TIP5_GOLDILOCKS_PARAMS),
        ITERS,
    );

    println!("\n== SHA2/Keccak (bytes, baseline) ==");
    bench_sha256("SHA-256 input=64B", ITERS, 64);
    bench_keccak256("Keccak-256 input=64B", ITERS, 64);
    bench_sha256("SHA-256 input=96B", ITERS, 96);
    bench_keccak256("Keccak-256 input=96B", ITERS, 96);
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

fn bench_skyscraper<F: PrimeFieldWords>(label: &str, perm: &Skyscraper<F>, iters: usize) {
    let input = make_input::<F>(2 * perm.get_n());
    bench_with_input(label, iters, &input, |inp| perm.permutation(inp));
}

fn bench_reinforced_concrete<F: PrimeFieldWords>(
    label: &str,
    perm: &ReinforcedConcrete<F>,
    iters: usize,
) {
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

fn bench_tip4<F: Tip4Field>(label: &str, perm: &Tip4<F>, iters: usize) {
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

fn bench_sha256(label: &str, iters: usize, input_len: usize) {
    let input = make_bytes_input(input_len);
    bench_with_bytes(label, iters, &input, |inp| Sha256::digest(inp));
}

fn bench_keccak256(label: &str, iters: usize, input_len: usize) {
    let input = make_bytes_input(input_len);
    bench_with_bytes(label, iters, &input, |inp| Keccak256::digest(inp));
}

fn make_input<F: FieldElement>(t: usize) -> Vec<F> {
    (0..t).map(|i| F::from_u64((i + 1) as u64)).collect()
}

fn make_bytes_input(len: usize) -> Vec<u8> {
    (0..len).map(|i| (i as u8).wrapping_add(1)).collect()
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

fn bench_with_bytes<R, FFn: FnMut(&[u8]) -> R>(
    label: &str,
    iters: usize,
    input: &[u8],
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
