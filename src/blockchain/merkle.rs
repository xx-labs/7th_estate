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
    data: Vec<CryptoSHA3256Hash>
}

impl CryptoHashData {
    pub fn new() -> CryptoHashData {
        CryptoHashData {
            hasher: CryptoSha3Algorithm::new(),
            data: Vec::<CryptoSHA3256Hash>::new()
        }
    }
    pub fn push(&mut self, data: &[u8]) {
        self.hasher.write(data);
        self.data.push(self.hasher.hash());
        self.hasher.reset();
    }

    pub fn add(&mut self, array: Vec<u8>){
        self.push(&(*array)[..]);
    }

    pub fn add_vec(&mut self, array: Vec<Vec<u8>>) {
        array.into_iter()
            .for_each(|voter| {
                self.add(voter);
            })
    }

    pub fn complete(&mut self) {
        let data_size = self.data.len() as f64;
        let tree_size = (2 as i32).pow(data_size.log2().ceil() as u32) as usize;
        
        for _ in data_size as usize .. tree_size as usize {
            self.push(&[0]);
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
    fn hash(&mut self) -> CryptoSHA3256Hash {
        let mut h = [0u8; 32];
        self.0.result(&mut h);
        h
    }

    #[inline]
    fn reset(&mut self) {
        self.0.reset();
    }
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