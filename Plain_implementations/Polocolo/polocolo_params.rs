use crate::fields::{FieldElement, PrimeField, PrimeFieldExt};
use crate::utils::pow_biguint;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::Shake128;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PolocoloParams<F: PrimeField> {
    pub(crate) t: usize,
    pub(crate) m: usize,
    pub(crate) ann: BigUint,
    pub(crate) rounds: usize,
    pub(crate) mds: Vec<Vec<F>>,
    pub(crate) rc: Vec<Vec<F>>,
    pub(crate) lut: HashMap<BigUint, F>,
    pub(crate) modulus: BigUint,
    pub(crate) modulus_minus_two: BigUint,
}

impl<F: PrimeField + PrimeFieldExt> PolocoloParams<F> {
    pub fn from_table(t: usize, label: &str) -> Self {
        let (rounds, m) = match t {
            3 => (6usize, 1024usize),
            4 => (5usize, 512usize),
            5 => (5usize, 128usize),
            6 => (5usize, 64usize),
            7 => (5usize, 32usize),
            8 => (5usize, 32usize),
            _ => panic!("unsupported Polocolo width"),
        };
        Self::from_spec(t, rounds, m, label)
    }

    pub fn from_spec(t: usize, rounds: usize, m: usize, label: &str) -> Self {
        assert!(t >= 3 && t <= 8);
        let modulus = F::modulus();
        let modulus_minus_one = &modulus - BigUint::one();
        let m_big = BigUint::from(m as u64);
        if &modulus_minus_one % &m_big != BigUint::zero() {
            panic!("m does not divide p-1 for Polocolo");
        }

        let ann = &modulus_minus_one / &m_big;
        let modulus_minus_two = &modulus - BigUint::from(2u32);

        let g = F::from_biguint(&F::generator());
        let sigma = generate_sigma::<F>(m, &modulus_minus_two, &g, &ann);
        let lut = build_lut::<F>(m, &sigma, &g, &ann);
        let mds = mds_matrix::<F>(t);
        let rc = generate_round_constants::<F>(t, rounds, m, label, &modulus);

        PolocoloParams {
            t,
            m,
            ann,
            rounds,
            mds,
            rc,
            lut,
            modulus,
            modulus_minus_two,
        }
    }
}

fn generate_round_constants<F: PrimeField>(
    t: usize,
    rounds: usize,
    m: usize,
    label: &str,
    modulus: &BigUint,
) -> Vec<Vec<F>> {
    let seed = format!("Polocolo-{}-{}-{}-{}", m, rounds, t, label);
    let mut tape = HashTape::new(seed.as_bytes());

    let mut rc = Vec::with_capacity(rounds);
    for _ in 0..rounds {
        let mut row = Vec::with_capacity(t);
        for _ in 0..t {
            let candidate = tape.rand_biguint(modulus);
            row.push(F::from_biguint(&candidate));
        }
        rc.push(row);
    }
    rc
}

fn generate_sigma<F: PrimeField + PrimeFieldExt>(
    m: usize,
    modulus_minus_two: &BigUint,
    g: &F,
    ann: &BigUint,
) -> Vec<usize> {
    let seed = format!("Polocolo-{}", m);
    let mut tape = HashTape::new(seed.as_bytes());

    let gk = pow_biguint(g, ann);
    loop {
        let sigma = random_permutation(m, &mut tape);
        if sigma_valid::<F>(&sigma, m, g, &gk, modulus_minus_two) {
            return sigma;
        }
    }
}

fn random_permutation(m: usize, tape: &mut HashTape) -> Vec<usize> {
    let mut sigma = Vec::with_capacity(m);
    while sigma.len() < m {
        let num = tape.randint_u64(m as u64) as usize;
        if sigma.contains(&num) {
            continue;
        }
        sigma.push(num);
    }
    sigma
}

