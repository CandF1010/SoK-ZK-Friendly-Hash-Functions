use crate::fields::{FieldElement, PrimeField};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Poseidon2Params<F: FieldElement> {
    pub(crate) t: usize,
    pub(crate) d: u64,
    pub(crate) rounds_f_beginning: usize,
    pub(crate) rounds_p: usize,
    #[allow(dead_code)]
    pub(crate) rounds_f_end: usize,
    pub(crate) rounds: usize,
    pub(crate) mat_internal_diag_m_1: Vec<F>,
    pub(crate) mat_internal: Vec<Vec<F>>,
    pub(crate) round_constants: Vec<Vec<F>>,
}

impl<F: FieldElement> Poseidon2Params<F> {
    pub fn new(
        t: usize,
        d: u64,
        rounds_f: usize,
        rounds_p: usize,
        mat_internal_diag_m_1: &[F],
        mat_internal: &[Vec<F>],
        round_constants: &[Vec<F>],
    ) -> Self {
        assert!(d == 3 || d == 5 || d == 7 || d == 11);
        assert_eq!(rounds_f % 2, 0);
        let r = rounds_f / 2;
        let rounds = rounds_f + rounds_p;

        Poseidon2Params {
            t,
            d,
            rounds_f_beginning: r,
            rounds_p,
            rounds_f_end: r,
            rounds,
            mat_internal_diag_m_1: mat_internal_diag_m_1.to_owned(),
            mat_internal: mat_internal.to_owned(),
            round_constants: round_constants.to_owned(),
        }
    }
}

impl<F: PrimeField> Poseidon2Params<F> {
    pub fn from_grain(
        t: usize,
        d: u64,
        rounds_f: usize,
        rounds_p: usize,
        mat_internal_diag_m_1: &[F],
        mat_internal: &[Vec<F>],
    ) -> Self {
        let modulus = F::modulus();
        let n_bits = modulus.bits() as usize;
        let seed = grain_seed_bits(n_bits as u64, t as u64, rounds_f as u64, rounds_p as u64);
        let mut lfsr = GrainLfsr::new(seed);
        lfsr.warmup(160);

        let round_constants =
            generate_round_constants::<F>(&mut lfsr, &modulus, n_bits, t, rounds_f, rounds_p);

        Poseidon2Params::new(
            t,
            d,
            rounds_f,
            rounds_p,
            mat_internal_diag_m_1,
            mat_internal,
            &round_constants,
        )
    }
}

struct GrainLfsr {
    state: VecDeque<u8>,
}

impl GrainLfsr {
    fn new(seed_bits: [u8; 80]) -> Self {
        Self {
            state: seed_bits.into_iter().collect(),
        }
    }

    fn warmup(&mut self, steps: usize) {
        for _ in 0..steps {
            self.next_bit();
        }
    }

    fn next_bit(&mut self) -> u8 {
        let new_bit = self.state[62]
            ^ self.state[51]
            ^ self.state[38]
            ^ self.state[23]
            ^ self.state[13]
            ^ self.state[0];
        self.state.pop_front();
        self.state.push_back(new_bit);
        new_bit
    }

    fn next_shrunk_bit(&mut self) -> u8 {
        loop {
            let first = self.next_bit();
            let second = self.next_bit();
            if first == 1 {
                return second;
            }
        }
    }

    fn random_bits(&mut self, n_bits: usize) -> BigUint {
        let mut out = BigUint::zero();
        for _ in 0..n_bits {
            out <<= 1u8;
            if self.next_shrunk_bit() == 1 {
                out += BigUint::one();
            }
        }
        out
    }
}

fn grain_seed_bits(n_bits: u64, t: u64, rounds_f: u64, rounds_p: u64) -> [u8; 80] {
    let mut bits = Vec::with_capacity(80);
    // FIELD=1 (GF(p)) -> "01", SBOX=1 (Poseidon2) -> "0001"
    bits.extend_from_slice(&[0, 1]);
    bits.extend_from_slice(&[0, 0, 0, 1]);
    push_bits(&mut bits, n_bits, 12);
    push_bits(&mut bits, t, 12);
    push_bits(&mut bits, rounds_f, 10);
    push_bits(&mut bits, rounds_p, 10);
    bits.extend(std::iter::repeat(1u8).take(30));
    bits.try_into().expect("poseidon2 grain seed is 80 bits")
}

fn push_bits(bits: &mut Vec<u8>, value: u64, width: usize) {
    for i in (0..width).rev() {
        bits.push(((value >> i) & 1) as u8);
    }
}

fn generate_round_constants<F: PrimeField>(
    lfsr: &mut GrainLfsr,
    modulus: &BigUint,
    n_bits: usize,
    t: usize,
    rounds_f: usize,
    rounds_p: usize,
) -> Vec<Vec<F>> {
    let rounds = rounds_f + rounds_p;
    let rf_half = rounds_f / 2;
    let mut round_constants = Vec::with_capacity(rounds);

    for r in 0..rounds {
        let mut row = Vec::with_capacity(t);
        for c in 0..t {
            if (rf_half..(rf_half + rounds_p)).contains(&r) && c > 0 {
                row.push(F::zero());
                continue;
            }
            let mut candidate = lfsr.random_bits(n_bits);
            while candidate >= *modulus {
                candidate = lfsr.random_bits(n_bits);
            }
            row.push(F::from_biguint(&candidate));
        }
        round_constants.push(row);
    }

    round_constants
}
