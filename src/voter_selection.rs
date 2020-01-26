//! # Voter Selection
//!
//! Voters are selected by combining the committed and drawn summands
//! modulo the number of entries in the voter roster.

use std::convert::From;
use crate::Result;
use crate::summands::*;

pub type VoterRosterIndices = Vec<usize>;

pub fn select_voters(committed: CommittedSummands, drawn: DrawnSummands, roster_size: usize) -> Result<VoterRosterIndices> {
    assert!(committed.len() == drawn.len(),
        "Number of committed summands and number of drawn summands must be equal.");
    Ok(Summands::from(committed.clone()).records.iter()
        .zip(Summands::from(drawn.clone()).records.iter())
        .map(|(crec, drec)| {
            assert!(crec.position == drec.position,
                "Summand record positions do not match.");
            (crec.summand + drec.summand) % roster_size
        })
        .collect::<VoterRosterIndices>())
}

