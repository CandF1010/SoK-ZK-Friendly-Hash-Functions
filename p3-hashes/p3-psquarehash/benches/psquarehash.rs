use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use p3_baby_bear::BabyBear;
use p3_field::PrimeField;
use p3_koala_bear::KoalaBear;
use p3_mersenne_31::Mersenne31;
use p3_psquarehash::PSquareHash;
use p3_symmetric::Permutation;
use p3_util::pretty_name;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::distr::{Distribution, StandardUniform};

const NUM_ROUNDS: usize = 4;
const NUM_STEPS: usize = 13;

fn bench_psquarehash(c: &mut Criterion) {
    psquarehash::<BabyBear, 16, NUM_ROUNDS, NUM_STEPS>(c);
    psquarehash::<BabyBear, 24, NUM_ROUNDS, NUM_STEPS>(c);

    psquarehash::<KoalaBear, 16, NUM_ROUNDS, NUM_STEPS>(c);
    psquarehash::<KoalaBear, 24, NUM_ROUNDS, NUM_STEPS>(c);

    psquarehash::<Mersenne31, 16, NUM_ROUNDS, NUM_STEPS>(c);
    psquarehash::<Mersenne31, 24, NUM_ROUNDS, NUM_STEPS>(c);
}

fn psquarehash<F, const WIDTH: usize, const NUM_ROUNDS: usize, const NUM_STEPS: usize>(c: &mut Criterion)
where
    F: PrimeField,
    StandardUniform: Distribution<F> 
{
    let mut rng = SmallRng::seed_from_u64(1);

    let psquarehash = PSquareHash::<F, WIDTH>::new_from_rng(NUM_ROUNDS, NUM_STEPS, &mut rng);

    let input = [F::ZERO; WIDTH];
    let name = format!("pSquare-hash::<{}, {}>::<{}>", NUM_ROUNDS, NUM_STEPS, pretty_name::<F>());
    let id = BenchmarkId::new(name, WIDTH);
    c.bench_with_input(id, &input, |b, input| {
        b.iter(|| psquarehash.permute(input.clone()))
    });
}


criterion_group!(
    name = benches;
    config = Criterion::default()
            .sample_size(1000)
            .warm_up_time(std::time::Duration::from_secs(5));
    targets = bench_psquarehash
);
criterion_main!(benches);
