//! # Command: Sign
//!
//! `sign` generates a signature using the poll signing key.

use std::fs;
use super::*;

pub fn sign_document(pollconf_filename: &str, document_filename: &str) -> Result<()> {
    let document_path = Path::new(document_filename);

    // Read poll configuration file.
    let mut secured_poll_configuration = read_poll_configuration_file(pollconf_filename)?;

    // Reconstruct the Poll Master Key from the trustee passwords.
    let (poll_master_key, aead_pmk) = read_poll_master_key(&secured_poll_configuration);

    // Decrypt poll configuration state.
    let pollconf_aead_values = secured_poll_configuration.encrypted_poll_configuration.values()?;
    let serialized_pollconf = aead_decrypt(&aead_pmk, &pollconf_aead_values)?;
    let pollconf: PollConfiguration = serde_yaml::from_slice(&serialized_pollconf).unwrap();

    let document: Vec<u8> = fs::read(&document_path)?;
    let (_, signature) = sign(&pollconf.signing_key, document)?;
    let document_signature_path_str = document_filename.to_owned() + ".sig";
    let document_signature_path = Path::new(&document_signature_path_str);
    fs::write(&document_signature_path, base64::encode(&signature))?;

    Ok(())
}
