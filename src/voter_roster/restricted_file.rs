//! Module for the Restricted Voter Roster File rows.

use serde::Serialize;
use super::RestrictedVoterInfo;

#[derive(Serialize)]
pub struct RestrictedVoterRosterFileRow {
    pub position: usize,
    pub restricted: RestrictedVoterInfo
}

