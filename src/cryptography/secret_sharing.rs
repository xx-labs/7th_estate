//! # Secret Sharing Functionality
//!
//! `secret_sharing` contains functions related to sharing and
//! reconstructing a secret.
//!
//! The specific secret sharing scheme employed is Shamir Secret
//! Sharing using a prime arithmetic field. Since the smallest
//! prime arithmetic field that can contain an 8-bit byte requires
//! 2 bytes (Z-257), we share over the larger 15-bit Z-32749.

use threshold_secret_sharing as tss;
use super::endian;
use super::Result;

#[derive(Debug, Clone)]
pub struct Secret(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct SecretShare(pub Vec<u16>);

impl SecretShare {
    pub fn from_vec8(v8: Vec<u8>) -> Self {
        SecretShare(endian::le_bytes::to_slice_u16(&v8).to_vec())
    }
        
    pub fn to_vec8(self: &Self) -> Vec<u8> {
        endian::le_bytes::from_slice_u16(&self.0).to_vec()
    }
}


pub struct ShamirSecretSharing {
    threshold: usize,
    share_count: usize
}

impl ShamirSecretSharing {
    //const PRIME: i64 = 7770492749; // Smallest 33-bit prime
    const PRIME: i64 = 32749; // Smallest 15-bit prime
    //const PRIME: i64 = 257; // Smallest 9-bit prime

    pub fn new() -> Self {
        ShamirSecretSharing {
            threshold: 0,
            share_count: 0
        }
    }

    pub fn with_share_count(mut self, share_count: usize) -> Self {
        self.share_count = share_count; self
    }

    pub fn with_threshold(mut self, threshold: usize) -> Self {
        self.threshold = threshold; self
    }

    pub fn with_majority_threshold(self) -> Self {
        assert!(0 < self.share_count);
        let threshold = (self.share_count / 2) + (1 - (self.share_count % 2));
        self.with_threshold(threshold)
        /*
        self.threshold = (self.share_count / 2) + (1 - (self.share_count % 2));
        self
        */

    }
    
    pub fn share(&self, secret: &Secret) -> Vec<SecretShare> {
        let tss = tss::shamir::ShamirSecretSharing {
            threshold: self.threshold,
            share_count: self.share_count,
            prime: Self::PRIME
        };

        // Share will give us a list of shares per secret byte.
        let transposed_shares: Vec<Vec<u16>> = secret.0.iter()
            .map(|&secret_byte| {
                tss.share(secret_byte as i64)
            }).map(|secret_vector| {
                secret_vector.iter().map(|&x| (x as u16)).collect::<Vec<u16>>()
            }).collect();

        // Transpose the previous operation to get a list of secret shares.
        let shares: Vec<SecretShare> = (0..self.share_count).into_iter()
            .map(|n| {
                let mut share = SecretShare(
                    transposed_shares.iter()
                        .map(|v| { v[n] })
                        .collect::<Vec<u16>>());
                share.0.insert(0, n as u16); share
            }).collect();
        shares
    }
    
    pub fn reconstruct(&self, shares: &[SecretShare]) -> Result<Secret> {
        let tss = tss::shamir::ShamirSecretSharing {
            threshold: self.threshold,
            share_count: self.share_count,
            prime: Self::PRIME
        };
        
        //println!("{:#?}", shares);
        let length_of_secret: usize = shares[0].0.len() - 1;

        // Transpose the shares to get a list of shares per secret byte.
        let indices: Vec<usize> = shares.iter().map(|v| v.0[0] as usize).collect();
        let tss_shares: Vec<Vec<i64>> = (1..length_of_secret+1).into_iter()
            .map(|n| shares.iter().map(|v| v.0[n] as i64).collect())
            .collect();

        // Reconstruct the secret.
        let secret: Secret = Secret(
            tss_shares.iter().map(|byte_shares| {
                tss.reconstruct(indices.as_slice(), byte_shares.as_slice())
            }).map(|secret_byte| { secret_byte as u8 })
            .collect()
        );
        Ok(secret)
    }
}

