//! Structures for the complete poll configuration.
//!
//! The Poll Configuration comprises:
//! * Poll Master Key (PMK)
//! * Plane Key [N] = GEN(0x1 || N || PMK)
//! * Plane Row Permutation Seed [N] = GEN(0x2 || N || PMK)
//!

use super::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollConfigurationTrustee {
    pub identifier: String,
    pub share: AEADString
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollConfiguration {
    pub poll_state: PollState,
    pub signing_key: Base64String,
    pub num_ballots: usize,
    pub num_decoys: usize,
    pub voter_roster: Option<Base64String>,
    pub voter_roster_size: usize,
    pub voter_privacy: bool,
    pub drawn_summands_seed: Option<String>,
    pub audited_columns_seed: Option<String>,
    pub audited_ballots: Option<Vec<String>>,
    pub votes: Option<Vec<VoteCode>>,
    pub question: String,
    pub option1: String,
    pub option2: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollState {
    pub announced: bool,
    pub roster_committed: bool,
    pub columns_committed: bool,
    pub summands_committed: bool,
    pub summands_drawn: bool,
    pub ceremony_conducted: bool,
    pub votes_committed: bool,
    pub summands_revealed: bool,
    pub roster_revealed: bool,
    pub columns_revealed: bool
}

impl PollState {
    pub fn new() -> Self {
        PollState {
            announced: false,
            roster_committed: false,
            columns_committed: false,
            summands_committed: false,
            summands_drawn: false,
            ceremony_conducted: false,
            votes_committed: false,
            summands_revealed: false,
            roster_revealed: false,
            columns_revealed: false
        }
    }
}

