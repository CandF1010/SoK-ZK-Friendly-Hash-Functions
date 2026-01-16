use super::{FieldElement, PrimeField};
use super::montgomery_4::{
    add_mod, from_hex_to_limbs, monty_mul, reduce_raw, sub_mod, to_monty, MontyParams,
};
use num_bigint::BigUint;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Bls12_381 {
    pub(crate) value: [u64; 4],
}

struct Bls12_381Params;

impl MontyParams for Bls12_381Params {
    const MODULUS: [u64; 4] = [
        0xffffffff00000001,
        0x53bda402fffe5bfe,
        0x3339d80809a1d805,
        0x73eda753299d7d48,
    ];
    const MU: u64 = 0x0000000100000001;
    const R2: [u64; 4] = [
        0xc999e990f3f29c6d,
        0x2b6cedcb87925c23,
        0x05d314967254398f,
        0x0748d9d99f59ff11,
    ];
}

impl Bls12_381 {
    pub fn from_hex(s: &str) -> Option<Self> {
        let raw = from_hex_to_limbs(s)?;
        Some(Self::from_raw(raw))
    }

    #[inline]
    fn from_raw(raw: [u64; 4]) -> Self {
        Self {
            value: to_monty::<Bls12_381Params>(reduce_raw::<Bls12_381Params>(raw)),
        }
    }
}

impl FieldElement for Bls12_381 {
    fn zero() -> Self {
        Self { value: [0; 4] }
    }

    fn one() -> Self {
        Self::from_u64(1)
    }

    fn from_u64(val: u64) -> Self {
        Self::from_raw([val, 0, 0, 0])
    }

    fn add_assign(&mut self, other: &Self) {
        self.value = add_mod::<Bls12_381Params>(self.value, other.value);
    }

    fn sub_assign(&mut self, other: &Self) {
        self.value = sub_mod::<Bls12_381Params>(self.value, other.value);
    }

    fn mul_assign(&mut self, other: &Self) {
        self.value = monty_mul::<Bls12_381Params>(self.value, other.value);
    }
}

impl PrimeField for Bls12_381 {
    fn modulus() -> BigUint {
        BigUint::parse_bytes(
            b"73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001",
            16,
        )
        .expect("valid BLS12-381 modulus")
    }

    fn from_biguint(value: &BigUint) -> Self {
        let modulus = Self::modulus();
        let reduced = value % &modulus;
        let hex = reduced.to_str_radix(16);
        let hex = if hex.is_empty() { "0".to_string() } else { hex };
        let prefixed = format!("0x{hex}");
        Self::from_hex(&prefixed).expect("valid BLS12-381 element")
    }

    fn generator() -> BigUint {
        BigUint::from(7u32)
    }
}
