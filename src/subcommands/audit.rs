//! Blockchain audit
//! 
//! Count votes present in blockchain with address of poll

use super::*;
use crate::blockchain::audit_votes;

pub fn blockchain_audit(pollconf_filename: &str, xxn_filename: &str) -> Result <()> {
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

    // Generate the Ballots.
    let serials: Vec<BallotSerial> = (0..pollconf.num_ballots).collect();
    let votecodes: Vec<VoteCode> = generate_votecodes(
        poll_secrets.votecode_root,
        2 * pollconf.num_ballots);

    // Regenerate ballots
    let ballots = generate_ballots(&serials, &votecodes);
    let decoys = get_decoys(&pollconf, poll_master_key)?;
    audit_votes(ballots, pollconf, xxn_filename, decoys)
}

pub fn get_decoys(pollconf: &PollConfiguration, poll_master_key: PollMasterKey) -> Result<Vec<usize>>{
    // Derive the poll secrets.
    let poll_secrets: PollSecrets = PollSecrets::derive(&poll_master_key);

    // Post the Fully Audited Column Planes.
    let column_planes: Vec<Plane> = generate_column_planes(
        &poll_secrets,
        NUMBER_OF_PLANES,
        2 * pollconf.num_ballots,
        pollconf.num_decoys)?;

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

    // Get first plane
    let n = 0;
    let decoys: Vec<usize> = match column_planes.get(n) {
        Some(plane) => {
            // Decrypt col1 (serial) and col3 (For|Against|Decoy)
            let psecrets = poll_secrets.plane_secrets[n].resolve(plane.len());
            let filter = PlaneFilter::from(&psecrets.col1_keys, &psecrets.col3_keys)
                .decrypt_serials(&audited_ballots)
                .decrypt_column(1)
                .decrypt_column(3);
            
            // Get only Decoys
            let permuted_plane = plane.mark_rows(&marked_rows).decrypt(&filter).permute(&psecrets.permutation);
            let decoys: Vec<usize> = permuted_plane.rows.iter().filter_map (| row | match &row.col3 {
                Column3Entry::Entry(opt) => 
                    if opt == "Decoy" { Some(row.col1.clone()) }
                    else { None },
                _ => None,
            }) // Get Serial codes
            .filter_map (| row | match row {
                Column1Entry::Entry(entry) => { Some(entry) },
                _ => None
            }).map(| serial | {
                *&serial[..2].parse::<usize>().unwrap()
            }).collect();
            decoys
        },
        _ => Vec::new()
    };
    Ok(decoys)
}