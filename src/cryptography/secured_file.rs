//! # Secured File Functionality
//!
//! `secured_file` contains functions related to reading and
//! writing files with data assurance and data privacy.
//!
//! This is simply a file-based version of the AEAD scheme.
//! The user of this module must supply their own key.

use std::str;
use super::*;

pub type SecuredFileKey = AEADKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuredFile {
    assured: String,
    protected: String,
    security: String
}

impl SecuredFile {
    pub fn new(key: &SecuredFileKey, assured: String, protected: String) -> Self {
        Self::maybe_secure(key, assured, protected).unwrap()
    }

    fn maybe_secure(key: &SecuredFileKey, assured: String, protected: String) -> Option<Self> {
        match Self::secure(key, assured, protected) {
            Ok(v) => Some(v),
            Err(_) => None
        }
    }

    fn secure(key: &SecuredFileKey, assured: String, protected: String) -> Result<Self> {
        let aead_values = aead_encrypt(key, assured.as_bytes().to_vec(), protected.as_bytes().to_vec())?;
        Ok(SecuredFile {
            assured: assured,
            protected: base64::encode(&aead_values.encrypted_value),
            security: format!("$chacha20_poly1305_aead${}${}$",
                base64::encode(&aead_values.nonce),
                base64::encode(&aead_values.tag))
        })
    }

    pub fn open(self: &Self, key: &SecuredFileKey) -> Result<(String, String)> {
        let components: Vec<&str> = self.security.split("$").collect();
        let nonce = base64::decode(components[2])?;
        let tag = base64::decode(components[3])?;
        let aead_values = AEADValues {
            nonce: nonce,
            aad: self.assured.as_bytes().to_vec(),
            encrypted_value: base64::decode(&self.protected)?,
            tag: tag
        };
        let protected: String = String::from_utf8(aead_decrypt(key, &aead_values)?)?;
        Ok((self.assured.clone(), protected))
    }
}

