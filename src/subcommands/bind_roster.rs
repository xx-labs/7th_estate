//! # Command: Bind Roster
//!
//! `bind_roster` binds a roster file to a secured poll configuration.
//! The purpose of this binding is to make the voter roster file
//! immutable without corrupting the entire poll.

use super::*;


pub fn bind_roster(pollconf_filename: &str, roster_filename: &str, disable_privacy: bool, force: bool) -> Result<()> {
    let pollconf_path = Path::new(pollconf_filename);
    let roster_path = Path::new(roster_filename);

    // Read poll configuration file.
    let mut secured_poll_configuration = read_poll_configuration_file(pollconf_filename)?;

    // Reconstruct the Poll Master Key from the trustee passwords.
    let (_poll_master_key, aead_pmk) = read_poll_master_key(&secured_poll_configuration);

    // Decrypt poll configuration state.
    let pollconf_aead_values = secured_poll_configuration.encrypted_poll_configuration.values()?;
    let serialized_pollconf = aead_decrypt(&aead_pmk, &pollconf_aead_values)?;
    let mut pollconf: PollConfiguration = serde_yaml::from_slice(&serialized_pollconf).unwrap();

    // TODO: Consider having a separate announcement step.
    pollconf.poll_state.announced = true;
    assert!(pollconf.poll_state.announced,
        "Cannot bind voter roster until poll is announced.");
    assert!(!pollconf.poll_state.roster_committed || force,
        "Voter roster already bound. To re-bind, pass --force.");

    // Read roster file.
    let roster = VoterRoster::from_file(&roster_path)?;
    let serialized_roster = serde_yaml::to_string(&roster)?;
    let roster64 = base64::encode(&serialized_roster);
    // Bind the roster.
    pollconf.voter_roster = Some(Base64String(roster64));
    pollconf.voter_roster_size = roster.len();
    pollconf.voter_privacy = !disable_privacy;
    pollconf.poll_state.roster_committed = true;
    // Re-encrypt the poll configuration.
    let serialized_pollconf = serde_yaml::to_string(&pollconf)?;
    let secure_serialized_pollconf = AEADString::from_values(
        aead_encrypt(&aead_pmk,
                     Vec::new(),
                     serialized_pollconf.as_bytes().to_vec())?);
    // Save the poll configuration.
    secured_poll_configuration.encrypted_poll_configuration = secure_serialized_pollconf;
    serde_yaml::to_writer(
        File::create(pollconf_path)?,
        &secured_poll_configuration)?;

    Ok(())
}


