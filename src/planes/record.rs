//! # Plane Record

use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct PlaneRecord {
    pub col1: Column1Entry,
    pub col2: Column2Entry,
    pub col3: Column3Entry
}

#[derive(Debug, Serialize)]
pub struct PlaneRecordFileRow {
    pub col1: String,
    pub col2: String,
    pub col3: String
}

impl PlaneRecord {
    pub fn mark_voted(self: &Self) -> Self {
        PlaneRecord {
            col1: self.col1.clone(),
            col2: Column2Entry::Entry(Vote::Voted),
            col3: self.col3.clone()
        }
    }

    pub fn mark_not_voted(self: &Self) -> Self {
        PlaneRecord {
            col1: self.col1.clone(),
            col2: Column2Entry::Entry(Vote::NotVoted),
            col3: self.col3.clone()
        }
    }

    pub fn decrypt(self: &Self, filter: &PlaneFilterRecord) -> Self {
        PlaneRecord {
            col1: self.col1.decrypt(&filter.col1),
            col2: self.col2.clone(),
            col3: self.col3.decrypt(&filter.col3)
        }
    }

    pub fn serializable(self: &Self, num_ballots: usize) -> PlaneRecordFileRow {
        let _num_ballots = num_ballots;
        PlaneRecordFileRow {
            col1: match &self.col1 {
                Column1Entry::Entry(svc) => svc.clone(),
                /*
                {
                    format!("{}: {}",
                        string_from_ballotserial(&svc.serial, num_ballots),
                        string_from_votecode(&svc.votecode))
                },
                */
                Column1Entry::Encrypted(v) => v.0.clone()
            },
            col2: match self.col2 {
                Column2Entry::Empty => "".to_owned(),
                Column2Entry::Entry(voted) => {
                    match voted {
                        Vote::Voted => "Voted".to_owned(),
                        Vote::NotVoted => "Not Voted".to_owned()
                    }
                }
            },
            col3: match &self.col3 {
                Column3Entry::Entry(choice) => choice.trim().to_owned(),
                //Column3Entry::Entry(choice) => string_from_taggedchoicevalue(&choice),
                Column3Entry::Encrypted(v) => v.0.clone()
            }
        }
    }
}

