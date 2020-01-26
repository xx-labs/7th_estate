//! # Trustee Shares
//!
//! `trustee_shares` contains the specific logic and functions for:
//! * dividing the Poll Master Key into shares;
//! * encrypting the shares;
//! * securing the integrity of the shares.
//!
//! The mechanism for protecting the share is AEAD.
//! Using AEAD, the trustee identity and key-derivation
//! parameters are bound to the encrypted share.
//!
//! The purpose of the AEAD scheme is to provide share encryption and
//! to provide evidence of tampering of the trustee identity binding
//! and the key-derivation parameters. It provides no protection against
//! denial-of-service.

use std::str;
use super::*;


/// Trustees Sharing implementation.
#[derive(Debug)]
pub struct TrusteeShare {
    pub identifier: String,
    pub value: Vec<u8>
}

impl TrusteeShare {
    pub fn new(identifier: String, value: Vec<u8>) -> Self {
        TrusteeShare {
            identifier: identifier,
            value: value
        }
    }

    pub fn secure(self: &Self) -> SecureTrusteeShare {
        let password = read_trustee_password(&self.identifier);
        let secure_value = encrypt_trustee_share(&password, &self.identifier, self.value.clone()).unwrap();
        SecureTrusteeShare {
            identifier: self.identifier.clone(),
            secure_value: secure_value
        }
    }
}


/// Trustee Encrypted Shares implementation.
pub struct SecureTrusteeShare {
    pub identifier: String,
    pub secure_value: AEADString
}

impl SecureTrusteeShare {
    pub fn new(identifier: String, secure_value: AEADString) -> Self {
        SecureTrusteeShare {
            identifier: identifier,
            secure_value: secure_value
        }
    }

    pub fn read(self: &Self) -> Option<TrusteeShare> {
        let password = read_trustee_password(&self.identifier);
        let value = maybe_decrypt_trustee_share(&password, &self.identifier, self.secure_value.clone());
        match value {
            Some(v) => Some(TrusteeShare {
                identifier: self.identifier.clone(),
                value: v
            }),
            None => None
        }
    }
}


///
/// Trustee utility functions.
///

/// Function to read a trustee's password from the terminal.
fn read_trustee_password_terminal(trustee: &str) -> String {
    let initial_prompt = format!("Password for \"{}\": ", trustee);
    let confirm_prompt = format!("Confirm Password: ");
    let mut passwords_match = false;
    let mut password: String = "".to_owned();

    while !passwords_match {
        let initial_password = rpassword::read_password_from_tty(Some(&initial_prompt)).unwrap();
        let confirm_password = rpassword::read_password_from_tty(Some(&confirm_prompt)).unwrap();
        passwords_match = initial_password == confirm_password;
        if passwords_match {
            password = initial_password.to_owned();
        } else {
            error!("Provided passwords did not match.");
        }
    };
    password
}

/// Function to abstract the functionality of reading a trustee's password.
pub fn read_trustee_password(trustee: &str) -> String {
    read_trustee_password_terminal(trustee)
}


/// Encrypt trustee share data and protect it via password.
///
/// # Examples
///
/// ```
/// let share: Vec<u8> = vec![1, 2, 3, 4];
/// let encrypted_share: AEADString = encrypt_trustee_share("password", "trustee", share).unwrap();
/// let decrypted_share: Vec<u8> = decrypt_trustee_share("password", "trustee", encrypted_share).unwrap();
/// assert_eq!(share, decrypted_share);
/// ```
pub fn encrypt_trustee_share(password: &str, identity: &str, share: Vec<u8>) -> Result<AEADString> {
    let (key, params) = kdf(password)?;
    let salt64 = base64::encode(&params.salt);
    let identity_string = identity.to_owned() + "-" + &salt64;
    Ok(AEADString::from_values(
        aead_encrypt(&AEADKey::from(key),
                     identity_string.as_bytes().to_vec(),
                     share)?))
}

/// Decrypt trustee share data protected via password.
///
/// # Examples
///
/// ```
/// let share: Vec<u8> = vec![1, 2, 3, 4];
/// let encrypted_share: AEADString = AEADString("".to_owned());
/// let decrypted_share: Vec<u8> = decrypt_trust_share("password", "trustee", encrypted_share).unwrap();
/// assert_eq!(share, decrypted_share);
/// ```
pub fn decrypt_trustee_share(password: &str, identity: &str, encrypted_share: AEADString) -> Result<Vec<u8>> {
    let values: AEADValues = encrypted_share.values()?;
    let aad_values: Vec<&str> = str::from_utf8(&values.aad)?.split("-").collect();
    let salt64 = aad_values[1];
    let salt = base64::decode(&salt64)?;
    let params = KDFValues { salt: salt };
    let key = kdf_with_params(password, &params)?;
    assert!(identity.as_bytes().to_vec() == aad_values[0].as_bytes().to_vec(),
        "Detected poll configuration tampering. Trustee identity does not match the authenticated share data.");
    aead_decrypt(&AEADKey::from(key), &values)
}

/// Maybe decrypt trustee share data protected via password.
///
/// Adapter function to yield an Option instead of a Result.
pub fn maybe_decrypt_trustee_share(password: &str, identity: &str, encrypted_share: AEADString) -> Option<Vec<u8>> {
    match decrypt_trustee_share(password, identity, encrypted_share) {
        Ok(v) => {
            Some(v)
        },
        Err(_) => None
    }
}

