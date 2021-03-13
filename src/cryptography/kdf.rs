//! # Key-Derivation Function (KDF)
//!
//! `kdf` contains functions related to deriving keys from passwords.
//! The specific KDF currently employed is scrypt(N=20, r=8, p=1).
//! The randomly generated salt is 256 bits.

use serde::{Serialize, Deserialize};
use super::{Result, debug};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KDFString(pub String);

#[derive(Debug, Clone)]
pub struct KDFValues {
    pub salt: Vec<u8>
}
/*
impl KDFString {
    pub fn from_values(values: KDFValues) -> KDFString {
        let salt64 = base64::encode(&values.salt);
        let string = format!("$scrypt${}$", salt64);
        KDFString(string)
    }

    pub fn values(self: &Self) -> Result<KDFValues> {
        // "$scrypt$salt$"
        let components: Vec<&str> = self.0.split("$").collect();
        let salt = base64::decode(components[2])?;
        Ok(KDFValues { salt: salt })
    }
}
*/

/// Derive a key from a password using randomly generated KDF inputs.
pub fn kdf(password: &str) -> Result<(Vec<u8>, KDFValues)> {
    let mut salt = [0u8; 32];
    getrandom::getrandom(&mut salt)?;
    let params = KDFValues { salt: salt.to_vec() };
    let output = kdf_with_params(password, &params)?;
    Ok((output, params))
}

/// Derive a key from a password using prior generated KDF inputs.
/// Only use this function for re-generation of a key or
/// verification of a password.
///
/// # Examples
///
/// ```
/// let password = "password";
/// let (key, params) = kdf(password).unwrap();
/// let verified_key = kdf_with_params(password, &params).unwrap();
/// assert_eq!(key, verified_key);
/// ```
pub fn kdf_with_params(password: &str, parameters: &KDFValues) -> Result<Vec<u8>> {
    //let params = scrypt::ScryptParams::new(20, 8, 1)?;
    let params = scrypt::ScryptParams::new(4, 3, 1)?;
    let password_bytes = password.as_bytes();
    let mut output = [0u8; 32];

    scrypt::scrypt(password_bytes, &parameters.salt, &params, &mut output)?;
    debug!("KDF Result: {}", hex::encode(output));
    Ok(output.to_vec())
}

