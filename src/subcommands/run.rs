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

    // Bind roster to Poll
    let secure_poll_configuration = bind_roster_run(pollconf_filename, secure_poll_configuration, aead_pmk, roster_filename, disable_voter_privacy)?;

    // Step 1
    let secure_poll_configuration = generate_poll_commitments_run (pollconf_filename, secure_poll_configuration, poll_master_key.clone(), aead_pmk)?;

    // Step 2
    let secure_poll_configuration = generate_drawn_summands_run (pollconf_filename, secure_poll_configuration, aead_pmk, drawn_summands_seed)?;

    // Step 3
    generate_print_files_run (secure_poll_configuration.clone(), poll_master_key.clone(), aead_pmk, address_label, ballot_information)?;
    
    // Step 4
    record_audited_ballots_run (pollconf_filename, secure_poll_configuration, poll_master_key.clone(), aead_pmk, audited_ballots, xxn_config)
}