fn sigma_valid<F: FieldElement + PrimeFieldExt>(
    sigma: &[usize],
    m: usize,
    g: &F,
    gk: &F,
    modulus_minus_two: &BigUint,
) -> bool {
    let mut xs_f = Vec::with_capacity(m);
    let mut ys_f = Vec::with_capacity(m);
    let mut xs_h = Vec::with_capacity(m);
    let mut ys_h = Vec::with_capacity(m);

    let m_u64 = m as u64;
    for r in 0..m {
        let r_u64 = r as u64;
        xs_f.push(g.pow_u64(r_u64));
        let exp_f = r_u64 * m_u64 + sigma[r] as u64;
        ys_f.push(g.pow_u64(exp_f));

        xs_h.push(gk.pow_u64(r_u64));
        let exp_h = (m_u64 + 1) * r_u64 + sigma[r] as u64;
        ys_h.push(g.pow_u64(exp_h));
    }

    let coeffs_f = interpolate_poly(&xs_f, &ys_f, modulus_minus_two);
    if coeffs_f.iter().any(|c| *c == F::zero()) {
        return false;
    }

    let coeffs_h = interpolate_poly(&xs_h, &ys_h, modulus_minus_two);
    if coeffs_h.iter().any(|c| *c == F::zero()) {
        return false;
    }

    true
}

fn interpolate_poly<F: FieldElement>(
    xs: &[F],
    ys: &[F],
    modulus_minus_two: &BigUint,
) -> Vec<F> {
    let m = xs.len();
    assert_eq!(ys.len(), m);

    let mut poly = vec![F::one()];
    for x in xs.iter() {
        poly = poly_mul_monic_linear(&poly, x);
    }

    let mut coeffs = vec![F::zero(); m];
    for i in 0..m {
        let mut denom = F::one();
        for j in 0..m {
            if i == j {
                continue;
            }
            let mut diff = xs[i].clone();
            diff.sub_assign(&xs[j]);
            denom.mul_assign(&diff);
        }
        let denom_inv = pow_biguint(&denom, modulus_minus_two);
        let q = poly_div_monic_linear(&poly, &xs[i]);

        let mut scale = ys[i].clone();
        scale.mul_assign(&denom_inv);
        for k in 0..m {
            let mut term = q[k].clone();
            term.mul_assign(&scale);
            coeffs[k].add_assign(&term);
        }
    }

    coeffs
}

fn poly_mul_monic_linear<F: FieldElement>(poly: &[F], root: &F) -> Vec<F> {
    let mut out = vec![F::zero(); poly.len() + 1];
    for (i, coeff) in poly.iter().enumerate() {
        let mut neg = root.clone();
        neg.mul_assign(coeff);
        out[i].sub_assign(&neg);
        out[i + 1].add_assign(coeff);
    }
    out
}

fn poly_div_monic_linear<F: FieldElement>(poly: &[F], root: &F) -> Vec<F> {
    let m = poly.len() - 1;
    let mut q = vec![F::zero(); m];
    q[m - 1] = poly[m].clone();
    for k in (1..m).rev() {
        let mut term = root.clone();
        term.mul_assign(&q[k]);
        let mut val = poly[k].clone();
        val.add_assign(&term);
        q[k - 1] = val;
    }
    q
}

fn build_lut<F: PrimeField + PrimeFieldExt>(
    m: usize,
    sigma: &[usize],
    g: &F,
    ann: &BigUint,
) -> HashMap<BigUint, F> {
    let mut lut = HashMap::with_capacity(m + 1);
    lut.insert(BigUint::zero(), F::zero());

    let gk = pow_biguint(g, ann);
    let m_u64 = m as u64;
    for r in 0..m {
        let r_u64 = r as u64;
        let lut_in = gk.pow_u64(r_u64);
        let exp = (m_u64 + 1) * r_u64 + sigma[r] as u64;
        let lut_out = g.pow_u64(exp);
        lut.insert(lut_in.to_biguint(), lut_out);
    }

    lut
}

