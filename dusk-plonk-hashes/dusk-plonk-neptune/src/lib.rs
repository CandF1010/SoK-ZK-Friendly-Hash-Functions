#![no_std]

extern crate alloc;

mod error;
pub use error::Error;

mod neptune;
pub use neptune::WIDTH as NEPTUNE_WIDTH;

mod hash;
#[cfg(feature = "zk")]
pub use hash::gadget::HashGadget;
pub use hash::{Domain, Hash};

#[cfg(feature = "encryption")]
mod encryption;

#[cfg(feature = "encryption")]
#[cfg(feature = "zk")]
pub use encryption::gadget::{decrypt_gadget, encrypt_gadget};
#[cfg(feature = "encryption")]
pub use encryption::{decrypt, encrypt};
