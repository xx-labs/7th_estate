//! # Command: Generate Poll Revelations
//!

use std::io::Write;
use super::*;


pub fn generate_poll_revelations(pollconf_filename: &str, force: bool) -> Result<()> {
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
    
    assert!(pollconf.poll_state.votes_committed,
        "Votes must be committed prior to auditing the tally.");
    assert!(!pollconf.poll_state.summands_revealed || force,
        "Summands already revealed. To re-reveal, pass --force.");
    assert!(!pollconf.poll_state.columns_revealed || force,
        "Column Planes already revealed. To re-commit, pass --force.");
    assert!(!pollconf.poll_state.roster_revealed || force,
        "Voter roster already revealed. To re-commit, pass --force.");

    // Derive the poll secrets.
    let poll_secrets: PollSecrets = PollSecrets::derive(&poll_master_key);

    // Re-construct the audited ballots.
    let audited_ballots: Vec<BallotSerial> = {
        pollconf.audited_ballots.clone().unwrap().iter()
            .map(|serial| usize::from_str_radix(serial, 10).unwrap())
            .collect()
    };

    // Re-construct the marked votes.
    let votes: Vec<VoteCode> = pollconf.votes.clone().unwrap();
    let marked_rows: Vec<usize> = {
        let votecodes: Vec<VoteCode> = generate_votecodes(
            poll_secrets.votecode_root,
            2 * pollconf.num_ballots);
        votecodes.iter().enumerate()
            .filter_map(|(n, vc)| {
                debug!("{:?}", vc);
                if votes.contains(vc) { Some(n) }
                else { None }
            }).collect()
    };

    // Reveal Committed Summands
    let committed_summands_revealed_path = {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(&datadir_path);
        pathbuf.push("committed_summands_revealed");
        pathbuf.set_extension("csv");
        pathbuf.into_boxed_path()
    };
    let committed_summands = CommittedSummands::from_csprng(
        poll_secrets.summands_root,
        pollconf.num_ballots,
        pollconf.voter_roster_size);
    let mut csvwriter = csv::Writer::from_path(committed_summands_revealed_path)?;
    Summands::from(committed_summands).records.iter().for_each(|summand| {
        csvwriter.serialize(summand).unwrap();
    });
    // Reveal Committed Summands Key
    let committed_summands_key_path = {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(&datadir_path);
        pathbuf.push("committed_summands_key");
        pathbuf.set_extension("key");
        pathbuf.into_boxed_path()
    };
    File::create(committed_summands_key_path)?
        .write(base64::encode(&poll_secrets.summands_key.0).as_bytes())?;

    // Reveal Audited Columns
    let audited_columns_seed: Vec<u8> = {
        let seed = pollconf.audited_columns_seed.clone();
        hex::decode(seed.unwrap())?
    };
    let audited_columns: Vec<usize> = {
        let seed = CSPRNGSeed::from_vec(&audited_columns_seed);
        let mut prng = CSPRNG::from_csprng_seed(seed);
        (0..NUMBER_OF_PLANES).into_iter().map(|_| prng.gen_range(0, 2)).collect()
    };

    // Post the Fully Audited Column Planes.
    let column_planes: Vec<Plane> = generate_column_planes(
        &poll_secrets,
        NUMBER_OF_PLANES,
        2 * pollconf.num_ballots,
        pollconf.num_decoys)?;
    // Filter planes.
    column_planes.iter().enumerate()
        .for_each(|(n, plane)| {
            let posted_planes_path = {
                let mut pathbuf = PathBuf::new();
                pathbuf.push(&datadir_path);
                pathbuf.push(format!("final_plane_{:02}", n+1));
                pathbuf.set_extension("csv");
                pathbuf.into_boxed_path()
            };
            let posted_keys_path = {
                let mut pathbuf = PathBuf::new();
                pathbuf.push(&datadir_path);
                pathbuf.push(format!("final_plane_{:02}_keys", n+1));
                pathbuf.set_extension("csv");
                pathbuf.into_boxed_path()
            };
            let psecrets = poll_secrets.plane_secrets[n].resolve(plane.len());
            let filter = PlaneFilter::from(&psecrets.col1_keys, &psecrets.col3_keys)
                .decrypt_serials(&audited_ballots)
                .decrypt_column(if audited_columns[n] == 0 {1} else {3});

            let permuted_plane = plane.mark_rows(&marked_rows).decrypt(&filter).permute(&psecrets.permutation);
            let mut csvwriter = csv::Writer::from_path(posted_planes_path).unwrap();
            permuted_plane.rows.iter()
                .for_each(|rec| {
                    csvwriter.serialize(rec.serializable(pollconf.num_ballots)).unwrap();
                });
            
            let permuted_filter = filter.permute(&psecrets.permutation);
            let mut csvwriter = csv::Writer::from_path(posted_keys_path).unwrap();
            permuted_filter.serializable().iter()
                .for_each(|rec| {
                    csvwriter.serialize(rec).unwrap();
                });
        });

    // Update the poll state.
    pollconf.poll_state.roster_revealed = true;
    pollconf.poll_state.summands_revealed = true;
    pollconf.poll_state.columns_revealed = true;
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


pub fn generate_poll_revelations_run(pollconf_filename: &str, mut secured_poll_configuration: SecuredPollConfiguration, poll_master_key: PollMasterKey, aead_pmk: AEADKey) -> Result<SecuredPollConfiguration> {
    let pollconf_path = Path::new(pollconf_filename);

    // Ensure the data directory exists.
    let datadir_path = ensure_poll_data_directory_exists(&secured_poll_configuration, &aead_pmk)?;

    // Decrypt poll configuration state.
    let pollconf_aead_values = secured_poll_configuration.encrypted_poll_configuration.values()?;
    let serialized_pollconf = aead_decrypt(&aead_pmk, &pollconf_aead_values)?;
    let mut pollconf: PollConfiguration = serde_yaml::from_slice(&serialized_pollconf).unwrap();
    
    assert!(pollconf.poll_state.votes_committed,
        "Votes must be committed prior to auditing the tally.");
    assert!(!pollconf.poll_state.summands_revealed,
        "Summands already revealed. To re-reveal.");
    assert!(!pollconf.poll_state.columns_revealed,
        "Column Planes already revealed. To re-commit.");
    assert!(!pollconf.poll_state.roster_revealed,
        "Voter roster already revealed. To re-commit.");

    // Derive the poll secrets.
    let poll_secrets: PollSecrets = PollSecrets::derive(&poll_master_key);

    // Re-construct the audited ballots.
    let audited_ballots: Vec<BallotSerial> = {
        pollconf.audited_ballots.clone().unwrap().iter()
            .map(|serial| usize::from_str_radix(serial, 10).unwrap())
            .collect()
    };

    // Re-construct the marked votes.
    let votes: Vec<VoteCode> = pollconf.votes.clone().unwrap();
    let marked_rows: Vec<usize> = {
        let votecodes: Vec<VoteCode> = generate_votecodes(
            poll_secrets.votecode_root,
            2 * pollconf.num_ballots);
        votecodes.iter().enumerate()
            .filter_map(|(n, vc)| {
                debug!("{:?}", vc);
                if votes.contains(vc) { Some(n) }
                else { None }
            }).collect()
    };

    // Reveal Committed Summands
    let committed_summands_revealed_path = {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(&datadir_path);
        pathbuf.push("committed_summands_revealed");
        pathbuf.set_extension("csv");
        pathbuf.into_boxed_path()
    };
    let committed_summands = CommittedSummands::from_csprng(
        poll_secrets.summands_root,
        pollconf.num_ballots,
        pollconf.voter_roster_size);
    let mut csvwriter = csv::Writer::from_path(committed_summands_revealed_path)?;
    Summands::from(committed_summands).records.iter().for_each(|summand| {
        csvwriter.serialize(summand).unwrap();
    });
    // Reveal Committed Summands Key
    let committed_summands_key_path = {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(&datadir_path);
        pathbuf.push("committed_summands_key");
        pathbuf.set_extension("key");
        pathbuf.into_boxed_path()
    };
    File::create(committed_summands_key_path)?
        .write(base64::encode(&poll_secrets.summands_key.0).as_bytes())?;

    // Reveal Audited Columns
    let audited_columns_seed: Vec<u8> = {
        let seed = pollconf.audited_columns_seed.clone();
        hex::decode(seed.unwrap())?
    };
    let audited_columns: Vec<usize> = {
        let seed = CSPRNGSeed::from_vec(&audited_columns_seed);
        let mut prng = CSPRNG::from_csprng_seed(seed);
        (0..NUMBER_OF_PLANES).into_iter().map(|_| prng.gen_range(0, 2)).collect()
    };

    // Post the Fully Audited Column Planes.
    let column_planes: Vec<Plane> = generate_column_planes(
        &poll_secrets,
        NUMBER_OF_PLANES,
        2 * pollconf.num_ballots,
        pollconf.num_decoys)?;
    // Filter planes.
    column_planes.iter().enumerate()
        .for_each(|(n, plane)| {
            let posted_planes_path = {
                let mut pathbuf = PathBuf::new();
                pathbuf.push(&datadir_path);
                pathbuf.push(format!("final_plane_{:02}", n+1));
                pathbuf.set_extension("csv");
                pathbuf.into_boxed_path()
            };
            let posted_keys_path = {
                let mut pathbuf = PathBuf::new();
                pathbuf.push(&datadir_path);
                pathbuf.push(format!("final_plane_{:02}_keys", n+1));
                pathbuf.set_extension("csv");
                pathbuf.into_boxed_path()
            };
            let psecrets = poll_secrets.plane_secrets[n].resolve(plane.len());
            let filter = PlaneFilter::from(&psecrets.col1_keys, &psecrets.col3_keys)
                .decrypt_serials(&audited_ballots)
                .decrypt_column(if audited_columns[n] == 0 {1} else {3});

            let permuted_plane = plane.mark_rows(&marked_rows).decrypt(&filter).permute(&psecrets.permutation);
            let mut csvwriter = csv::Writer::from_path(posted_planes_path).unwrap();
            permuted_plane.rows.iter()
                .for_each(|rec| {
                    csvwriter.serialize(rec.serializable(pollconf.num_ballots)).unwrap();
                });
            
            let permuted_filter = filter.permute(&psecrets.permutation);
            let mut csvwriter = csv::Writer::from_path(posted_keys_path).unwrap();
            permuted_filter.serializable().iter()
                .for_each(|rec| {
                    csvwriter.serialize(rec).unwrap();
                });
        });

    // Update the poll state.
    pollconf.poll_state.roster_revealed = true;
    pollconf.poll_state.summands_revealed = true;
    pollconf.poll_state.columns_revealed = true;
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