fn mds_matrix<F: PrimeField>(t: usize) -> Vec<Vec<F>> {
    match t {
        3 => vec![
            vec![F::from_u64(2), F::from_u64(1), F::from_u64(1)],
            vec![F::from_u64(1), F::from_u64(2), F::from_u64(1)],
            vec![F::from_u64(1), F::from_u64(1), F::from_u64(2)],
        ],
        4 => vec![
            vec![F::from_u64(5), F::from_u64(7), F::from_u64(1), F::from_u64(3)],
            vec![F::from_u64(4), F::from_u64(6), F::from_u64(1), F::from_u64(1)],
            vec![F::from_u64(1), F::from_u64(3), F::from_u64(5), F::from_u64(7)],
            vec![F::from_u64(1), F::from_u64(1), F::from_u64(4), F::from_u64(6)],
        ],
        5 => vec![
            vec![
                F::from_u64(39),
                F::from_u64(6),
                F::from_u64(10),
                F::from_u64(28),
                F::from_u64(8),
            ],
            vec![
                F::from_u64(174),
                F::from_u64(28),
                F::from_u64(32),
                F::from_u64(80),
                F::from_u64(16),
            ],
            vec![
                F::from_u64(348),
                F::from_u64(58),
                F::from_u64(42),
                F::from_u64(84),
                F::from_u64(2),
            ],
            vec![
                F::from_u64(39),
                F::from_u64(4),
                F::from_u64(54),
                F::from_u64(100),
                F::from_u64(44),
            ],
            vec![
                F::from_u64(204),
                F::from_u64(20),
                F::from_u64(300),
                F::from_u64(560),
                F::from_u64(244),
            ],
        ],
        6 => vec![
            vec![
                F::from_u64(1011),
                F::from_u64(1470),
                F::from_u64(42),
                F::from_u64(140),
                F::from_u64(508),
                F::from_u64(1700),
            ],
            vec![
                F::from_u64(232),
                F::from_u64(70),
                F::from_u64(48),
                F::from_u64(48),
                F::from_u64(264),
                F::from_u64(1280),
            ],
            vec![
                F::from_u64(4227),
                F::from_u64(7371),
                F::from_u64(3),
                F::from_u64(490),
                F::from_u64(1420),
                F::from_u64(2900),
            ],
            vec![
                F::from_u64(6744),
                F::from_u64(11760),
                F::from_u64(60),
                F::from_u64(844),
                F::from_u64(2272),
                F::from_u64(4670),
            ],
            vec![
                F::from_u64(13281),
                F::from_u64(23163),
                F::from_u64(9),
                F::from_u64(1540),
                F::from_u64(4460),
                F::from_u64(9100),
            ],
            vec![
                F::from_u64(48),
                F::from_u64(84),
                F::from_u64(12),
                F::from_u64(35),
                F::from_u64(40),
                F::from_u64(200),
            ],
        ],
        7 => vec![
            vec![
                F::from_u64(3538),
                F::from_u64(3090),
                F::from_u64(768),
                F::from_u64(480),
                F::from_u64(720),
                F::from_u64(96),
                F::from_u64(336),
            ],
            vec![
                F::from_u64(470862),
                F::from_u64(470750),
                F::from_u64(1120),
                F::from_u64(16380),
                F::from_u64(94284),
                F::from_u64(136),
                F::from_u64(924),
            ],
            vec![
                F::from_u64(10112885),
                F::from_u64(10113269),
                F::from_u64(24960),
                F::from_u64(352496),
                F::from_u64(2023200),
                F::from_u64(768),
                F::from_u64(18048),
            ],
            vec![
                F::from_u64(3799380),
                F::from_u64(3799524),
                F::from_u64(9024),
                F::from_u64(132256),
                F::from_u64(760128),
                F::from_u64(288),
                F::from_u64(6783),
            ],
            vec![
                F::from_u64(94120),
                F::from_u64(94080),
                F::from_u64(232),
                F::from_u64(3276),
                F::from_u64(18816),
                F::from_u64(5),
                F::from_u64(198),
            ],
            vec![
                F::from_u64(1357780),
                F::from_u64(1357788),
                F::from_u64(3240),
                F::from_u64(47268),
                F::from_u64(271632),
                F::from_u64(101),
                F::from_u64(2454),
            ],
            vec![
                F::from_u64(270260),
                F::from_u64(270260),
                F::from_u64(640),
                F::from_u64(9402),
                F::from_u64(54108),
                F::from_u64(64),
                F::from_u64(480),
            ],
        ],
        8 => vec![
            vec![
                F::from_u64(3840),
                F::from_u64(24),
                F::from_u64(4728),
                F::from_u64(2952),
                F::from_u64(258912),
                F::from_u64(99840),
                F::from_u64(94222),
                F::from_u64(74400),
            ],
            vec![
                F::from_u64(1386),
                F::from_u64(78),
                F::from_u64(280),
                F::from_u64(1218),
                F::from_u64(32256),
                F::from_u64(13044),
                F::from_u64(8120),
                F::from_u64(6496),
            ],
            vec![
                F::from_u64(6180),
                F::from_u64(743),
                F::from_u64(10416),
                F::from_u64(4428),
                F::from_u64(508032),
                F::from_u64(194858),
                F::from_u64(193984),
                F::from_u64(153056),
            ],
            vec![
                F::from_u64(432),
                F::from_u64(400),
                F::from_u64(1920),
                F::from_u64(144),
                F::from_u64(73728),
                F::from_u64(27776),
                F::from_u64(30400),
                F::from_u64(23936),
            ],
            vec![
                F::from_u64(10122),
                F::from_u64(1246),
                F::from_u64(5320),
                F::from_u64(8526),
                F::from_u64(346752),
                F::from_u64(136724),
                F::from_u64(108570),
                F::from_u64(86184),
            ],
            vec![
                F::from_u64(950),
                F::from_u64(1052),
                F::from_u64(5424),
                F::from_u64(240),
                F::from_u64(202944),
                F::from_u64(76333),
                F::from_u64(84683),
                F::from_u64(66656),
            ],
            vec![
                F::from_u64(2564),
                F::from_u64(16),
                F::from_u64(3072),
                F::from_u64(1920),
                F::from_u64(172128),
                F::from_u64(66380),
                F::from_u64(62528),
                F::from_u64(49408),
            ],
            vec![
                F::from_u64(661),
                F::from_u64(35),
                F::from_u64(908),
                F::from_u64(585),
                F::from_u64(43008),
                F::from_u64(16448),
                F::from_u64(14512),
                F::from_u64(11456),
            ],
        ],
        _ => panic!("unsupported Polocolo width"),
    }
}

