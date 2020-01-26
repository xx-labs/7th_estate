//! # Column 1 Entries

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialVoteCode {
    pub serial: BallotSerial,
    pub votecode: VoteCode
}

#[derive(Debug, Clone, Serialize)]
pub enum Column1Entry {
    //Entry(SerialVoteCode),
    Entry(String),
    Encrypted(AEADString)
}

impl Column1Entry {
    pub fn decrypt(self: &Self, filter: &PlaneFilterEntry) -> Self {
        match self {
            Self::Encrypted(aestr) => {
                match filter.decrypt {
                    true => {
                        let aevalues = aestr.values().unwrap();
                        let serialized_bytes = aead_decrypt(&filter.key, &aevalues).unwrap();
                        let serialized = String::from_utf8(serialized_bytes).unwrap();
                        let entry = serialized;
                        Self::Entry(entry)
                    },
                    false => (*self).clone()
                }
            },
            _ => (*self).clone()
        }
    }

    pub fn encrypt(self: &Self, key: &AEADKey, nonce: &AEADNonce) -> Option<Self> {
        match self {
            Self::Entry(value) => {
                let serialized = value.as_bytes().to_vec();
                let aad = base64::encode(&nonce.0)
                    .as_bytes().to_vec();
                let aead_values = aead_encrypt_ex(&key, &nonce, aad, serialized).unwrap();
                Some(Self::Encrypted(AEADString::from_values(aead_values)))
            }
            Self::Encrypted(_) => None,
        }
    }
}

