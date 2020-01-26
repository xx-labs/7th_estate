//! Implementation file for derived poll secrets.

use rand::seq::SliceRandom;
use super::*;
use crate::NUMBER_OF_PLANES;

#[derive(Debug, Clone)]
pub struct DerivedPlaneSecrets {
    pub permutation: Vec<usize>,
    pub col1_keys: Vec<AEADKey>,
    pub col1_nonce: Vec<AEADNonce>,
    pub col3_keys: Vec<AEADKey>,
    pub col3_nonce: Vec<AEADNonce>
}

#[derive(Debug, Clone)]
pub struct PlaneSecrets {
    // Top-level Secrets
    pub plane_root: CSPRNGSeed,
    pub permutation_root: CSPRNGSeed,
    pub key_root: CSPRNGSeed,
    // Column-level Secrets
    pub col1_key_root: CSPRNGSeed,
    pub col1_nonce_root: CSPRNGSeed,
    pub col3_key_root: CSPRNGSeed,
    pub col3_nonce_root: CSPRNGSeed,
}

#[derive(Debug, Clone)]
pub struct PollSecrets {
    // Top-level Secrets
    pub votecode_root: CSPRNGSeed,
    pub decoy_root: CSPRNGSeed,
    pub summands_root: CSPRNGSeed,
    pub planes_root: CSPRNGSeed,
    pub summands_key: AEADKey,
    // Plane-level Secrets
    pub plane_secrets: Vec<PlaneSecrets>
}


impl PollSecrets {
    pub fn derive(pmk: &PollMasterKey) -> Self {
        assert!(pmk.0.len() == CSPRNGSeed::SIZE,
            "Poll Master Key not a valid seed length.");
        let pmk_seed = CSPRNGSeed::from_vec(&pmk.0);

        // Derivation Setup
        let mut secrets = PollSecrets::new();
        let mut pmkrng = CSPRNG::from_csprng_seed(pmk_seed);
        // Top-level Secrets
        secrets.votecode_root = CSPRNGSeed::next_seed(&mut pmkrng);
        secrets.decoy_root = CSPRNGSeed::next_seed(&mut pmkrng);
        secrets.summands_root = CSPRNGSeed::next_seed(&mut pmkrng);
        secrets.planes_root = CSPRNGSeed::next_seed(&mut pmkrng);
        pmkrng.fill_bytes(&mut secrets.summands_key.0);
        // Plane-level Secrets
        let mut planesrng = CSPRNG::from_csprng_seed(secrets.planes_root);
        secrets.plane_secrets = (0..NUMBER_OF_PLANES).into_iter()
            .map(|_| PlaneSecrets::derive(CSPRNGSeed::next_seed(&mut planesrng)))
            .collect();
        secrets
    }

    fn new() -> Self {
        PollSecrets {
            votecode_root: CSPRNGSeed::DEFAULT,
            decoy_root: CSPRNGSeed::DEFAULT,
            summands_root: CSPRNGSeed::DEFAULT,
            planes_root: CSPRNGSeed::DEFAULT,
            summands_key: AEADKey(Default::default()),
            plane_secrets: Vec::new()
        }
    }
}

impl PlaneSecrets {
    pub fn derive(prk: CSPRNGSeed) -> Self {
        let mut secrets = PlaneSecrets::new();
        secrets.plane_root = prk;
        // Top-level Secrets
        let mut prkrng = CSPRNG::from_csprng_seed(secrets.plane_root);
        secrets.permutation_root = CSPRNGSeed::next_seed(&mut prkrng);
        secrets.key_root = CSPRNGSeed::next_seed(&mut prkrng);
        // Column-level Secrets
        let mut keyrng = CSPRNG::from_csprng_seed(secrets.key_root);
        secrets.col1_key_root = CSPRNGSeed::next_seed(&mut keyrng);
        secrets.col1_nonce_root = CSPRNGSeed::next_seed(&mut keyrng);
        secrets.col3_key_root = CSPRNGSeed::next_seed(&mut keyrng);
        secrets.col3_nonce_root = CSPRNGSeed::next_seed(&mut keyrng);
        secrets
    }

    pub fn resolve(self: &Self, num_rows: usize) -> DerivedPlaneSecrets {
        let mut permutation_csprng = CSPRNG::from_csprng_seed(self.permutation_root);
        let mut col1_key_csprng = CSPRNG::from_csprng_seed(self.col1_key_root);
        let mut col1_nonce_csprng = CSPRNG::from_csprng_seed(self.col1_nonce_root);
        let mut col3_key_csprng = CSPRNG::from_csprng_seed(self.col3_key_root);
        let mut col3_nonce_csprng = CSPRNG::from_csprng_seed(self.col3_nonce_root);

        let mut permutation: Vec<usize> = (0..num_rows).collect();
        permutation.shuffle(&mut permutation_csprng);

        DerivedPlaneSecrets {
            permutation: permutation,
            col1_keys: (0..num_rows).into_iter()
                .map(|_| {
                    let mut key: AEADKey = AEADKey([0; 32]);
                    col1_key_csprng.fill_bytes(&mut key.0);
                    key
                }).collect::<Vec<AEADKey>>(),
            col1_nonce: (0..num_rows).into_iter()
                .map(|_| {
                    let mut nonce: AEADNonce = AEADNonce([0; 12]);
                    col1_nonce_csprng.fill_bytes(&mut nonce.0);
                    nonce
                }).collect::<Vec<AEADNonce>>(),
            col3_keys: (0..num_rows).into_iter()
                .map(|_| {
                    let mut key: AEADKey = AEADKey([0; 32]);
                    col3_key_csprng.fill_bytes(&mut key.0);
                    key
                }).collect::<Vec<AEADKey>>(),
            col3_nonce: (0..num_rows).into_iter()
                .map(|_| {
                    let mut nonce: AEADNonce = AEADNonce([0; 12]);
                    col3_nonce_csprng.fill_bytes(&mut nonce.0);
                    nonce
                }).collect::<Vec<AEADNonce>>()
        }
    }

    fn new() -> Self {
         PlaneSecrets {
            plane_root: CSPRNGSeed::DEFAULT,
            permutation_root: CSPRNGSeed::DEFAULT,
            key_root: CSPRNGSeed::DEFAULT,
            col1_key_root: CSPRNGSeed::DEFAULT,
            col1_nonce_root: CSPRNGSeed::DEFAULT,
            col3_key_root: CSPRNGSeed::DEFAULT,
            col3_nonce_root: CSPRNGSeed::DEFAULT,
        }
    }
}
