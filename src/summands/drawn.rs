//! # Drawn Summands Module
//!
//! The drawn summands are generated from a deterministic random number
//! generator.

use std::convert::From;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawnSummands {
    records: SummandRecords
}

impl DrawnSummands {
    pub fn from_csprng(seed: CSPRNGSeed, count: usize, modulus: usize) -> Self {
        drawn_summands_from_csprng_fdr(seed, count, modulus)
    }

    pub fn len(self: &Self) -> usize { self.records.len() }
}

impl From<Summands> for DrawnSummands {
    fn from(summands: Summands) -> DrawnSummands {
        DrawnSummands { records: summands.records.clone() }
    }
}

impl From<DrawnSummands> for Summands {
    fn from(summands: DrawnSummands) -> Summands {
        Summands { records: summands.records.clone() }
    }
}

/// Generate a list of summands using a CSPRNG feeding the Fast Dice Roller.
fn drawn_summands_from_csprng_fdr(seed: CSPRNGSeed, count: usize, modulus: usize) -> DrawnSummands {
    let mut prng = CSPRNG::from_csprng_seed(seed);
    DrawnSummands {
        records: (0..count).into_iter()
            .map(|n| {
                SummandRecord { position: n, summand: prng.gen_range(0, modulus) }
            }).collect()
    }
}
