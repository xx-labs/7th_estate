//! # Poll Master Key
//!
//! `poll_master_key` provides an abstraction layer
//! for generating, sharing, and reconstructing the
//! Poll Master Key.

use super::*;

pub struct PollMasterKey(pub Vec<u8>);

pub struct PollMasterKeyShare(pub Vec<u8>);

pub type ListOfPollMasterKeyShares = Vec<PollMasterKeyShare>;

impl PollMasterKey {
    pub fn new() -> Self {
        let mut pmk = [0u8;32];
        getrandom::getrandom(&mut pmk).unwrap();
        PollMasterKey(pmk.to_vec())
    }
    
    pub fn share(self: &Self, num_shares: usize) -> ListOfPollMasterKeyShares {
        let tss = ShamirSecretSharing::new()
            .with_share_count(num_shares)
            .with_majority_threshold();
        let shares = tss.share(&Secret(self.0.clone()))
            .iter()
            .map(|share| PollMasterKeyShare(share.to_vec8()))
            .collect();
        shares
    }

    pub fn reconstruct(shares: ListOfPollMasterKeyShares, total_shares: usize) -> Self {
        let tss = ShamirSecretSharing::new()
            .with_share_count(total_shares)
            .with_majority_threshold();
        let reconstructable_shares: Vec<SecretShare> = shares.iter()
            .map(|share| SecretShare::from_vec8(share.0.clone()))
            .collect();
        PollMasterKey(tss.reconstruct(&reconstructable_shares).unwrap().0.clone())
    }
}

