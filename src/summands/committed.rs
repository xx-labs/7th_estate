//! # Committed Summands Module
//!
//! The committed summands is a single AEAD encryption.
//! The summands are encrypted and the commitment for the
//! encryption key is protected as associated data in the
//! AEAD scheme. This enables the commitment to remain
//! protected on disk prior to signing of a package.

use std::convert::From;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommittedSummands {
    records: SummandRecords
}

impl CommittedSummands {
    pub fn aead_commit(self: &Self, key: &AEADKey) -> Result<SecuredFile> {
        let csvec = {
            let csvec = Vec::<u8>::new();
            let mut csvwriter = csv::Writer::from_writer(csvec);
            self.records.iter()
                .for_each(|rec| {
                    csvwriter.serialize(rec).unwrap();
                });
            csvwriter.into_inner()?
        };
        Ok(SecuredFile::new(key, "".to_owned(), String::from_utf8(csvec)?))
    }

    pub fn from_csprng(seed: CSPRNGSeed, count: usize, modulus: usize) -> Self {
        committed_summands_from_csprng_fdr(seed, count, modulus)
    }

    pub fn len(self: &Self) -> usize { self.records.len() }
}

impl From<Summands> for CommittedSummands {
    fn from(summands: Summands) -> CommittedSummands {
        CommittedSummands { records: summands.records }
    }
}

impl From<CommittedSummands> for Summands {
    fn from(summands: CommittedSummands) -> Summands {
        Summands { records: summands.records }
    }
}


/// Generate a list of summands using a CSPRNG feeding the Fast Dice Roller.
fn committed_summands_from_csprng_fdr(seed: CSPRNGSeed, count: usize, modulus: usize) -> CommittedSummands {
    /*
    fn try_generate(seed: CSPRNGSeed, count: usize, modulus: usize, num_bytes: usize) -> Option<Vec<usize>> {
        let mut prng = CSPRNG::from_csprng_seed(seed);
        let mut bytes = Vec::<u8>::new();
        bytes.resize_with(num_bytes, || {0});
        prng.fill_bytes(&mut bytes);
        let mut fdr = FastDiceRoller::from_bytes(&bytes);
        let summands: Vec<Option<u128>> = (0..count).into_iter()
            .map(|n| { fdr.random(modulus as u128) })
            .collect();
        match summands.iter().any(|s| s.is_none()) {
            true => None,
            false => Some(summands.iter().map(|s| s.unwrap() as usize).collect())
        }
    }

    let mut num_bytes: usize = 1024;
    let mut committed_summands: CommittedSummands;
    loop {
        let maybe_summands = try_generate(seed, count, modulus, num_bytes);
        if let Some(summands) = maybe_summands {
            committed_summands = CommittedSummands {
                records: summands.iter().enumerate()
                    .map(|(n, &value)| {
                        SummandRecord { position: n, summand: value }
                    }).collect()
            };
            break;
        }
        num_bytes = num_bytes + 1024;
    }
    committed_summands
    */
    let mut prng = CSPRNG::from_csprng_seed(seed);
    CommittedSummands {
        records: (0..count).into_iter()
            .map(|n| {
                SummandRecord { position: n, summand: prng.gen_range(0, modulus) }
            }).collect()
    }
}
