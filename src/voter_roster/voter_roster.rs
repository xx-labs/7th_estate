//! Implementation for the Voter Roster.

use serde::{Serialize, Deserialize};
use super::*;


/// Voter Roster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoterRoster {
    pub records: VoterRosterRecords
}

/// List of voter roster records
pub type VoterRosterRecords = Vec<VoterRosterRecord>;

/// Voter Roster Record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoterRosterRecord {
    pub position: usize,
    pub voter_info: VoterInfo
}

/// Voter Information contained in the roster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoterInfo {
    pub last_name: String,
    pub first_name: String,
    pub street_address: String,
    pub city: String,
    pub state: String,
    pub zip_code: String
}

impl VoterRoster {
    pub fn from_file(path: &dyn AsRef<Path>) -> Result<Self> {
        let mut csvreader = csv::Reader::from_path(path)?;
        let records = csvreader.deserialize::<VoterRosterFileRow>();
        Ok(VoterRoster {
            records: records.enumerate()
                .map(|(n, result)| {
                    VoterRosterRecord {
                        position: n,
                        voter_info: VoterInfo::from(result.unwrap())
                    }})
                .collect::<VoterRosterRecords>()
        })
    }

    pub fn len(self: &Self) -> usize {
        self.records.len()
    }

    pub fn restricted(self: &Self) -> RestrictedVoterRoster {
        RestrictedVoterRoster::from((*self).clone())
    }
}

