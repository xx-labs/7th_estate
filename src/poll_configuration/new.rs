//! # New Poll Configuration Data
//!
//! This file is used to generate a new poll.
//! This file contains no sensitive information or secrets.

use super::*;

#[derive(Debug, Clone, Deserialize)]
pub struct NewPollConfigurationTrustee { pub identifier: String }

#[derive(Debug, Clone, Deserialize)]
pub struct NewPollConfiguration {
    pub poll_identifier: String,
    pub poll_trustees: Vec<NewPollConfigurationTrustee>,
    pub num_ballots: usize,
    pub num_decoys: usize,
    pub question: String,
    pub option1: String,
    pub option2: String
}