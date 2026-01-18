use super::{biguint_from_limbs_le, FieldElement, PrimeField, PrimeFieldExt, PrimeFieldWords};
use super::montgomery_4::{
    add_mod, from_hex_to_limbs, monty_mul, reduce_raw, sub_mod, to_monty, MontyParams,
};
use num_bigint::BigUint;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Bn254 {
    pub(crate) value: [u64; 4],
}

struct Bn254Params;

impl MontyParams for Bn254Params {
    const MODULUS: [u64; 4] = [
        0x43e1f593f0000001,
        0x2833e84879b97091,
        0xb85045b68181585d,
        0x30644e72e131a029,
    ];
    const MU: u64 = 0x3d1e0a6c10000001;
    const R2: [u64; 4] = [
        0x1bb8e645ae216da7,
        0x53fe3ab1e35c59e3,
        0x8c49833d53bb8085,
        0x0216d0b17f4e44a5,
    ];
}

impl Bn254 {
    pub fn from_hex(s: &str) -> Option<Self> {
        let raw = from_hex_to_limbs(s)?;
        Some(Self::from_raw(raw))
    }

    #[inline]
    fn from_raw(raw: [u64; 4]) -> Self {
        Self {
            value: to_monty::<Bn254Params>(reduce_raw::<Bn254Params>(raw)),
        }
    }
}

impl FieldElement for Bn254 {
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
        self.value = add_mod::<Bn254Params>(self.value, other.value);
    }

    fn sub_assign(&mut self, other: &Self) {
        self.value = sub_mod::<Bn254Params>(self.value, other.value);
    }

    fn mul_assign(&mut self, other: &Self) {
        self.value = monty_mul::<Bn254Params>(self.value, other.value);
    }
}

impl PrimeField for Bn254 {
    fn modulus() -> BigUint {
        BigUint::parse_bytes(
            b"30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001",
            16,
        )
        .expect("valid BN254 modulus")
    }

    fn from_biguint(value: &BigUint) -> Self {
        let modulus = Self::modulus();
        let reduced = value % &modulus;
        let hex = reduced.to_str_radix(16);
        let hex = if hex.is_empty() { "0".to_string() } else { hex };
        let prefixed = format!("0x{hex}");
        Self::from_hex(&prefixed).expect("valid BN254 element")
    }

    fn generator() -> BigUint {
        BigUint::from(5u32)
    }
}

impl PrimeFieldExt for Bn254 {
    fn to_biguint(&self) -> BigUint {
        let normal = monty_mul::<Bn254Params>(self.value, [1, 0, 0, 0]);
        biguint_from_limbs_le(&normal)
    }
}

impl PrimeFieldWords for Bn254 {
    fn to_words_le(&self) -> [u64; 4] {
        monty_mul::<Bn254Params>(self.value, [1, 0, 0, 0])
    }
}
