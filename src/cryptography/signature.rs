//! # Digital Signature Functionality
//!
//! `signature` contains functions related to generating digital
//! signature key pairs, signing documents, and verifying
//! signatures.
//!
//! The specific digital signature scheme employed is Ed25519,
//! which is the Edwards-curve Digital Signature Algorithm (EdDSA)
//! using SHA-512 and Curve25519.

use signatory::ed25519;
use signatory::encoding::{Encode, Decode, Base64};
// use signatory::public_key::PublicKey;
use signatory::signature::{Signer};
use signatory_sodiumoxide::{Ed25519Signer};

use super::{Result, Base64String};

/// Generate a key pair for signing and signature verification.
pub fn new_signing_key() -> Result<(Base64String, Base64String)> {
    let seed = ed25519::Seed::generate();
    let signer = Ed25519Signer::from(&seed);
    let pk = signatory::ed25519::PublicKey::from(&signer);
    
    Ok((Base64String(seed.encode_to_string(&Base64::default()).unwrap()),
        Base64String(pk.encode_to_string(&Base64::default()).unwrap())))
}

/// Sign data using a provided signing key.
pub fn sign(key: &Base64String, data: Vec<u8>) -> Result<(Vec<u8>, Vec<u8>)> {
    let seed = ed25519::Seed::decode_from_str(&key.0, &Base64::default()).unwrap();
    let signer = Ed25519Signer::from(&seed);
    let signature = signer.sign(&data).to_bytes().to_vec();
    Ok((data, signature))
}

