//! Module for Voter Roster File rows.

use std::convert::From;
use serde::{Serialize, Deserialize};
use super::VoterInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoterRosterFileRow {
    pub last_name: String,
    pub first_name: String,
    pub street_address: String,
    pub city: String,
    pub state: String,
    pub zip_code: String
}

impl From<VoterRosterFileRow> for VoterInfo {
    fn from(row: VoterRosterFileRow) -> Self {
        VoterInfo {
            last_name: row.last_name,
            first_name: row.first_name,
            street_address: row.street_address,
            city: row.city,
            state: row.state,
            zip_code: row.zip_code
        }
    }
}

