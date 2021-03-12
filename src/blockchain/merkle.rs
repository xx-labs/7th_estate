
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
pub struct CryptoHashData(Vec<String>);


pub fn slice_as_hash(xs: &[u8]) -> &[u8; 32] {
    slice_as_array!(xs, [u8; 32]).expect("bad hash length")
}

impl CryptoHashData {
    pub fn new(newdata: Vec<String>) -> CryptoHashData {
        CryptoHashData(newdata)
    }
    pub fn push(&mut self, data: String) {
        self.0.push(data);
    }
    pub fn push_vec(&mut self, data: Vec<String>) {
        for d in data.into_iter() {
            self.push(d);
        }
    }

    pub fn pad(&mut self){
        let size = self.0.len();
        for _ in size .. size.next_power_of_two() {
            self.0.push(String::from("\0"));
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

fn get_hash (a: &mut CryptoSha3Algorithm, v: &String) -> [u8; 32] {
    a.reset();
    a.write(v.as_bytes());
    let b = a.hash();

    a.reset();
    a.write(&[0x00]);
    a.write(&b);
    let b = a.hash();
    b
}

fn get_leaf_index(t: &MerkleRoot, hash: CryptoSHA3256Hash) -> Option<usize>{
    let leafs = t.leafs();

    for i in 0..leafs {
        let e = t.read_at(i).unwrap();
        if e == hash {
            return Some(i)
        }
    }
    None
}

pub fn new_tree(hashed: CryptoHashData) -> Result<MerkleRoot> {
    Ok(MerkleTree::from_data(hashed.0)? as MerkleRoot)
}

pub fn get_path(t: MerkleRoot, data: String) -> Option<Proof<CryptoSHA3256Hash>> {
    let proof_item = get_hash(&mut CryptoSha3Algorithm::default(), &data);
    if let Some(index) = get_leaf_index(&t, proof_item) {
        let proof = t.gen_proof(index).unwrap();
        return Some(proof)
    }
    None
}

pub fn validate(lemma: Vec<String>, path: Vec<usize>, data: String) -> Result<bool> {
    let lemma: Vec<CryptoSHA3256Hash> = lemma.into_iter().map(|l| {
        let decode = hex::decode(l).unwrap();
        *slice_as_hash(&decode)
    }).collect();

    let proof: Proof<CryptoSHA3256Hash> = Proof::new::<U0, U0>(
        None,
        lemma,
        path,
    ).unwrap();

    Ok(proof.validate_with_data::<CryptoSha3Algorithm>(&data).unwrap())
}