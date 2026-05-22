use super::monolith_params::{
    Monolith31Params, Monolith64Params, MonolithCauchy64Params, MonolithCauchy31Params,
    MonolithField32, MonolithField64,
};
use crate::fields::babybear::BabyBear;
use crate::fields::goldilocks::Goldilocks;
use crate::fields::koalabear::KoalaBear;
use crate::fields::mersenne31::Mersenne31;
use crate::utils::build_cauchy_mds;
use lazy_static::lazy_static;
use std::sync::Arc;

// ============================================================
// MonolithField implementations (same for both circulant & Cauchy)
// ============================================================

impl MonolithField64 for Goldilocks {
    fn to_u64(&self) -> u64 {
        Goldilocks::to_u64(self)
    }

    fn modulus_u64() -> u64 {
        Goldilocks::MODULUS
    }
}

impl MonolithField32 for Mersenne31 {
    fn to_u32(&self) -> u32 {
        Mersenne31::to_u32(self)
    }

    fn modulus_u32() -> u32 {
        Mersenne31::MODULUS
    }
}

impl MonolithField32 for BabyBear {
    fn to_u32(&self) -> u32 {
        BabyBear::to_u32(self)
    }

    fn modulus_u32() -> u32 {
        BabyBear::MODULUS
    }
}

impl MonolithField32 for KoalaBear {
    fn to_u32(&self) -> u32 {
        KoalaBear::to_u32(self)
    }

    fn modulus_u32() -> u32 {
        KoalaBear::MODULUS
    }
}

// ============================================================
// Original circulant-MDS instances (kept for reference)
// ============================================================

lazy_static! {
    pub static ref MONOLITH_GOLDILOCKS_8_PARAMS: Arc<Monolith64Params<Goldilocks>> =
        Arc::new(Monolith64Params::new(8));
    pub static ref MONOLITH_GOLDILOCKS_12_PARAMS: Arc<Monolith64Params<Goldilocks>> =
        Arc::new(Monolith64Params::new(12));

    pub static ref MONOLITH_MERSENNE31_16_PARAMS: Arc<Monolith31Params<Mersenne31>> =
        Arc::new(Monolith31Params::new(16));
    pub static ref MONOLITH_MERSENNE31_24_PARAMS: Arc<Monolith31Params<Mersenne31>> =
        Arc::new(Monolith31Params::new(24));

    pub static ref MONOLITH_BABYBEAR_16_PARAMS: Arc<Monolith31Params<BabyBear>> =
        Arc::new(Monolith31Params::new(16));
    pub static ref MONOLITH_BABYBEAR_24_PARAMS: Arc<Monolith31Params<BabyBear>> =
        Arc::new(Monolith31Params::new(24));

    pub static ref MONOLITH_KOALABEAR_16_PARAMS: Arc<Monolith31Params<KoalaBear>> =
        Arc::new(Monolith31Params::new(16));
    pub static ref MONOLITH_KOALABEAR_24_PARAMS: Arc<Monolith31Params<KoalaBear>> =
        Arc::new(Monolith31Params::new(24));
}

// ============================================================
// Cauchy-MDS instances (used for fair benchmark)
// ============================================================

/// Helper: convert a Cauchy MDS matrix from one PrimeField type to MonolithField type.
/// Both use the same underlying field modulus, so the BigUint inverses are identical.
fn cauchy_mds_64<F: MonolithField64>(t: usize) -> Vec<Vec<F>> {
    build_cauchy_mds::<Goldilocks>(t)
        .into_iter()
        .map(|row| row.into_iter().map(|x| F::from_u64(Goldilocks::to_u64(&x))).collect())
        .collect()
}

fn cauchy_mds_31_m31<F: MonolithField32>(t: usize) -> Vec<Vec<F>> {
    build_cauchy_mds::<Mersenne31>(t)
        .into_iter()
        .map(|row| row.into_iter().map(|x| F::from_u64(Mersenne31::to_u32(&x) as u64)).collect())
        .collect()
}

fn cauchy_mds_31_bb<F: MonolithField32>(t: usize) -> Vec<Vec<F>> {
    build_cauchy_mds::<BabyBear>(t)
        .into_iter()
        .map(|row| row.into_iter().map(|x| F::from_u64(BabyBear::to_u32(&x) as u64)).collect())
        .collect()
}

fn cauchy_mds_31_kb<F: MonolithField32>(t: usize) -> Vec<Vec<F>> {
    build_cauchy_mds::<KoalaBear>(t)
        .into_iter()
        .map(|row| row.into_iter().map(|x| F::from_u64(KoalaBear::to_u32(&x) as u64)).collect())
        .collect()
}

lazy_static! {
    // -- Goldilocks (64-bit field) --
    pub static ref MONOLITH_CAUCHY_GOLDILOCKS_8_PARAMS: Arc<MonolithCauchy64Params<Goldilocks>> =
        Arc::new(MonolithCauchy64Params::new(8, cauchy_mds_64::<Goldilocks>(8)));
    pub static ref MONOLITH_CAUCHY_GOLDILOCKS_12_PARAMS: Arc<MonolithCauchy64Params<Goldilocks>> =
        Arc::new(MonolithCauchy64Params::new(12, cauchy_mds_64::<Goldilocks>(12)));

    // -- Mersenne31 --
    pub static ref MONOLITH_CAUCHY_MERSENNE31_16_PARAMS: Arc<MonolithCauchy31Params<Mersenne31>> =
        Arc::new(MonolithCauchy31Params::new(16, cauchy_mds_31_m31::<Mersenne31>(16)));
    pub static ref MONOLITH_CAUCHY_MERSENNE31_24_PARAMS: Arc<MonolithCauchy31Params<Mersenne31>> =
        Arc::new(MonolithCauchy31Params::new(24, cauchy_mds_31_m31::<Mersenne31>(24)));

    // -- BabyBear --
    pub static ref MONOLITH_CAUCHY_BABYBEAR_16_PARAMS: Arc<MonolithCauchy31Params<BabyBear>> =
        Arc::new(MonolithCauchy31Params::new(16, cauchy_mds_31_bb::<BabyBear>(16)));
    pub static ref MONOLITH_CAUCHY_BABYBEAR_24_PARAMS: Arc<MonolithCauchy31Params<BabyBear>> =
        Arc::new(MonolithCauchy31Params::new(24, cauchy_mds_31_bb::<BabyBear>(24)));

    // -- KoalaBear --
    pub static ref MONOLITH_CAUCHY_KOALABEAR_16_PARAMS: Arc<MonolithCauchy31Params<KoalaBear>> =
        Arc::new(MonolithCauchy31Params::new(16, cauchy_mds_31_kb::<KoalaBear>(16)));
    pub static ref MONOLITH_CAUCHY_KOALABEAR_24_PARAMS: Arc<MonolithCauchy31Params<KoalaBear>> =
        Arc::new(MonolithCauchy31Params::new(24, cauchy_mds_31_kb::<KoalaBear>(24)));
}
