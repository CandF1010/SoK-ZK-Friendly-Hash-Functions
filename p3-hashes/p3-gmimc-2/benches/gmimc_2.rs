use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use p3_baby_bear::BabyBear;
use p3_field::PrimeField;
use p3_koala_bear::KoalaBear;
use p3_mersenne_31::Mersenne31;
use p3_goldilocks::Goldilocks;
use p3_bn254::Bn254;
use p3_gmimc_2::GMiMC2;
use p3_symmetric::Permutation;
use p3_util::pretty_name;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::distr::{Distribution, StandardUniform};

// 32 bit fields
const ROUNDS_16: usize = 176;
const ROUNDS_24: usize = 264;
// 64 bit fields
const ROUNDS_8_2: usize = 168;
const ROUNDS_8_4: usize = 88;
const ROUNDS_8_8: usize = 64;
const ROUNDS_12_2: usize = 168;
const ROUNDS_12_4: usize = 96;
const ROUNDS_12_8: usize = 72;
// 256 bit fields
const ROUNDS_4_4: usize = 92;
const ROUNDS_4_8: usize = 64;

fn bench_gmimc_2(c: &mut Criterion) {
    // BabyBear
    gmimc_2::<BabyBear, 16, 2, ROUNDS_16>(c);
    gmimc_2::<BabyBear, 24, 2, ROUNDS_24>(c);

    gmimc_2::<BabyBear, 16, 4, ROUNDS_16>(c);
    gmimc_2::<BabyBear, 16, 8, ROUNDS_16>(c);

    // KoalaBear
    gmimc_2::<KoalaBear, 16, 2,ROUNDS_16>(c);
    gmimc_2::<KoalaBear, 24, 2,ROUNDS_24>(c);

    gmimc_2::<KoalaBear, 16, 4,ROUNDS_16>(c);
    gmimc_2::<KoalaBear, 16, 8,ROUNDS_16>(c);

    // Mersenne31
    gmimc_2::<Mersenne31, 16, 2,ROUNDS_16>(c);
    gmimc_2::<Mersenne31, 24, 2, ROUNDS_24>(c);

    gmimc_2::<Mersenne31, 16, 4,ROUNDS_16>(c);
    gmimc_2::<Mersenne31, 16, 8,ROUNDS_16>(c);

    // Goldilocks
    gmimc_2::<Goldilocks, 8, 2,ROUNDS_8_2>(c);
    gmimc_2::<Goldilocks, 12, 2,ROUNDS_12_2>(c);

    gmimc_2::<Goldilocks, 8, 4,ROUNDS_8_4>(c);
    gmimc_2::<Goldilocks, 12, 4,ROUNDS_12_4>(c);

    gmimc_2::<Goldilocks, 8, 8,ROUNDS_8_8>(c);
    gmimc_2::<Goldilocks, 12, 8,ROUNDS_12_8>(c);

    // BN254
    gmimc_2::<Bn254, 4, 4,ROUNDS_4_4>(c);
    gmimc_2::<Bn254, 4, 8,ROUNDS_4_8>(c);
}

fn gmimc_2<F, const WIDTH: usize, const D: usize, const NUM_ROUNDS: usize>(c: &mut Criterion)
where
    F: PrimeField,
    StandardUniform: Distribution<F> 
{
    let mut rng = SmallRng::seed_from_u64(1);

    let gmimc_2 = GMiMC2::<F, WIDTH, D>::new_from_rng(NUM_ROUNDS, &mut rng);

    let input = [F::ZERO; WIDTH];
    let name = format!("GMiMC2::<{}>::<{}>", D, pretty_name::<F>());
    let id = BenchmarkId::new(name, WIDTH);
    c.bench_with_input(id, &input, |b, input| {
        b.iter(|| gmimc_2.permute(input.clone()))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default()
            .sample_size(1000)
            .warm_up_time(std::time::Duration::from_secs(5));
    targets = bench_gmimc_2
);
criterion_main!(benches);
