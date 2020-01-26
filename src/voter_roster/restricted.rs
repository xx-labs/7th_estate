//! Restricted Voter Roster Module

use super::*;

/// Restricted Voter Roster
pub struct RestrictedVoterRoster {
    pub records: RestrictedVoterRosterRecords
}

/// List of restricted voter roster records
type RestrictedVoterRosterRecords = Vec<RestrictedVoterRosterRecord>;

/// Restricted voter roster record
pub struct RestrictedVoterRosterRecord {
    pub position: usize,
    pub voter_info: RestrictedVoterInfo
}

/// Restricted voter information
pub type RestrictedVoterInfo = String;


impl RestrictedVoterRoster {
    pub fn to_file(self: &Self, path: &dyn AsRef<Path>) -> Result<()> {
        let mut csvwriter = csv::Writer::from_path(path)?;
        self.records.iter().for_each(|record| {
            csvwriter.serialize(RestrictedVoterRosterFileRow {
                position: record.position,
                restricted: record.voter_info.clone()
            }).unwrap();
        });
        Ok(())
    }
}

impl From<VoterRoster> for RestrictedVoterRoster {
    fn from(roster: VoterRoster) -> Self {
        RestrictedVoterRoster {
            records: roster.records.iter().map(|record| {
                RestrictedVoterRosterRecord::from((*record).clone())
            }).collect::<RestrictedVoterRosterRecords>()
        }
    }
}

impl From<VoterRosterRecord> for RestrictedVoterRosterRecord {
    fn from(record: VoterRosterRecord) -> Self {
        RestrictedVoterRosterRecord {
            position: record.position,
            voter_info: RestrictedVoterInfo::from(record.voter_info)
        }
    }
}

impl From<VoterInfo> for RestrictedVoterInfo {
    fn from(info: VoterInfo) -> Self {
        let public = format!("{}, {}", info.last_name, info.first_name);
        let private = format!("{}\n{} {}, {}",
                info.street_address,
                info.city,
                info.state,
                info.zip_code);
        format!("{}\n{}", public, private)
    }
}


