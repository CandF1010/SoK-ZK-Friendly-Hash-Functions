pub mod instances;
pub mod psquarehash;

pub use instances::{PSQUAREHASH_MERSENNE31_16_PARAMS, PSQUAREHASH_MERSENNE31_24_PARAMS};
pub use psquarehash::{PSquareHash, PSquareHashParams, PSQUAREHASH_ROUNDS};
