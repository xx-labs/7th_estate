//! # Cryptographically Secure Pseudorandom Number Generation Functionality
//!
//! `csprng` contains functions related to generating cryptographically
//! secure pseudorandom numbers and bits.
//!
//! The specific CSPRNG employed is ChaCha20. ChaCha20 accepts a 256-bit
//! random seed, has a 512-bit state, and has a period of 70 bits.

pub use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;

pub type CSPRNG = ChaChaRng;

#[derive(Debug, Clone, Copy)]
pub struct CSPRNGSeed([u8; 32]);

impl CSPRNGSeed {
    pub const DEFAULT: CSPRNGSeed = CSPRNGSeed([0; 32]);
    pub const SIZE: usize = 32;

    pub fn from_vec(value: &Vec<u8>) -> Self {
        let mut seed = Self::DEFAULT;
        seed.0.copy_from_slice(&value[..Self::SIZE]);
        seed
    }

    pub fn next_seed(rng: &mut dyn RngCore) -> Self {
        let mut seed = Self::DEFAULT;
        rng.fill_bytes(&mut seed.0);
        seed
    }
}

pub trait CSPRNGExt<T: SeedableRng> {
    fn from_csprng_seed(value: CSPRNGSeed) -> T;
}

impl CSPRNGExt<ChaChaRng> for ChaChaRng {
    fn from_csprng_seed(value: CSPRNGSeed) -> Self {
        Self::from_seed(value.0)
    }
}

