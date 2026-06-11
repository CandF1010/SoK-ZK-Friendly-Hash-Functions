use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use p3_baby_bear::BabyBear;
use p3_field::PrimeField;
use p3_koala_bear::KoalaBear;
use p3_mersenne_31::Mersenne31;
use p3_goldilocks::Goldilocks;
use p3_bn254::Bn254;
use p3_gmimc::GMiMC;
use p3_symmetric::Permutation;
use p3_util::pretty_name;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::distr::{Distribution, StandardUniform};

// 32 bit fields
const ROUNDS_16: usize = 158;
const ROUNDS_24: usize = 335;
// 64 bit fields
const ROUNDS_8: usize = 68;
const ROUNDS_12: usize = 93;
// 256 bit fields
const ROUNDS_4: usize = 334;

fn bench_gmimc(c: &mut Criterion) {
    // BabyBear
    gmimc::<BabyBear, 16, 7, ROUNDS_16>(c);
    gmimc::<BabyBear, 24, 7, ROUNDS_24>(c);

    // KoalaBear
    gmimc::<KoalaBear, 16, 3,ROUNDS_16>(c);
    gmimc::<KoalaBear, 24, 3,ROUNDS_24>(c);

    // Mersenne31
    gmimc::<Mersenne31, 16, 5,ROUNDS_16>(c);
    gmimc::<Mersenne31, 24, 5, ROUNDS_24>(c);

    // Goldilocks
    gmimc::<Goldilocks, 8, 7,ROUNDS_8>(c);
    gmimc::<Goldilocks, 12, 7,ROUNDS_12>(c);

    // BN254
    gmimc::<Bn254, 4, 5,ROUNDS_4>(c);
}

fn gmimc<F, const WIDTH: usize, const D: usize, const NUM_ROUNDS: usize>(c: &mut Criterion)
where
    F: PrimeField,
    StandardUniform: Distribution<F> 
{
    let mut rng = SmallRng::seed_from_u64(1);

    let gmimc = GMiMC::<F, WIDTH, D>::new_from_rng(NUM_ROUNDS, &mut rng);

    let input = [F::ZERO; WIDTH];
    let name = format!("GMiMC::<{}>::<{}>", D, pretty_name::<F>());
    let id = BenchmarkId::new(name, WIDTH);
    c.bench_with_input(id, &input, |b, input| {
        b.iter(|| gmimc.permute(input.clone()))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default()
            .sample_size(1000)
            .warm_up_time(std::time::Duration::from_secs(5));
    targets = bench_gmimc
);
criterion_main!(benches);
