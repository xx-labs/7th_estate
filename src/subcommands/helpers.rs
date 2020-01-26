//! Helper functions for the `subcommands` module.
//!
//! These functions perform tasks repeated across the various subcommands.

use super::*;


pub fn read_poll_configuration_file(filename: &str) -> Result<SecuredPollConfiguration> {
    let secured_poll_configuration: SecuredPollConfiguration = {
        serde_yaml::from_reader(
            File::open(
                Path::new(filename))?)?
    };
    Ok(secured_poll_configuration)
}


/// Reconstruct the Poll Master Key from the trustee passwords.
pub fn read_poll_master_key(secured_poll_configuration: &SecuredPollConfiguration) -> (PollMasterKey, AEADKey)  {
    let poll_master_key = {
        let num_trustees = secured_poll_configuration.poll_trustees.len();
        let master_key_shares = secured_poll_configuration.poll_trustees.iter()
            .map(|trustee| SecureTrusteeShare {
                identifier: trustee.identifier.clone(),
                secure_value: trustee.share.clone()
            }).filter_map(|secure_share| secure_share.read())
            .map(|share| PollMasterKeyShare(share.value))
            .collect::<ListOfPollMasterKeyShares>();
        PollMasterKey::reconstruct(master_key_shares, num_trustees)
    };
    let aead_pmk = AEADKey::from(poll_master_key.0.clone());
    (poll_master_key, aead_pmk)
}


/// Ensure the data directory for the poll exists.
pub fn ensure_poll_data_directory_exists(secured_poll_configuration: &SecuredPollConfiguration, aead_pmk: &AEADKey) -> Result<String> {
    // Attempt to create the data directory.
    // Note: Rust does not currently support securing directories on creation.
    let identifier = {
        let aead_values = secured_poll_configuration.poll_identifier.values()?;
        aead_decrypt(&aead_pmk, &aead_values)?;
        String::from_utf8(aead_values.aad)?
    };
    let datadir_path = Path::new(&identifier);
    debug!("{}", identifier);
    match DirBuilder::new().create(datadir_path) {
        Ok(_) => (),
        Err(err) => {
            match err.kind() {
                ErrorKind::AlreadyExists => (),
                _ => return Err(Box::new(err))
            }
        }
    }
    Ok(identifier)
}

/// Generate the column planes using the poll secrets.
pub fn generate_column_planes(secrets: &PollSecrets, num_planes: usize, num_rows: usize, num_decoys: usize) -> Result<Vec<Plane>> {
    fn generate_column_plane(secrets: &PollSecrets, plane_num: usize, votecodes: Vec<VoteCode>, decoys: Vec<BallotSerial>) -> Result<Plane> {
        let num_ballots: usize = votecodes.len() / 2;
        let psecrets = secrets.plane_secrets[plane_num].resolve(votecodes.len());
        // Column 1
        let col1: Vec<Column1Entry> = {
            let unencrypted: Vec<Column1Entry> = votecodes.iter().enumerate()
                .map(|(n, &vc)| {
                    Column1Entry::Entry(format!("{}: {}",
                        string_from_ballotserial(&(n / 2), num_ballots),
                        string_from_votecode(&vc)
                    ))
                    /*
                    Column1Entry::Entry(SerialVoteCode {
                        serial: n / 2,
                        votecode: vc
                    })
                    */
                }).collect();
            unencrypted.iter().zip(psecrets.col1_keys.iter()).zip(psecrets.col1_nonce.iter())
                .map(|((entry, key), nonce)| {
                    entry.encrypt(key, nonce).unwrap()
                }).collect::<Vec<Column1Entry>>()
        };
        // Column 2
        let col2: Vec<Column2Entry> = votecodes.iter().map(|_| Column2Entry::Empty).collect();
        // Column 3
        let col3: Vec<Column3Entry> = {
            let unencrypted: Vec<Column3Entry> = votecodes.iter().enumerate().zip(CHOICE_VALUES.iter().cycle())
                .map(|((n, _), &cv)| {
                    let serial = n / 2;
                    let tagged_choice = match decoys.contains(&serial) {
                        true => TaggedChoiceValue::Decoy,
                        false => TaggedChoiceValue::from(cv)
                    };
                    Column3Entry::Entry(string_from_taggedchoicevalue_padded(&tagged_choice))
                    /*
                    Column3Entry::Entry(match decoys.contains(&serial) {
                        true => TaggedChoiceValue::Decoy,
                        false => TaggedChoiceValue::from(cv)
                    })
                    */
                }).collect();
            unencrypted.iter().zip(psecrets.col3_keys.iter()).zip(psecrets.col3_nonce.iter())
                .map(|((entry, key), nonce)| {
                    entry.encrypt(key, nonce).unwrap()
                }).collect::<Vec<Column3Entry>>()
        };
        
        Ok(Plane {
            rows: col1.into_iter().zip(col2.into_iter()).zip(col3.into_iter())
                .map(|((x, y), z)| PlaneRecord { col1: x, col2: y, col3: z })
                .collect::<Vec<PlaneRecord>>()
        })
    }

    let votecodes: Vec<VoteCode> = generate_votecodes(secrets.votecode_root, num_rows);
    let decoys: Vec<BallotSerial> = generate_decoy_serials(secrets.decoy_root, num_decoys, num_rows / 2);

    Ok((0..num_planes).into_iter()
        .map(|n| {
            generate_column_plane(secrets, n, votecodes.clone(), decoys.clone()).unwrap()
        }).collect::<Vec<Plane>>())
}

