//! # Command: Record Votes
//!
//! `record_votes` takes a votes file (list of vote codes) and records them
//! as part of the secured poll configuration.

use std::cmp::max;
use super::*;


#[derive(Debug, Clone, Deserialize)]
pub struct VoteRecordFileRow {
    votecode: String
}


impl VoteRecordFileRow {
    fn to_votecode(self: &Self) -> VoteCode {
        let mut votecode: VoteCode = [0; VOTE_CODE_LENGTH];
        let votecode_vec: Vec<u8> = self.votecode.replace("-", "").split("")
            .filter_map(|x| {
                if 0 < x.len() { Some(u8::from_str_radix(x, 10).unwrap()) }
                else { None }
            })
            .collect();
        let copylen: usize = max(votecode_vec.len(), VOTE_CODE_LENGTH);
        votecode.copy_from_slice(&votecode_vec[0..copylen]);
        votecode
    }
}


pub fn record_votes(pollconf_filename: &str, votes_file: &str, force: bool) -> Result<()> {
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
    
    assert!(pollconf.poll_state.ceremony_conducted,
        "Recording votes cannot take place prior to public audit.");
    assert!(!pollconf.poll_state.votes_committed || force,
        "Votes already committed. To re-commit, pass --force.");

    // Derive the poll secrets.
    let poll_secrets: PollSecrets = PollSecrets::derive(&poll_master_key);

    // Re-construct the audited ballots.
    let audited_ballots: Vec<BallotSerial> = {
        pollconf.audited_ballots.clone().unwrap().iter()
            .map(|serial| usize::from_str_radix(serial, 10).unwrap())
            .collect()
    };

    // Read the Votes file.
    let votes: Vec<VoteCode> = {
        let votes_path = Path::new(votes_file);
        let mut csvreader = csv::Reader::from_path(votes_path)?;
        let records = csvreader.deserialize::<VoteRecordFileRow>();
        records.map(|row| { row.unwrap().to_votecode() }).collect()
    };
    pollconf.votes = Some(votes.clone());
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

    // Post the Column Planes.
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
                pathbuf.push(format!("vote_plane_{:02}", n+1));
                pathbuf.set_extension("csv");
                pathbuf.into_boxed_path()
            };
            let posted_keys_path = {
                let mut pathbuf = PathBuf::new();
                pathbuf.push(&datadir_path);
                pathbuf.push(format!("vote_plane_{:02}_keys", n+1));
                pathbuf.set_extension("csv");
                pathbuf.into_boxed_path()
            };
            let psecrets = poll_secrets.plane_secrets[n].resolve(plane.len());
            let filter = PlaneFilter::from(&psecrets.col1_keys, &psecrets.col3_keys)
                .decrypt_serials(&audited_ballots);

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
    pollconf.poll_state.votes_committed = true;
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


pub fn record_votes_run (pollconf_filename: &str, votes_file: &str) -> Result<(SecuredPollConfiguration, PollMasterKey, AEADKey)> {
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
    
    assert!(pollconf.poll_state.ceremony_conducted,
        "Recording votes cannot take place prior to public audit.");
    assert!(!pollconf.poll_state.votes_committed,
        "Votes already committed. To re-commit.");

    // Derive the poll secrets.
    let poll_secrets: PollSecrets = PollSecrets::derive(&poll_master_key);

    // Re-construct the audited ballots.
    let audited_ballots: Vec<BallotSerial> = {
        pollconf.audited_ballots.clone().unwrap().iter()
            .map(|serial| usize::from_str_radix(serial, 10).unwrap())
            .collect()
    };

    // Read the Votes file.
    let votes: Vec<VoteCode> = {
        let votes_path = Path::new(votes_file);
        let mut csvreader = csv::Reader::from_path(votes_path)?;
        let records = csvreader.deserialize::<VoteRecordFileRow>();
        records.map(|row| { row.unwrap().to_votecode() }).collect()
    };
    pollconf.votes = Some(votes.clone());
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

    // Post the Column Planes.
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
                pathbuf.push(format!("vote_plane_{:02}", n+1));
                pathbuf.set_extension("csv");
                pathbuf.into_boxed_path()
            };
            let posted_keys_path = {
                let mut pathbuf = PathBuf::new();
                pathbuf.push(&datadir_path);
                pathbuf.push(format!("vote_plane_{:02}_keys", n+1));
                pathbuf.set_extension("csv");
                pathbuf.into_boxed_path()
            };
            let psecrets = poll_secrets.plane_secrets[n].resolve(plane.len());
            let filter = PlaneFilter::from(&psecrets.col1_keys, &psecrets.col3_keys)
                .decrypt_serials(&audited_ballots);

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
    pollconf.poll_state.votes_committed = true;
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

    Ok((secured_poll_configuration, poll_master_key, aead_pmk))
}