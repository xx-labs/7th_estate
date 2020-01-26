//! # Persistent Secure Poll Configuration File
//!
//! The structure of the secured configuration has the following properties:
//! * The poll identifier is visible, but integrity-protected.
//! * The poll trustees and KDF parameters are visible, but integrity-protected.
//! * The poll trustee shares are confidential via password, butintegrity-protected.
//! * The poll configuration is confidential and integrity-protected.
//! * The poll public key is visible, but integrity-protected.
//!
//! The construction works by using the poll trustee KDF parameters to derive
//! individual secret keys that encrypt the shares. The decrypted shares are
//! assembled to reconstruct the Master Key. The Master Key enables verification
//! of the poll's identifier, trustees, configuration, and signature
//! verification key.
//!
//! Note that Master Key does not directly protect the integrity of the trustee
//! share information. By virtue of the fact that the shares can construct the
//! Master Key provides the integrity protection.

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuredPollConfiguration {
    pub poll_identifier: AEADString,
    pub poll_trustees: Vec<PollConfigurationTrustee>,
    pub encrypted_poll_configuration: AEADString,
    pub signing_certificate: AEADString
}

