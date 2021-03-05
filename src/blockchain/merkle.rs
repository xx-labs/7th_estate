use merkletree::merkle::MerkleTree;
use merkletree::store::VecStore;
use merkletree::proof::Proof;
use crate::Result;
use crypto::digest::Digest;
use crypto::sha3::{Sha3, Sha3Mode};
use merkletree::hash::Algorithm;
use std::hash::Hasher;

use typenum::U0;

pub type MerkleRoot = MerkleTree<CryptoSHA3256Hash, CryptoSha3Algorithm, VecStore<CryptoSHA3256Hash>>;
pub type CryptoSHA3256Hash = [u8; 32];
pub struct CryptoSha3Algorithm(Sha3);

pub struct CryptoHashData{
    hasher: CryptoSha3Algorithm,
    data: Vec<String>
}

impl CryptoHashData {
    pub fn new(newdata: Vec<String>) -> CryptoHashData {
        CryptoHashData {
            hasher: CryptoSha3Algorithm::new(),
            data: newdata
        }
    }
    pub fn push(&mut self, data: String) {
        self.data.push(data);
    }
    pub fn push_vec(&mut self, data: Vec<String>) {
        for d in data.into_iter() {
            self.push(d);
        }
    }

    pub fn pad(&mut self){
        let size = self.data.len();
        for _ in size .. size.next_power_of_two() {
            self.data.push(String::from("\0"));
        }
    }
}

impl CryptoSha3Algorithm {
    pub fn new() -> CryptoSha3Algorithm {
        CryptoSha3Algorithm(Sha3::new(Sha3Mode::Sha3_256))
    }
}

impl Default for CryptoSha3Algorithm {
    fn default() -> CryptoSha3Algorithm {
        CryptoSha3Algorithm::new()
    }
}

impl Hasher for CryptoSha3Algorithm {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.input(msg)
    }

    #[inline]
    fn finish(&self) -> u64 {
        unimplemented!()
    }
}

impl Algorithm<CryptoSHA3256Hash> for CryptoSha3Algorithm {
    #[inline]
    fn hash(&mut self) -> [u8; 32] {
        let mut h = [0u8; 32];
        self.0.result(&mut h);
        h
    }

    #[inline]
    fn reset(&mut self) {
        self.0.reset();
    }
}

pub fn pad_to_power_2(mut v: Vec<String>) -> Vec<String> {
    let size = v.len();
    for _ in size .. size.next_power_of_two() {
        v.push(String::from("\0"));
    }
    v
}

pub fn new_tree(hashed: CryptoHashData) -> Result<MerkleRoot> {
    Ok(MerkleTree::from_data(hashed.data)? as MerkleRoot)
}

pub fn validate(t: MerkleRoot, proof_item: CryptoSHA3256Hash) -> Result<bool> {
    let generated_proof = t.gen_proof(0).unwrap();
    let proof: Proof<CryptoSHA3256Hash> = Proof::new::<U0, U0>(
        None,
        generated_proof.lemma().to_owned(),
        generated_proof.path().to_owned(),
    )
    .unwrap();
    Ok(proof.validate_with_data::<CryptoSha3Algorithm>(&proof_item).unwrap())
}