//! Run 7th Estate
//! 
//! Compiles all main 7th Estate commands into 2 commands
//! Avoid inputting passwords multiple times

use super::*;

pub fn start(
    pollconf_filename: &str,
    roster_filename: &str,
    disable_voter_privacy: bool,
    drawn_summands_seed: &str,
    address_label: &str,
    ballot_information: &str,
    audited_ballots: &str,
    xxn_config: &str
) -> Result<()> {
    // Start new Poll
    let (secure_poll_configuration, poll_master_key, aead_pmk) = create_new_poll_run(pollconf_filename)?;
    let secure_pollconf_filename = pollconf_filename.to_owned() + ".secure";

    // Bind roster to Poll
    let secure_poll_configuration = bind_roster_run(&secure_pollconf_filename, secure_poll_configuration, aead_pmk, roster_filename, disable_voter_privacy)?;

    // Step 1
    let secure_poll_configuration = generate_poll_commitments_run (&secure_pollconf_filename, secure_poll_configuration, poll_master_key.clone(), aead_pmk)?;

    // Step 2
    let secure_poll_configuration = generate_drawn_summands_run (&secure_pollconf_filename, secure_poll_configuration, aead_pmk, drawn_summands_seed)?;

    // Step 3
    generate_print_files_run (secure_poll_configuration.clone(), poll_master_key.clone(), aead_pmk, address_label, ballot_information)?;
    
    // Step 4
    record_audited_ballots_run (&secure_pollconf_filename, secure_poll_configuration, poll_master_key.clone(), aead_pmk, audited_ballots, xxn_config)
}

pub fn finish(
    poll_configuration: &str,
    xxn_config: &str,
    votes_file: &str,
    tally_audit_seed: &str
) -> Result<()> {

    // step 6
    let secure_pollconf_filename = poll_configuration.to_owned() + ".secure";
    let (secure_poll_configuration, poll_master_key, aead_pmk) = record_votes_run (&secure_pollconf_filename, votes_file)?;

    // step 7
    let secure_poll_configuration = generate_tally_audit_run (&secure_pollconf_filename, tally_audit_seed, secure_poll_configuration, aead_pmk)?;
    
    // step 8
    let secure_poll_configuration = generate_poll_revelations_run (&secure_pollconf_filename, secure_poll_configuration, poll_master_key.clone(), aead_pmk)?;

    // audit
    blockchain_audit_run (xxn_config, secure_poll_configuration, poll_master_key.clone(), aead_pmk)?;

    Ok(())
}