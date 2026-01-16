use crate::fields::{FieldElement, PrimeField};
use crate::utils::modinv;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct PoseidonParams<F: FieldElement> {
    pub(crate) t: usize,
    pub(crate) d: u64,
    pub(crate) rounds_f_beginning: usize,
    pub(crate) rounds_p: usize,
    #[allow(dead_code)]
    pub(crate) rounds_f_end: usize,
    pub(crate) rounds: usize,
    pub(crate) mds: Vec<Vec<F>>,
    pub(crate) round_constants: Vec<Vec<F>>,
}

impl<F: FieldElement> PoseidonParams<F> {
    pub fn new(
        t: usize,
        d: u64,
        rounds_f: usize,
        rounds_p: usize,
        mds: &[Vec<F>],
        round_constants: &[Vec<F>],
    ) -> Self {
        assert!(d == 3 || d == 5 || d == 7);
        assert_eq!(rounds_f % 2, 0);
        assert_eq!(mds.len(), t);
        for row in mds {
            assert_eq!(row.len(), t);
        }

        let rounds = rounds_f + rounds_p;
        assert_eq!(round_constants.len(), rounds);
        for rc in round_constants {
            assert_eq!(rc.len(), t);
        }

        let r = rounds_f / 2;
        PoseidonParams {
            t,
            d,
            rounds_f_beginning: r,
            rounds_p,
            rounds_f_end: r,
            rounds,
            mds: mds.to_owned(),
            round_constants: round_constants.to_owned(),
        }
    }
}

impl<F: PrimeField> PoseidonParams<F> {
    pub fn from_grain(t: usize, d: u64, rounds_f: usize, rounds_p: usize) -> Self {
        let modulus = F::modulus();
        let n_bits = modulus.bits() as usize;
        let seed = grain_seed_bits(n_bits as u64, t as u64, rounds_f as u64, rounds_p as u64);
        let mut lfsr = GrainLfsr::new(seed);
        lfsr.warmup(160);

        let round_constants =
            generate_round_constants::<F>(&mut lfsr, &modulus, n_bits, t, rounds_f, rounds_p);
        let mds = generate_mds::<F>(&mut lfsr, &modulus, n_bits, t);

        PoseidonParams::new(t, d, rounds_f, rounds_p, &mds, &round_constants)
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
    // FIELD=1 (GF(p)) -> "01", SBOX=0 (x^alpha) -> "0000"
    bits.extend_from_slice(&[0, 1]);
    bits.extend_from_slice(&[0, 0, 0, 0]);
    push_bits(&mut bits, n_bits, 12);
    push_bits(&mut bits, t, 12);
    push_bits(&mut bits, rounds_f, 10);
    push_bits(&mut bits, rounds_p, 10);
    bits.extend(std::iter::repeat(1u8).take(30));
    bits.try_into().expect("poseidon grain seed is 80 bits")
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
    let num_constants = (rounds_f + rounds_p) * t;
    let mut flat = Vec::with_capacity(num_constants);
    for _ in 0..num_constants {
        let mut candidate = lfsr.random_bits(n_bits);
        while candidate >= *modulus {
            candidate = lfsr.random_bits(n_bits);
        }
        flat.push(F::from_biguint(&candidate));
    }

    flat.chunks(t).map(|chunk| chunk.to_vec()).collect()
}

fn generate_mds<F: PrimeField>(
    lfsr: &mut GrainLfsr,
    modulus: &BigUint,
    n_bits: usize,
    t: usize,
) -> Vec<Vec<F>> {
    loop {
        let mut rand_list = Vec::with_capacity(2 * t);
        for _ in 0..(2 * t) {
            let value = lfsr.random_bits(n_bits) % modulus;
            rand_list.push(value);
        }
        if has_duplicates(&rand_list) {
            continue;
        }

        let xs = &rand_list[..t];
        let ys = &rand_list[t..];
        let mut mds = vec![vec![F::zero(); t]; t];
        let mut ok = true;
        for i in 0..t {
            for j in 0..t {
                let denom = (&xs[i] + &ys[j]) % modulus;
                if denom.is_zero() {
                    ok = false;
                    break;
                }
                let inv = modinv(&denom, modulus);
                mds[i][j] = F::from_biguint(&inv);
            }
            if !ok {
                break;
            }
        }
        if ok {
            return mds;
        }
    }
}

fn has_duplicates(values: &[BigUint]) -> bool {
    for i in 0..values.len() {
        for j in (i + 1)..values.len() {
            if values[i] == values[j] {
                return true;
            }
        }
    }
    false
}
