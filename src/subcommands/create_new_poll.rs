//! # Command: Create New Poll
//!
//! `create_new_poll` generates a secured poll configuration
//! from a new poll configuration file.

use super::*;


pub fn create_new_poll(pollconf_filename: &str) -> Result<()> {
    let pollconf_path = Path::new(pollconf_filename);
    let securepollconf_filename = pollconf_filename.to_owned() + ".secure";
    let securepollconf_path = Path::new(&securepollconf_filename);

    // Read poll configuration file.
    let new_poll_configuration: NewPollConfiguration = {
        let pollconf_file = File::open(pollconf_path)?;
        serde_yaml::from_reader(pollconf_file)?
    };

    // Generate Master Key and Shares.
    let num_trustees: usize = new_poll_configuration.poll_trustees.len();
    let poll_master_key = PollMasterKey::new();
    let aead_pmk = AEADKey::from(poll_master_key.0.clone());
    let poll_master_key_shares = poll_master_key.share(num_trustees);

    // Secure shares with trustee passwords.
    let secure_key_shares = new_poll_configuration.poll_trustees.iter()
        .zip(poll_master_key_shares.iter())
        .map(|(trustee, share)| {
            let trustee_share = TrusteeShare::new(
                trustee.identifier.clone(),
                share.0.clone());
            trustee_share.secure()
        })
        .map(|trustee_share| PollConfigurationTrustee {
            identifier: trustee_share.identifier.clone(),
            share: trustee_share.secure_value.clone()
        })
        .collect::<Vec<PollConfigurationTrustee>>();

    // Generate signing key/certificate.
    let (private_key, public_key): (Base64String, Base64String) = new_signing_key()?;

    // Create new poll configuration file
    let pollconf = PollConfiguration {
        poll_state: PollState::new(),
        signing_key: private_key,
        num_ballots: new_poll_configuration.num_ballots,
        num_decoys: new_poll_configuration.num_decoys,
        question: new_poll_configuration.question,
        option1: new_poll_configuration.option1,
        option2: new_poll_configuration.option2,
        start_date: new_poll_configuration.start_date,
        end_date: new_poll_configuration.end_date,
        voter_roster: None,
        voter_roster_size: 0,
        voter_privacy: true,
        drawn_summands_seed: None,
        audited_columns_seed: None,
        audited_ballots: None,
        votes: None
    };
    let serialized_pollconf = serde_yaml::to_string(&pollconf)?;
    //debug!("{}\n", serialized_pollconf);

    // Encrypt the properties needed for the secure file.
    let secure_poll_identifier = AEADString::from_values(
        aead_authenticate(&aead_pmk,
                         new_poll_configuration.poll_identifier.as_bytes().to_vec())?);
    let secure_public_key = AEADString::from_values(
        aead_authenticate(&aead_pmk,
                          public_key.0.as_bytes().to_vec())?);
    let secure_serialized_pollconf = AEADString::from_values(
        aead_encrypt(&aead_pmk,
                     Vec::new(),
                     serialized_pollconf.as_bytes().to_vec())?);

    // Write poll configuration out to secure file.
    let secure_poll_configuration: SecuredPollConfiguration = SecuredPollConfiguration {
        poll_identifier: secure_poll_identifier,
        poll_trustees: secure_key_shares,
        encrypted_poll_configuration: secure_serialized_pollconf,
        signing_certificate: secure_public_key
    };
    //debug!("{:#?}\n", secure_poll_configuration);
    serde_yaml::to_writer(
        File::create(securepollconf_path)?,
        &secure_poll_configuration)?;

    Ok(())
}

pub fn create_new_poll_run(pollconf_filename: &str) -> Result<(SecuredPollConfiguration, PollMasterKey, AEADKey)> {
    let pollconf_path = Path::new(pollconf_filename);
    let securepollconf_filename = pollconf_filename.to_owned() + ".secure";
    let securepollconf_path = Path::new(&securepollconf_filename);

    // Read poll configuration file.
    let new_poll_configuration: NewPollConfiguration = {
        let pollconf_file = File::open(pollconf_path)?;
        serde_yaml::from_reader(pollconf_file)?
    };

    // Generate Master Key and Shares.
    let num_trustees: usize = new_poll_configuration.poll_trustees.len();
    let poll_master_key = PollMasterKey::new();
    let aead_pmk = AEADKey::from(poll_master_key.0.clone());
    let poll_master_key_shares = poll_master_key.share(num_trustees);

    // Secure shares with trustee passwords.
    let secure_key_shares = new_poll_configuration.poll_trustees.iter()
        .zip(poll_master_key_shares.iter())
        .map(|(trustee, share)| {
            let trustee_share = TrusteeShare::new(
                trustee.identifier.clone(),
                share.0.clone());
            trustee_share.secure()
        })
        .map(|trustee_share| PollConfigurationTrustee {
            identifier: trustee_share.identifier.clone(),
            share: trustee_share.secure_value.clone()
        })
        .collect::<Vec<PollConfigurationTrustee>>();

    // Generate signing key/certificate.
    let (private_key, public_key): (Base64String, Base64String) = new_signing_key()?;

    // Create new poll configuration file
    let pollconf = PollConfiguration {
        poll_state: PollState::new(),
        signing_key: private_key,
        num_ballots: new_poll_configuration.num_ballots,
        num_decoys: new_poll_configuration.num_decoys,
        question: new_poll_configuration.question,
        option1: new_poll_configuration.option1,
        option2: new_poll_configuration.option2,
        start_date: new_poll_configuration.start_date,
        end_date: new_poll_configuration.end_date,
        voter_roster: None,
        voter_roster_size: 0,
        voter_privacy: true,
        drawn_summands_seed: None,
        audited_columns_seed: None,
        audited_ballots: None,
        votes: None
    };
    let serialized_pollconf = serde_yaml::to_string(&pollconf)?;
    //debug!("{}\n", serialized_pollconf);

    // Encrypt the properties needed for the secure file.
    let secure_poll_identifier = AEADString::from_values(
        aead_authenticate(&aead_pmk,
                         new_poll_configuration.poll_identifier.as_bytes().to_vec())?);
    let secure_public_key = AEADString::from_values(
        aead_authenticate(&aead_pmk,
                          public_key.0.as_bytes().to_vec())?);
    let secure_serialized_pollconf = AEADString::from_values(
        aead_encrypt(&aead_pmk,
                     Vec::new(),
                     serialized_pollconf.as_bytes().to_vec())?);

    // Write poll configuration out to secure file.
    let secure_poll_configuration: SecuredPollConfiguration = SecuredPollConfiguration {
        poll_identifier: secure_poll_identifier,
        poll_trustees: secure_key_shares,
        encrypted_poll_configuration: secure_serialized_pollconf,
        signing_certificate: secure_public_key
    };
    //debug!("{:#?}\n", secure_poll_configuration);
    serde_yaml::to_writer(
        File::create(securepollconf_path)?,
        &secure_poll_configuration)?;

    Ok((secure_poll_configuration, poll_master_key, aead_pmk))
}