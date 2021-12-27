//! # Interact with blockchain
//!
//! Post/read information to/from blockchain
//! Information posted is a merkle root

// Imports for merkle tree handling
use crate::blockchain::merkle::{CryptoSHA3256Hash, new_tree, CryptoHashData, store_tree};
use crate::Result;
use crate::voter_roster::VoterRoster;
use crate::poll_configuration::PollConfiguration;
use crate::planes::Plane;
use crate::debug;
use hex;
use std::fs::File;
use serde::{Serialize, Deserialize};


// Imports for blockchain audit
use crate::untagged::*;
use crate::untagged::{Ballot};
use std::collections::HashMap;
use crate::blockchain::etherscan::{Transaction, Response, SubmittedVote, ResponseBlockNumber};
use chrono::NaiveDateTime;

// Imports to interact with blockchain (web3)
use web3::types::{BlockNumber, Address, TransactionParameters, U256, CallRequest};
use web3::signing::Key;
use secp256k1::SecretKey;
use web3::signing::SecretKeyRef;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NetworkConfig {
    node: String,
    key: String,
    api: String
}

// returns block #
pub fn retrieve_from_chain(value: Vec<u8>) -> u64 {
    let _value = value;
    0
}

// Map votecodes to choice value
// More efficient for vote count
pub fn map_votes(ballots: &Vec<Ballot>) -> Result<HashMap<VoteCode, ChoiceValue>> {
    let mut choices = HashMap::new();
    
    // Each votecode is maped to its corresponding Choice value
    // p.e 1234-1234-1234-1234 => ChoiceValue::For
    ballots.into_iter()
            .for_each(|ballot| {
                // println!("{} {:?} {:?}", ballot.serial, ballot.choice1.votecode, ballot.choice2.votecode);
                choices.insert(ballot.choice1.votecode, ballot.choice1.choice);
                choices.insert(ballot.choice2.votecode, ballot.choice2.choice);
    });

    Ok(choices)
}

pub fn map_serials(ballots: &Vec<Ballot>) -> Result<HashMap<VoteCode, BallotSerial>> {
    let mut serials = HashMap::new();
    
    // Each votecode is maped to its corresponding Choice value
    // p.e 1234-1234-1234-1234 => ChoiceValue::For
    ballots.into_iter()
            .for_each(|ballot| {
                // println!("{} {:?} {:?}", ballot.serial, ballot.choice1.votecode, ballot.choice2.votecode);
                serials.insert(ballot.choice1.votecode, ballot.serial);
                serials.insert(ballot.choice2.votecode, ballot.serial);
    });

    Ok(serials)
}

// Decode the vote from the transcation input
pub fn transaction_to_votecode(transaction: Transaction) -> Option<SubmittedVote> {
    // Remove '0x' from hex input
    let vote = &transaction.input[2..];
    
    // Decode rest of input into u8
    let vote: Vec<u8> = match  hex::decode(vote) {
        Ok(votecode) => votecode,
        _ => return None
    };

    let vote: String = match String::from_utf8(vote){
        Ok(votecode) => votecode,
        _ => return None
    };

    let vote: SubmittedVote = match serde_json::from_str(&vote) {
        Ok(votecode) => votecode,
        _ => return None
    };

    Some(vote)
}

// Count the votes found in the blockchain
pub fn count_votes(mut choices: HashMap<VoteCode, ChoiceValue>, serials: HashMap<VoteCode, BallotSerial>, transactions: Vec<Transaction>, pollconf: PollConfiguration, decoys: Vec<usize>) -> Result<()> {

    let mut vote_for: u64 = 0;
    let mut vote_against: u64 = 0;

    let option1 = pollconf.option1;
    let option2 = pollconf.option2;
    
    // Re-construct the audited ballots.
    let audited_ballots: Vec<BallotSerial> = {
        pollconf.audited_ballots.clone().unwrap().iter()
            .map(|serial| usize::from_str_radix(serial, 10).unwrap())
            .collect()
    };
    
    // Run through all transactions
    transactions.into_iter()
        .for_each(|transaction| {     
           // Get vote from transaction
            if let Some(vote) = transaction_to_votecode(transaction) {
                    let votecode = vote.to_votecode().unwrap();
                    // println!("{:?}", votecode);

                    // Get vote serial number
                    if let Some(vote_serial) = serials.get(&votecode) {
                        //println!("{:?}", vote_serial);

                        // If vote is not audit and not decoy, count vote
                        if !audited_ballots.contains(vote_serial) && !decoys.contains(vote_serial) {

                            // Get ChoiceValue of vote
                            if let Some(choice) = choices.remove(&votecode) {
                                
                                // println!("{:?}: {:?}", vote, choice);
                                // If both votecodes are submitted, they cancel eachother
                                // Increment the correct counter
                                match choice {
                                    ChoiceValue::For => vote_for += 1,
                                    ChoiceValue::Against => vote_against += 1,
                                }
                            }
                        }
                    }
            }
        });
    
    println!("\"{}\": {},  \"{}\": {}", option1, vote_for, option2, vote_against);
    Ok(())
}

fn get_block_by_timestamp(timestamp: String, api: &String) -> Result<String> {
    let url = format!("https://api-ropsten.etherscan.io/api?module=block&action=getblocknobytime&timestamp={}&closest=before&apikey={}", timestamp, api);

    let response = async {
        let resp = reqwest::get(&url).await.expect("Error requesting data");
        let text = resp.text().await.expect("Error retrieving data form request");
        let data: ResponseBlockNumber = serde_json::from_str(&text).expect("Problem parsing response from etherscan");
        data.result
    };
    
    Ok(web3::block_on(response))

}