struct HashTape {
    reader: Box<dyn XofReader>,
}

impl HashTape {
    fn new(label: &[u8]) -> Self {
        let mut shake = Shake128::default();
        shake.update(label);
        let reader = shake.finalize_xof();
        HashTape {
            reader: Box::new(reader),
        }
    }

    fn randint_u64(&mut self, modulus: u64) -> u64 {
        if modulus <= 1 {
            return 0;
        }
        let bits = 64 - modulus.leading_zeros() as usize;
        let bytes = (bits + 7) / 8;
        loop {
            let mut buf = vec![0u8; bytes];
            self.reader.read(&mut buf);
            if bits % 8 != 0 {
                let mask = (1u8 << (bits % 8)) - 1;
                buf[0] &= mask;
            }
            let mut value = 0u64;
            for b in buf.iter() {
                value = (value << 8) | (*b as u64);
            }
            if value < modulus {
                return value;
            }
        }
    }

    fn rand_biguint(&mut self, modulus: &BigUint) -> BigUint {
        let bits = modulus.bits() as usize;
        let bytes = (bits + 7) / 8;
        loop {
            let mut buf = vec![0u8; bytes];
            self.reader.read(&mut buf);
            if bits % 8 != 0 {
                let mask = (1u8 << (bits % 8)) - 1;
                buf[0] &= mask;
            }
            let value = BigUint::from_bytes_be(&buf);
            if value < *modulus {
                return value;
            }
        }
    }
}
