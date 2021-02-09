//! # Interact with blockchain
//!
//! Post/read information to/from blockchain
//! Information posted is a merkle root

use crate::blockchain::merkle::{CryptoSHA3256Hash, MerkleRoot, new_tree, validate, CryptoHashData};
use crate::Result;
use crate::voter_roster::VoterRoster;
use crate::ballots::untagged::BallotSerial;
use crate::poll_configuration::PollConfiguration;
use super::*;

// returns true if successfully posted value to chain
fn post_on_chain(value: CryptoHashData) -> bool {
    let t: MerkleRoot = new_tree(value).unwrap();
    post(t.root()).unwrap()
}
// returns block #
pub fn retrieve_from_chain(value: Vec<u8>) -> u64 {
    0
}

pub fn data_on_blockchain(root: MerkleRoot, data: CryptoSHA3256Hash) -> bool {
    validate(root, data).unwrap()
}

fn post(data: CryptoSHA3256Hash) -> Result<bool> {
    Ok(true)
}

pub fn commit (pollconf: PollConfiguration) -> bool {
    // Re-construct roster
    let roster: VoterRoster = {
        let encoded_roster = pollconf.voter_roster.clone().unwrap();
        let decoded_roster = base64::decode(&encoded_roster.0).unwrap();
        let serialized_roster = std::str::from_utf8(&decoded_roster).unwrap();
        serde_yaml::from_str(serialized_roster).unwrap()
    };

    // Serialize roster into Vec<u8>
    let serialized_roster = roster.records.into_iter()
        .map(|voter| {
            bincode::serialize(&voter).unwrap()
        }).collect::<Vec<Vec<u8>>>();

    // Re-construct the audited ballots.
    let audited_ballots: Vec<BallotSerial> = {
        pollconf.audited_ballots.clone().unwrap().iter()
            .map(|serial| usize::from_str_radix(serial, 10).unwrap())
            .collect()
    };

    // Serialize audited ballots into [u8]
    let serialized_audited = audited_ballots.into_iter()
    .map(|ballot| {
        ballot as u8
    }).collect();

    let mut serialized_data = CryptoHashData::new();
    serialized_data.add_vec(serialized_roster);
    serialized_data.add(serialized_audited);
    serialized_data.complete();

    post_on_chain(serialized_data)
}