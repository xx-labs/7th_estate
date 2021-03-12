//! # Interact with blockchain
//!
//! Post/read information to/from blockchain
//! Information posted is a merkle root

use crate::blockchain::merkle::{CryptoSHA3256Hash, new_tree, CryptoHashData};
use crate::Result;
use crate::voter_roster::VoterRoster;
use crate::poll_configuration::PollConfiguration;
use crate::planes::Plane;
use crate::debug;

// returns block #
pub fn retrieve_from_chain(value: Vec<u8>) -> u64 {
    0
}

fn post(data: CryptoSHA3256Hash) -> Result<bool> {
    Ok(true)
}

pub fn commit (pollconf: PollConfiguration, planes: Vec<Plane>) -> bool {
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


    // Re-construct the audited ballots.
    let audited_ballots: Vec<String> = pollconf.audited_ballots.iter()
        .map(|audited| {
            serde_yaml::to_string(&audited).unwrap()
        }).collect();
    
    let mut data = CryptoHashData::new(roster);
    data.push_vec(audited_ballots);
   
    planes.into_iter().for_each(|plane|
    {        
        plane.rows.into_iter().for_each(|row|
        {
            let ser_row = row.serializable(pollconf.num_ballots);

            // Each row entry is a leaf
            data.push(ser_row.col1);
            data.push(ser_row.col3);
        });
    });
    data.pad();

    let merkle_tree = new_tree(data).unwrap();
    debug!("Root: {}", hex::encode(merkle_tree.root()));
    post(merkle_tree.root()).unwrap()    
}