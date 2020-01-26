//! # Table of Summands
//!
//! `generic` provides an implementation of unqualified position/summand pairs.
//! The purpose of this is to help distinguish between the CommittedSummands and
//! the DrawnSummands within the confines of the type system.

use super::{Serialize, Deserialize};

pub struct Summands {
    pub records: SummandRecords
}

pub type SummandRecords = Vec<SummandRecord>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct SummandRecord {
    pub position: usize,
    pub summand: usize
}


impl Summands {
    pub fn modular_sum(summands1: Summands, summands2: Summands, modulus: usize) -> Summands {
        assert!(summands1.records.len() == summands2.records.len(),
            "Number of summands must be equal.");
        Summands {
            records: summands1.records.iter()
                .zip(summands2.records.iter())
                .map(|(&rec1, &rec2)| SummandRecord::modular_sum(rec1, rec2, modulus))
                .collect()
        }
    }
    
    pub fn len(self: &Self) -> usize { self.records.len() }
}

impl SummandRecord {
    pub fn modular_sum(summand1: SummandRecord, summand2: SummandRecord, modulus: usize) -> SummandRecord {
        assert!(summand1.position == summand2.position,
            "Summand record position mismatch.");
        SummandRecord {
            position: summand1.position,
            summand: (summand1.summand + summand2.summand) % modulus
        }
    }
}

