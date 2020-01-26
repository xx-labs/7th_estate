//! # Command: Generate Print Files
//!
//! `generate_print_files` reads from the voter roster and poll secrets
//! from the secured poll configuration and writes out CSV files for the
//! address labels and ballot information. These files are intended
//! for mass printing.

use super::*;
use crate::voter_selection::select_voters;


#[derive(Debug, Clone, Serialize)]
pub struct AddressLabel {
    pub last_name: String,
    pub first_name: String,
    pub address1: String,
    pub address2: String,
    pub city: String,
    pub state: String,
    pub zip_code: String
}


#[derive(Debug, Clone, Serialize)]
pub struct CompleteBallotRow {
    pub serial: String,
    pub choice1_votecode: String,
    pub choice1_value: String,
    pub choice2_votecode: String,
    pub choice2_value: String
}

#[derive(Debug, Clone, Serialize)]
pub struct SplitBallotRow {
    pub serial: String,
    pub votecode: String,
    pub choice: String
}


pub fn generate_print_files(pollconf_filename: &str, addresses_filename: &str, ballots_filename: &str) -> Result<()> {
    // Read poll configuration file.
    let secured_poll_configuration = read_poll_configuration_file(pollconf_filename)?;

    // Reconstruct the Poll Master Key from the trustee passwords.
    let (poll_master_key, aead_pmk) = read_poll_master_key(&secured_poll_configuration);

    // Decrypt poll configuration state.
    let pollconf_aead_values = secured_poll_configuration.encrypted_poll_configuration.values()?;
    let serialized_pollconf = aead_decrypt(&aead_pmk, &pollconf_aead_values)?;
    let pollconf: PollConfiguration = serde_yaml::from_slice(&serialized_pollconf).unwrap();
    
    assert!(pollconf.poll_state.summands_drawn,
        "Summands must be drawn to generate voters and print content for public audit.");

    // Derive the poll secrets.
    let poll_secrets: PollSecrets = PollSecrets::derive(&poll_master_key);
    
    // Regenerate the Committed Summands.
    let committed_summands = CommittedSummands::from_csprng(
        poll_secrets.summands_root,
        pollconf.num_ballots,
        pollconf.voter_roster_size);
    // Regenerate the Drawn Summands.
    let drawn_summands_seed: Vec<u8> = {
        let seed = pollconf.drawn_summands_seed.clone();
        hex::decode(seed.unwrap())?
    };
    let drawn_summands = DrawnSummands::from_csprng(
        CSPRNGSeed::from_vec(&drawn_summands_seed),
        pollconf.num_ballots,
        pollconf.voter_roster_size);
    // Select the Voters.
    let roster_indices = select_voters(
        committed_summands,
        drawn_summands,
        pollconf.voter_roster_size)?;
    debug!("Selected Voters: {:?}", roster_indices);

    // Generate the Ballots.
    let serials: Vec<BallotSerial> = (0..pollconf.num_ballots).collect();
    let votecodes: Vec<VoteCode> = generate_votecodes(
        poll_secrets.votecode_root,
        2 * pollconf.num_ballots);
    let ballots = generate_ballots(&serials, &votecodes);
    debug!("Ballots: {:?}", ballots);

    // Print the Address Labels
    let roster: VoterRoster = {
        let encoded_roster = pollconf.voter_roster.clone().unwrap();
        let decoded_roster = base64::decode(&encoded_roster.0)?;
        let serialized_roster = str::from_utf8(&decoded_roster)?;
        serde_yaml::from_str(serialized_roster)?
    };
    let addresses: Vec<AddressLabel> = roster_indices.iter()
        .map(|&n| { roster.records[n].voter_info.clone() })
        .map(|voter| {
            AddressLabel {
                last_name: voter.last_name,
                first_name: voter.first_name,
                address1: voter.street_address,
                address2: "".to_owned(),
                city: voter.city,
                state: voter.state,
                zip_code: voter.zip_code
            }
        }).collect();
    let address_labels_path = Path::new(addresses_filename);
    let mut csvwriter = csv::Writer::from_path(address_labels_path)?;
    addresses.iter()
        .for_each(|record| { csvwriter.serialize(record).unwrap(); });

    // Print the Ballots
    let ballots_path = Path::new(ballots_filename);
    let mut csvwriter = csv::Writer::from_path(ballots_path)?;
    ballots.iter()
        .for_each(|ballot| {
            let record = CompleteBallotRow {
                serial: string_from_ballotserial(&ballot.serial, pollconf.num_ballots),
                choice1_votecode: string_from_votecode(&ballot.choice1.votecode),
                choice1_value: string_from_choicevalue(&ballot.choice1.choice),
                choice2_votecode: string_from_votecode(&ballot.choice2.votecode),
                choice2_value: string_from_choicevalue(&ballot.choice2.choice)
            };
            debug!("{:?}", record);
            csvwriter.serialize(record).unwrap();
        });

    // No need to update the poll state since this is not a public operation.

    Ok(())
}


