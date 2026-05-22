use sok_zk_friendly_hash_functions::arion::arion::Arion;
use sok_zk_friendly_hash_functions::arion::instances::{
    ARION_BLS12_381_3_PARAMS,
    ARION_BN254_3_PARAMS, ARION_GOLDILOCKS_12_PARAMS, ARION_GOLDILOCKS_8_PARAMS,
};
use sok_zk_friendly_hash_functions::xhash::instances::{
    XHASH_BABYBEAR_16_PARAMS, XHASH_BABYBEAR_24_PARAMS, XHASH_BLS12_381_2_PARAMS,
    XHASH_BLS12_381_3_PARAMS, XHASH_BN254_2_PARAMS, XHASH_BN254_3_PARAMS,
    XHASH_GOLDILOCKS_12_PARAMS, XHASH_GOLDILOCKS_8_PARAMS, XHASH_KOALABEAR_16_PARAMS,
    XHASH_KOALABEAR_24_PARAMS, XHASH_MERSENNE31_16_PARAMS, XHASH_MERSENNE31_24_PARAMS,
};
use sok_zk_friendly_hash_functions::xhash::xhash::XHash;
// Vision-mark32 removed from benchmark: designed for binary tower fields,
// not suitable for prime-field comparison (S-box inversion only fast on F_2^32).
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
    MONOLITH_CAUCHY_BABYBEAR_16_PARAMS, MONOLITH_CAUCHY_BABYBEAR_24_PARAMS,
    MONOLITH_CAUCHY_GOLDILOCKS_12_PARAMS, MONOLITH_CAUCHY_GOLDILOCKS_8_PARAMS,
    MONOLITH_CAUCHY_KOALABEAR_16_PARAMS, MONOLITH_CAUCHY_KOALABEAR_24_PARAMS,
    MONOLITH_CAUCHY_MERSENNE31_16_PARAMS, MONOLITH_CAUCHY_MERSENNE31_24_PARAMS,
};
use sok_zk_friendly_hash_functions::monolith::monolith::{MonolithCauchy31, MonolithCauchy64};
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
use sok_zk_friendly_hash_functions::plain_hashes;
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

    println!("\n== Monolith-Cauchy (state ~512) ==");
    bench_monolith_cauchy64(
        "Monolith Goldilocks t=8",
        &MonolithCauchy64::new(&MONOLITH_CAUCHY_GOLDILOCKS_8_PARAMS),
        ITERS,
    );
    bench_monolith_cauchy31(
        "Monolith Mersenne31 t=16",
        &MonolithCauchy31::new(&MONOLITH_CAUCHY_MERSENNE31_16_PARAMS),
        ITERS,
    );
    bench_monolith_cauchy31(
        "Monolith BabyBear t=16",
        &MonolithCauchy31::new(&MONOLITH_CAUCHY_BABYBEAR_16_PARAMS),
        ITERS,
    );
    bench_monolith_cauchy31(
        "Monolith KoalaBear t=16",
        &MonolithCauchy31::new(&MONOLITH_CAUCHY_KOALABEAR_16_PARAMS),
        ITERS,
    );

    println!("\n== Monolith-Cauchy (state ~768) ==");
    bench_monolith_cauchy64(
        "Monolith Goldilocks t=12",
        &MonolithCauchy64::new(&MONOLITH_CAUCHY_GOLDILOCKS_12_PARAMS),
        ITERS,
    );
    bench_monolith_cauchy31(
        "Monolith Mersenne31 t=24",
        &MonolithCauchy31::new(&MONOLITH_CAUCHY_MERSENNE31_24_PARAMS),
        ITERS,
    );
    bench_monolith_cauchy31(
        "Monolith BabyBear t=24",
        &MonolithCauchy31::new(&MONOLITH_CAUCHY_BABYBEAR_24_PARAMS),
        ITERS,
    );
    bench_monolith_cauchy31(
        "Monolith KoalaBear t=24",
        &MonolithCauchy31::new(&MONOLITH_CAUCHY_KOALABEAR_24_PARAMS),
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

    println!("\n== Arion (~256-bit fields) ==");
    bench_arion("Arion BN254 t=3", &Arion::new(&ARION_BN254_3_PARAMS), ITERS);
    bench_arion(
        "Arion BLS12-381 t=3",
        &Arion::new(&ARION_BLS12_381_3_PARAMS),
        ITERS,
    );

    println!("\n== Arion (~64-bit field) ==");
    bench_arion(
        "Arion Goldilocks t=8",
        &Arion::new(&ARION_GOLDILOCKS_8_PARAMS),
        ITERS,
    );
    bench_arion(
        "Arion Goldilocks t=12",
        &Arion::new(&ARION_GOLDILOCKS_12_PARAMS),
        ITERS,
    );

    // Arion is designed for large prime fields (≥256-bit).
    // 31-bit fields are not its design target — excluded from benchmark.

    println!("\n== XHash (~256-bit fields) ==");
    bench_xhash("XHash BN254 t=2", &XHash::new(&XHASH_BN254_2_PARAMS), ITERS);
    bench_xhash("XHash BN254 t=3", &XHash::new(&XHASH_BN254_3_PARAMS), ITERS);
    bench_xhash(
        "XHash BLS12-381 t=2",
        &XHash::new(&XHASH_BLS12_381_2_PARAMS),
        ITERS,
    );
    bench_xhash(
        "XHash BLS12-381 t=3",
        &XHash::new(&XHASH_BLS12_381_3_PARAMS),
        ITERS,
    );

    println!("\n== XHash (~64-bit field) ==");
    bench_xhash(
        "XHash Goldilocks t=8",
        &XHash::new(&XHASH_GOLDILOCKS_8_PARAMS),
        ITERS,
    );
    bench_xhash(
        "XHash Goldilocks t=12",
        &XHash::new(&XHASH_GOLDILOCKS_12_PARAMS),
        ITERS,
    );

    println!("\n== XHash (~31-bit fields) ==");
    bench_xhash(
        "XHash Mersenne31 t=16",
        &XHash::new(&XHASH_MERSENNE31_16_PARAMS),
        ITERS,
    );
    bench_xhash(
        "XHash Mersenne31 t=24",
        &XHash::new(&XHASH_MERSENNE31_24_PARAMS),
        ITERS,
    );
    bench_xhash(
        "XHash BabyBear t=16",
        &XHash::new(&XHASH_BABYBEAR_16_PARAMS),
        ITERS,
    );
    bench_xhash(
        "XHash BabyBear t=24",
        &XHash::new(&XHASH_BABYBEAR_24_PARAMS),
        ITERS,
    );
    bench_xhash(
        "XHash KoalaBear t=16",
        &XHash::new(&XHASH_KOALABEAR_16_PARAMS),
        ITERS,
    );
    bench_xhash(
        "XHash KoalaBear t=24",
        &XHash::new(&XHASH_KOALABEAR_24_PARAMS),
        ITERS,
    );

    // Vision-mark32 removed: designed for binary tower fields (F_2^32),
    // S-box inversion only fast on binary fields, not on prime fields.

    println!("\n== SHA-256 / Keccak-f / Blake2b / Blake3 (permutations) ==");
    bench_sha256_perm("SHA-256 compress (1 block)", ITERS);
    bench_keccak_f1600("Keccak-f[1600] (24 rounds)", ITERS);
    bench_blake2b_perm("Blake2b compress (1 block)", ITERS);
    bench_blake3_perm("Blake3 compress (1 block)", ITERS);
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

fn bench_monolith_cauchy64<F: MonolithField64>(label: &str, perm: &MonolithCauchy64<F>, iters: usize) {
    let input = make_input::<F>(perm.get_t());
    bench_with_input(label, iters, &input, |inp| perm.permutation(inp));
}

fn bench_monolith_cauchy31<F: MonolithField32>(label: &str, perm: &MonolithCauchy31<F>, iters: usize) {
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

fn bench_arion<F: FieldElement>(label: &str, perm: &Arion<F>, iters: usize) {
    let input = make_input::<F>(perm.get_t());
    bench_with_input(label, iters, &input, |inp| perm.permutation(inp));
}

fn bench_xhash<F: FieldElement>(label: &str, perm: &XHash<F>, iters: usize) {
    let input = make_input::<F>(perm.get_t());
    bench_with_input(label, iters, &input, |inp| perm.permutation(inp));
}

fn bench_sha256_perm(label: &str, iters: usize) {
    // SHA-256 IV (NIST FIPS 180-4 §5.3.3)
    // sha2 crate compress256 takes &[GenericArray<u8, U64>]
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
    // keccak crate → f1600(&mut [u64; 25])
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
    // Adapted from blake2 crate (RustCrypto) macros.rs
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
    // Adapted from blake3 crate (BLAKE3-team) portable.rs
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


