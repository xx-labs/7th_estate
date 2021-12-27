//! # Command: Generate Poll Commitments
//!
//! `generate_poll_commitments` reads the secrets from a secured
//! poll configuration and generates commitment files for the
//! committed summands and the column planes.

use super::*;

pub fn generate_poll_commitments(pollconf_filename: &str, force: bool) -> Result<()> {
    let pollconf_path = Path::new(pollconf_filename);

    // Read poll configuration file.
    let mut secured_poll_configuration = read_poll_configuration_file(pollconf_filename)?;

    // Reconstruct the Poll Master Key from the trustee passwords.
    let (poll_master_key, aead_pmk) = read_poll_master_key(&secured_poll_configuration);

    // Ensure the data directory exists.
    let datadir_path = ensure_poll_data_directory_exists(&secured_poll_configuration, &aead_pmk)?;

    // Decrypt poll configuration state.
    let pollconf_aead_values = secured_poll_configuration.encrypted_poll_configuration.values()?;
    let serialized_pollconf = aead_decrypt(&aead_pmk, &pollconf_aead_values)?;
    let mut pollconf: PollConfiguration = serde_yaml::from_slice(&serialized_pollconf).unwrap();

    assert!(pollconf.poll_state.roster_committed,
        "Voter roster must be bound to generate poll commitments.");
    assert!(!pollconf.poll_state.summands_committed || force,
        "Summands already committed. To re-commit, pass --force.");
    assert!(!pollconf.poll_state.columns_committed || force,
        "Columns already committed. To re-commit, pass --force.");

    // Derive the poll secrets.
    let poll_secrets: PollSecrets = PollSecrets::derive(&poll_master_key);
    debug!("{:?}", poll_secrets);

    // Commit the Roster.
    let committed_roster_path = {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(&datadir_path);
        pathbuf.push("committed_roster");
        pathbuf.set_extension("csv");
        pathbuf.into_boxed_path()
    };
    let committed_roster = {
        let full_roster: VoterRoster = {
            let encoded_roster = pollconf.voter_roster.clone().unwrap();
            let decoded_roster = base64::decode(&encoded_roster.0)?;
            let serialized_roster = str::from_utf8(&decoded_roster)?;
            serde_yaml::from_str(serialized_roster)?
        };

        // TODO: Implement voter privacy.
        if pollconf.voter_privacy {
            full_roster.restricted()
        } else {
            full_roster.restricted()
        }
    };
    committed_roster.to_file(&committed_roster_path)?;

    // Commit the Summands.
    let committed_summands_path = {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(&datadir_path);
        pathbuf.push("committed_summands");
        pathbuf.set_extension("yaml");
        pathbuf.into_boxed_path()
    };
    let committed_summands = CommittedSummands::from_csprng(
        poll_secrets.summands_root,
        pollconf.num_ballots,
        pollconf.voter_roster_size);
    let summands_commitment: SecuredFile = committed_summands.aead_commit(&poll_secrets.summands_key)?;
    serde_yaml::to_writer(
        File::create(committed_summands_path)?,
        &summands_commitment)?;

    // Commit the Column Planes.
    let column_planes: Vec<Plane> = generate_column_planes(
        &poll_secrets,
        NUMBER_OF_PLANES,
        2 * pollconf.num_ballots,
        pollconf.num_decoys)?;
    column_planes.iter().enumerate()
        .for_each(|(n, plane)| {
            let committed_planes_path = {
                let mut pathbuf = PathBuf::new();
                pathbuf.push(&datadir_path);
                pathbuf.push(format!("committed_plane_{:02}", n+1));
                pathbuf.set_extension("csv");
                pathbuf.into_boxed_path()
            };
            let psecrets = poll_secrets.plane_secrets[n].resolve(plane.len());
            let permuted_plane = plane.permute(&psecrets.permutation);
            let mut csvwriter = csv::Writer::from_path(committed_planes_path).unwrap();
            permuted_plane.rows.iter()
                .for_each(|rec| { csvwriter.serialize(rec).unwrap(); });
        });

    // Update the poll state.
    pollconf.poll_state.summands_committed = true;
    pollconf.poll_state.columns_committed = true;
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

pub fn generate_poll_commitments_run (pollconf_filename: &str, mut secured_poll_configuration: SecuredPollConfiguration, poll_master_key: PollMasterKey, aead_pmk: AEADKey) -> Result<SecuredPollConfiguration> {
    let pollconf_path = Path::new(pollconf_filename);

    // Ensure the data directory exists.
    let datadir_path = ensure_poll_data_directory_exists(&secured_poll_configuration, &aead_pmk)?;
    
    // Decrypt poll configuration state.
    let pollconf_aead_values = secured_poll_configuration.encrypted_poll_configuration.values()?;
    let serialized_pollconf = aead_decrypt(&aead_pmk, &pollconf_aead_values)?;
    let mut pollconf: PollConfiguration = serde_yaml::from_slice(&serialized_pollconf).unwrap();

    assert!(pollconf.poll_state.roster_committed,
        "Voter roster must be bound to generate poll commitments.");
    assert!(!pollconf.poll_state.summands_committed,
        "Summands already committed.");
    assert!(!pollconf.poll_state.columns_committed,
        "Columns already committed.");

    // Derive the poll secrets.
    let poll_secrets: PollSecrets = PollSecrets::derive(&poll_master_key);
    debug!("{:?}", poll_secrets);

    // Commit the Roster.
    let committed_roster_path = {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(&datadir_path);
        pathbuf.push("committed_roster");
        pathbuf.set_extension("csv");
        pathbuf.into_boxed_path()
    };
    let committed_roster = {
        let full_roster: VoterRoster = {
            let encoded_roster = pollconf.voter_roster.clone().unwrap();
            let decoded_roster = base64::decode(&encoded_roster.0)?;
            let serialized_roster = str::from_utf8(&decoded_roster)?;
            serde_yaml::from_str(serialized_roster)?
        };

        // TODO: Implement voter privacy.
        if pollconf.voter_privacy {
            full_roster.restricted()
        } else {
            full_roster.restricted()
        }
    };
    committed_roster.to_file(&committed_roster_path)?;

    // Commit the Summands.
    let committed_summands_path = {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(&datadir_path);
        pathbuf.push("committed_summands");
        pathbuf.set_extension("yaml");
        pathbuf.into_boxed_path()
    };
    let committed_summands = CommittedSummands::from_csprng(
        poll_secrets.summands_root,
        pollconf.num_ballots,
        pollconf.voter_roster_size);
    let summands_commitment: SecuredFile = committed_summands.aead_commit(&poll_secrets.summands_key)?;
    serde_yaml::to_writer(
        File::create(committed_summands_path)?,
        &summands_commitment)?;

    // Commit the Column Planes.
    let column_planes: Vec<Plane> = generate_column_planes(
        &poll_secrets,
        NUMBER_OF_PLANES,
        2 * pollconf.num_ballots,
        pollconf.num_decoys)?;
    column_planes.iter().enumerate()
        .for_each(|(n, plane)| {
            let committed_planes_path = {
                let mut pathbuf = PathBuf::new();
                pathbuf.push(&datadir_path);
                pathbuf.push(format!("committed_plane_{:02}", n+1));
                pathbuf.set_extension("csv");
                pathbuf.into_boxed_path()
            };
            let psecrets = poll_secrets.plane_secrets[n].resolve(plane.len());
            let permuted_plane = plane.permute(&psecrets.permutation);
            let mut csvwriter = csv::Writer::from_path(committed_planes_path).unwrap();
            permuted_plane.rows.iter()
                .for_each(|rec| { csvwriter.serialize(rec).unwrap(); });
        });

    // Update the poll state.
    pollconf.poll_state.summands_committed = true;
    pollconf.poll_state.columns_committed = true;
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


    Ok(secured_poll_configuration)
}

