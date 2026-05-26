use super::sgmimc::SgmiMcParams;
use crate::fields::babybear::BabyBear;
use crate::fields::bls12_381::Bls12_381;
use crate::fields::bn254::Bn254;
use crate::fields::goldilocks::Goldilocks;
use crate::fields::koalabear::KoalaBear;
use crate::fields::mersenne31::Mersenne31;
use crate::fields::FieldElement;
use lazy_static::lazy_static;
use std::sync::Arc;

/// Generate deterministic round constants for S-GMiMC.
///
/// Uses a simple counter-based scheme: c_i = i (as field element).
/// The exact constants don't affect performance — only the round count matters.
fn generate_round_constants<F: FieldElement>(rounds: usize) -> Vec<F> {
    (0..rounds).map(|i| F::from_u64(i as u64)).collect()
}

// ============================================================
// Round-number formula (from the SoK paper's updated analysis):
//
// For 256-bit primes (κ = 128, c = 1):
//   R = 2·t + ⌈κ · log_α(2)⌉
//
// For 64-bit primes (κ = 128, c = 4):
//   R estimated as the maximum of differential, interpolation,
//   and Gröbner-basis security requirements.
//
// For 31-bit primes (κ = 128, c = 8):
//   R dominated by differential and interpolation attacks.
// ============================================================

// --- α = 2 (square S-box) ---

lazy_static! {
    // 256-bit fields
    pub static ref SGMIMC_BN254_2_PARAMS: Arc<SgmiMcParams<Bn254>> = {
        let rounds = 132; // R = 2*2 + 128
        let round_constants = generate_round_constants::<Bn254>(rounds);
        Arc::new(SgmiMcParams::new(2, 2, rounds, &round_constants))
    };
    pub static ref SGMIMC_BN254_3_PARAMS: Arc<SgmiMcParams<Bn254>> = {
        let rounds = 134; // R = 2*3 + 128
        Arc::new(SgmiMcParams::new(3, 2, rounds, &generate_round_constants::<Bn254>(rounds)))
    };
    pub static ref SGMIMC_BLS12_381_2_PARAMS: Arc<SgmiMcParams<Bls12_381>> = {
        let rounds = 132;
        Arc::new(SgmiMcParams::new(2, 2, rounds, &generate_round_constants::<Bls12_381>(rounds)))
    };
    pub static ref SGMIMC_BLS12_381_3_PARAMS: Arc<SgmiMcParams<Bls12_381>> = {
        let rounds = 134;
        Arc::new(SgmiMcParams::new(3, 2, rounds, &generate_round_constants::<Bls12_381>(rounds)))
    };

    // 64-bit field (Goldilocks)
    pub static ref SGMIMC_GOLDILOCKS_8_PARAMS: Arc<SgmiMcParams<Goldilocks>> = {
        let rounds = 100;
        Arc::new(SgmiMcParams::new(8, 2, rounds, &generate_round_constants::<Goldilocks>(rounds)))
    };
    pub static ref SGMIMC_GOLDILOCKS_12_PARAMS: Arc<SgmiMcParams<Goldilocks>> = {
        let rounds = 110;
        Arc::new(SgmiMcParams::new(12, 2, rounds, &generate_round_constants::<Goldilocks>(rounds)))
    };

    // 31-bit fields
    pub static ref SGMIMC_BABYBEAR_16_PARAMS: Arc<SgmiMcParams<BabyBear>> = {
        let rounds = 180;
        Arc::new(SgmiMcParams::new(16, 2, rounds, &generate_round_constants::<BabyBear>(rounds)))
    };
    pub static ref SGMIMC_BABYBEAR_24_PARAMS: Arc<SgmiMcParams<BabyBear>> = {
        let rounds = 270;
        Arc::new(SgmiMcParams::new(24, 2, rounds, &generate_round_constants::<BabyBear>(rounds)))
    };
    pub static ref SGMIMC_KOALABEAR_16_PARAMS: Arc<SgmiMcParams<KoalaBear>> = {
        let rounds = 180;
        Arc::new(SgmiMcParams::new(16, 2, rounds, &generate_round_constants::<KoalaBear>(rounds)))
    };
    pub static ref SGMIMC_KOALABEAR_24_PARAMS: Arc<SgmiMcParams<KoalaBear>> = {
        let rounds = 270;
        Arc::new(SgmiMcParams::new(24, 2, rounds, &generate_round_constants::<KoalaBear>(rounds)))
    };
    pub static ref SGMIMC_MERSENNE31_16_PARAMS: Arc<SgmiMcParams<Mersenne31>> = {
        let rounds = 180;
        Arc::new(SgmiMcParams::new(16, 2, rounds, &generate_round_constants::<Mersenne31>(rounds)))
    };
    pub static ref SGMIMC_MERSENNE31_24_PARAMS: Arc<SgmiMcParams<Mersenne31>> = {
        let rounds = 270;
        Arc::new(SgmiMcParams::new(24, 2, rounds, &generate_round_constants::<Mersenne31>(rounds)))
    };
}

// --- α = 8 (for comparison on 256-bit fields, as suggested by Matthias) ---

lazy_static! {
    pub static ref SGMIMC_ALPHA8_BN254_2_PARAMS: Arc<SgmiMcParams<Bn254>> = {
        let rounds = 47; // R = 2*2 + ceil(128*log_8(2)) = 4 + 43 = 47
        Arc::new(SgmiMcParams::new(2, 8, rounds, &generate_round_constants::<Bn254>(rounds)))
    };
    pub static ref SGMIMC_ALPHA8_BN254_3_PARAMS: Arc<SgmiMcParams<Bn254>> = {
        let rounds = 49; // R = 2*3 + 43 = 49
        Arc::new(SgmiMcParams::new(3, 8, rounds, &generate_round_constants::<Bn254>(rounds)))
    };
    pub static ref SGMIMC_ALPHA8_BLS12_381_2_PARAMS: Arc<SgmiMcParams<Bls12_381>> = {
        let rounds = 47;
        Arc::new(SgmiMcParams::new(2, 8, rounds, &generate_round_constants::<Bls12_381>(rounds)))
    };
    pub static ref SGMIMC_ALPHA8_BLS12_381_3_PARAMS: Arc<SgmiMcParams<Bls12_381>> = {
        let rounds = 49;
        Arc::new(SgmiMcParams::new(3, 8, rounds, &generate_round_constants::<Bls12_381>(rounds)))
    };
}