// Get data associated with address
pub fn get_data(addr: Address, api: String, start_date: &str, end_date: &str) -> Result <Vec<Transaction>> {
    let addr = String::from("0x") + &hex::encode(addr.0);
    
    let start = NaiveDateTime::parse_from_str(&format!("{} 00:00:00", start_date), "%Y-%m-%d %H:%M:%S")?;
    let end = NaiveDateTime::parse_from_str(&format!("{} 00:00:00", end_date), "%Y-%m-%d %H:%M:%S")?;

    let start_block = get_block_by_timestamp(start.timestamp().to_string(), &api)?;
    let end_block = get_block_by_timestamp(end.timestamp().to_string(), &api)?;

    debug!("Looking for blocks from {} ({}) to {} ({})",start_date, start_block, end_date, end_block);

    let url = format!("https://api-ropsten.etherscan.io/api?module=account&action=txlist&address={}&startblock={}&endblock={}&sort=asc&apikey={}", addr, start_block, end_block, api);

    let response = async {
        let resp = reqwest::get(&url).await.expect("Error requesting data");
        let text = resp.text().await.expect("Error retrieving data form request");
        let data: Response = serde_json::from_str(&text).expect("Problem parsing response from etherscan");
        data.result
    };
    
    Ok(web3::block_on(response))
}

// Audit blockchain for votecodes
// Count votes
pub fn audit_votes(ballots: Vec<Ballot>, pollconf: PollConfiguration, xxn_config: &str, decoys: Vec<usize>) -> Result<()> {
    // Load configuration file
    let config = load_xxn(xxn_config)?;
    
    // Get private key from config
    let key = SecretKey::from_slice(&hex::decode(config.key)?)?;
    let key = SecretKeyRef::new(&key);
    
    // Get public address of private key
    let pub_addr: Address = key.address();
    
    // Map vote codes to choices values
    let choices: HashMap<VoteCode, ChoiceValue> = map_votes(&ballots)?;
    let serials: HashMap<VoteCode, BallotSerial> = map_serials(&ballots)?;

    // Get data associated with poll addr -> votes submited via web interface
    let data: Vec<Transaction> = get_data(pub_addr, config.api, &pollconf.start_date, &pollconf.end_date)?;

    // Count the votes
    count_votes(choices, serials, data, pollconf, decoys)
}

// Load blockchain network configurations
fn load_xxn(config: &str) -> Result<NetworkConfig>{
    let config = File::open(config)?;
    let config: NetworkConfig  = serde_yaml::from_reader(config).expect("Error loading XXN config file");

    Ok(config)
}

pub fn post(xxn: &str, data: CryptoSHA3256Hash) -> Result<()> {
    // Load configuration file
    let config = load_xxn(xxn)?;

    // Get private key from config
    let key = SecretKey::from_slice(&hex::decode(config.key)?)?;
    let key = SecretKeyRef::new(&key);
    
    // Get public address of private key
    let pub_addr: Address = key.address();
    let uri = config.node;

    // Placeholder request to be used to estimate gas
    let req = CallRequest {
        from: None,
        to: None,
        gas: None,
        gas_price: None,
        value: None,
        data: None
    };

    // Start web3 class
    let transport = web3::transports::Http::new(&uri).unwrap();
    let web3 = web3::Web3::new(transport);
    
    let send = async {
        // Get last block and estimate gas
        let block_number = web3.eth().block_number().await.expect("Error getting last block number");
        let gas = web3.eth().estimate_gas(req, Some(BlockNumber::Number(block_number))).await.expect("Error getting gas value");

        // Build transaction with data to post
        let params = TransactionParameters {
            nonce: None,
            to: Some(pub_addr), // Send to own address
            gas_price: None,
            chain_id: None,
            data: data.into(), // Data to be posted
            value: U256::zero(),
            gas: gas
        };

        // Sign transaction before posting
        let signed = web3.accounts().sign_transaction(params, key).await.expect("Error signing transaction");
        let transaction = signed.raw_transaction;

        // Send signed transaction
        let sent = web3.eth().send_raw_transaction(transaction.into()).await.expect("Error sending transaction");
        debug!("Transaction Hash: {:?}", sent);

    };

    web3::block_on(send);
    Ok(())   
}

pub fn commit (xxn: &str, pollconf: PollConfiguration, planes: Vec<Plane>) -> Result<()> {
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
            let ser_v = serde_yaml::to_string(&voter).unwrap();
            ser_v
        }).collect();


    // Re-construct the audited ballots.
    let audited_ballots = pollconf.audited_ballots.to_owned().unwrap();
    
    // Start vec of data for the tree
    // Push roster
    let mut data = CryptoHashData::new(roster);

    // Push audited ballots
    data.push_vec(audited_ballots);
   
    // Push planes
    planes.into_iter().for_each(|plane|
    {        
        plane.rows.into_iter().for_each(|row|
        {
            let ser_row = row.serializable(pollconf.num_ballots);

            // Each row cell is a leaf
            data.push(ser_row.col1);
            data.push(ser_row.col3);
        });
    });

    // After all data is in vec, pad it to be pow 2
    data.pad();


    // Create new tree with Vec of data
    let merkle_tree = new_tree(data).unwrap();
    debug!("Root: {}", hex::encode(merkle_tree.root()));

    // Store full tree in file, to be later used for proof of inclusions
    store_tree(&merkle_tree, String::from("merkle.yaml"))?;

    // Post root to blockchain
    post(xxn, merkle_tree.root())
}