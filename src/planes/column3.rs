//! # Column 3 Entries

use super::*;

#[derive(Debug, Clone, Serialize)]
pub enum Column3Entry {
    Entry(String),//TaggedChoiceValue),
    Encrypted(AEADString)
}

impl Column3Entry {
    pub fn decrypt(self: &Self, filter: &PlaneFilterEntry) -> Self {
        match self {
            Self::Encrypted(aestr) => {
                match filter.decrypt {
                    true => {
                        let aevalues = aestr.values().unwrap();
                        let serialized_bytes = aead_decrypt(&filter.key, &aevalues).unwrap();
                        let serialized = String::from_utf8(serialized_bytes).unwrap();
                        let entry = serde_yaml::from_str(&serialized).unwrap();
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
                //let serialized = string_from_taggedchoicevalue_padded(value)
                //    .as_bytes().to_vec();
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

