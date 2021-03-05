//! # Interact with blockchain
//!
//! Post/read information to/from blockchain
//! Information posted is a merkle root

use crate::blockchain::merkle::{CryptoSHA3256Hash, MerkleRoot, new_tree, validate, CryptoHashData, pad_to_power_2};
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

    // Get voter info
    let roster = roster.records.into_iter()
        .map(|voter| {
            serde_yaml::to_string(&voter).unwrap()
        }).collect();
    let roster = pad_to_power_2(roster);
    /*
    let copy_data = &roster;
    for v in copy_data.into_iter() {
        println!("{}:{}", v, v.len()); 
    }
    */


    // Re-construct the audited ballots.
    let audited_ballots: Vec<String> = pollconf.audited_ballots.iter()
        .map(|audited| {
            serde_yaml::to_string(&audited).unwrap()
        }).collect();
    
    let mut data = CryptoHashData::new(roster);
    data.push_vec(audited_ballots);
    data.pad();
    
    let merkle_tree = new_tree(data).unwrap();
    println!("Root {}", hex::encode(merkle_tree.root()));

    panic!("done");
    /*
    let mut serialized_data = CryptoHashData::new();
    serialized_data.add_vec(serialized_roster);
    serialized_data.add(serialized_audited);
    serialized_data.complete();
    
    post_on_chain(serialized_data)
    */
    true
}