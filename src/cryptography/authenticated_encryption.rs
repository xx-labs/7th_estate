//! # Authenticated Encryption Functionality
//!
//! `authenticated_encryption` contains functions related to
//! authenticating and verifying both encrypted and associated
//! data.
//!
//! The specific authenticated encryption scheme employed is
//! ChaCha20-Poly1305 using a 256-bit key.

use std::str;
use std::cmp::max;
use serde::{Serialize, Deserialize};
use super::{Result, debug};

#[derive(Debug, Clone, Copy)]
pub struct AEADKey(pub [u8; 32]);
#[derive(Debug, Clone, Copy)]
pub struct AEADNonce(pub [u8; 12]);
#[derive(Debug, Clone, Copy)]
pub struct AEADTag(pub [u8; 16]);
#[derive(Debug, Clone)]
pub struct EncryptedData(pub Vec<u8>);
#[derive(Debug, Clone)]
pub struct DecryptedData(pub Vec<u8>);

impl AEADKey {
    pub fn from(value: Vec<u8>) -> Self {
        let mut key = [0u8; 32];
        let copylen = max(key.len(), value.len());
        key.copy_from_slice(&value[0..copylen]);
        AEADKey(key)
    }
}

#[derive(Debug, Clone)]
pub struct AEADValues {
    pub nonce: Vec<u8>,
    pub aad: Vec<u8>,
    pub encrypted_value: Vec<u8>,
    pub tag: Vec<u8>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AEADString(pub String);

impl AEADString {
    pub fn from_values(values: AEADValues) -> AEADString {
        let nonce64 = base64::encode(&values.nonce);
        let aad64 = base64::encode(&values.aad);
        let encrypted64 = base64::encode(&values.encrypted_value);
        let tag64 = base64::encode(&values.tag);
        let string = format!("$chacha20_poly1305_aead${}${}${}${}$",
            nonce64, aad64, encrypted64, tag64);
        AEADString(string)
    }

    pub fn values(self: &Self) -> Result<AEADValues> {
        // "$chacha20_poly1305_aead$nonce$aad$encrypted$tag$"
        let components: Vec<&str> = self.0.split("$").collect();
        let nonce = base64::decode(components[2])?;
        let aad = base64::decode(components[3])?;
        let encrypted_value = base64::decode(components[4])?;
        let tag = base64::decode(components[5])?;
        Ok(AEADValues {
            nonce: nonce,
            aad: aad,
            encrypted_value: encrypted_value,
            tag: tag
        })
    }
}


/// Authenticate data using randomly generated AEAD inputs.
pub fn aead_authenticate(aead_key: &AEADKey, aad: Vec<u8>) -> Result<AEADValues> {
    let mut nonce: [u8; 12] = [0u8; 12];
    getrandom::getrandom(&mut nonce)?;
    aead_authenticate_ex(aead_key, &AEADNonce(nonce), aad)
}

/// Authenticate and encrypt data using randomly generated AEAD inputs.
pub fn aead_encrypt(aead_key: &AEADKey, aad: Vec<u8>, value: Vec<u8>) -> Result<AEADValues> {
    let mut nonce: [u8; 12] = [0u8; 12];
    getrandom::getrandom(&mut nonce)?;
    aead_encrypt_ex(aead_key, &AEADNonce(nonce), aad, value)
}

/// Authenticate data using previously generated AEAD inputs.
///
/// This function is private since it should never need to be used.
fn aead_authenticate_ex(aead_key: &AEADKey, aead_nonce: &AEADNonce, aad: Vec<u8>) -> Result<AEADValues> {
    let value: Vec<u8> = Vec::new();
    aead_encrypt_ex(aead_key, aead_nonce, aad, value)
}


/// Authenticate and encrypt data using previously generated AEAD inputs.
///
/// This function should never need to be used.
pub fn aead_encrypt_ex(aead_key: &AEADKey, aead_nonce: &AEADNonce, aad: Vec<u8>, value: Vec<u8>) -> Result<AEADValues> {
    let key: [u8; 32] = aead_key.0;
    let nonce: [u8; 12] = aead_nonce.0;
    let mut encrypted = Vec::with_capacity(value.len());
    let tag = chacha20_poly1305_aead::encrypt(
        &key,
        &nonce,
        &aad,
        &value,
        &mut encrypted)?;

    debug!("Authenticated Data: {}", match String::from_utf8(aad.clone()) {
        Ok(s) => s,
        Err(_) => hex::encode(&aad)
    });
    debug!("Unencrypted Data:   {}", match String::from_utf8(value.clone()) {
        Ok(s) => s,
        Err(_) => hex::encode(&value)
    });
    debug!("Encrypted Data:     {}", hex::encode(&encrypted));
    debug!("Tag:                {}", hex::encode(&tag));
    
    Ok(AEADValues {
        nonce: nonce.to_vec(),
        aad: aad.clone(),
        encrypted_value: encrypted,
        tag: tag.to_vec()
    })
}

/// Verify and decrypt data secured by the AEAD scheme.
///
/// # Examples
///
/// ```
/// ```
pub fn aead_decrypt(aead_key: &AEADKey, aead_values: &AEADValues) -> Result<Vec<u8>> {
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 12];
    let aad: Vec<u8> = aead_values.aad.clone();
    let encrypted_value: Vec<u8> = aead_values.encrypted_value.clone();
    let mut tag = [0u8; 16];
    let mut decrypted = Vec::with_capacity(encrypted_value.len());

    let key_len = key.len();
    let nonce_len = nonce.len();
    let tag_len = tag.len();
    key.copy_from_slice(&aead_key.0[..key_len]);
    nonce.copy_from_slice(&aead_values.nonce[..nonce_len]);
    tag.copy_from_slice(&aead_values.tag[..tag_len]);
    
    chacha20_poly1305_aead::decrypt(
        &key,
        &nonce,
        &aad,
        &encrypted_value,
        &tag,
        &mut decrypted)?;

    debug!("Encrypted Data: {}", hex::encode(&encrypted_value));
    debug!("Tag:            {}", hex::encode(tag));
    debug!("Decrypted Data: {}", match String::from_utf8(decrypted.clone()) {
        Ok(s) => s,
        Err(_) => hex::encode(&decrypted)
    });

    Ok(decrypted.clone())
}


