use merkletree::merkle::MerkleTree;
use merkletree::store::{Store, VecStore};
use merkletree::proof::Proof;
use crate::Result;
use crypto::digest::Digest;
use crypto::sha3::{Sha3, Sha3Mode};
use merkletree::hash::Algorithm;
use std::hash::Hasher;

use typenum::U0;

use std::fs::File;
use std::io::{Write, Read}; //, BufReader, BufRead};

pub type MerkleRoot = MerkleTree<CryptoSHA3256Hash, CryptoSha3Algorithm, VecStore<CryptoSHA3256Hash>>;
pub type CryptoSHA3256Hash = [u8; 32];
pub struct CryptoSha3Algorithm(Sha3);
pub struct CryptoHashData(pub Vec<String>);


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

// Get hash of String of data
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

// Search in a tree for leaf index of a given hash
fn get_leaf_index(t: &MerkleRoot, hash: CryptoSHA3256Hash) -> Result<usize>{
    let leafs = t.leafs();

    // Iterate tree leafs
    for i in 0..leafs {
        let e = t.read_at(i).unwrap();

        // If leaf == hash, return index
        if e == hash {
            return Ok(i)
        }
    }
    panic!("Data not found in tree");
}

// Create new tree from array of data
// Size of data MUST be power of 2
pub fn new_tree(hashed: CryptoHashData) -> Result<MerkleRoot> {
    Ok(MerkleTree::from_data(hashed.0)? as MerkleRoot)
}

// Get merkle path for a String of data
// Returns Proof struct if data in tree
pub fn get_path(t: MerkleRoot, data: String) -> Result<Proof<CryptoSHA3256Hash>> {
    // Hash input data
    let proof_item = get_hash(&mut CryptoSha3Algorithm::default(), &data);

    // Get leaf index of hashed data
    let index = get_leaf_index(&t, proof_item)?;
    
    // If hashed data in leafs, return Proof
    let proof = t.gen_proof(index).unwrap();
    
    Ok(proof)
}


// Validate proof of inclusion
pub fn validate(lemma: Vec<String>, path: Vec<usize>, data: String) -> Result<bool> {
    // Decode hash Strings into [u8; 32] bytes 
    let lemma: Vec<CryptoSHA3256Hash> = lemma.into_iter().map(|l| {
        let decode = hex::decode(l).unwrap();
        *slice_as_hash(&decode)
    }).collect();

    // Generate Proof struct with given Lemma and Path
    let proof: Proof<CryptoSHA3256Hash> = Proof::new::<U0, U0>(
        None,
        lemma,
        path,
    ).unwrap();

    // Return proof result
    Ok(proof.validate_with_data::<CryptoSha3Algorithm>(&data).unwrap())
}

// Store tree in YAML file
pub fn store_tree(tree: &MerkleRoot, path: String) -> Result<()> {
    // Open file for writing
    let mut output_file = File::create(path)?;

    // Get tree data
    let t_data = tree.data().unwrap();

    // Serialize tree data (hashes) into Vec of hex encoded strings
    let mut ser_data = Vec::with_capacity(t_data.len());
    for d in t_data.into_iter() {
        ser_data.push(hex::encode(d));
    }

    // Load Vec<String> into YAML array
    let ser_data = serde_yaml::to_string(&ser_data).unwrap();

    // Write YAML array to file
    Ok(write!(output_file, "{}", ser_data)?)
}


// Load tree from YAML file
pub fn load_tree(path: String) -> Result<MerkleRoot> {
    // Open file for reading
    let mut input_file = File::open(path)?;


    // Load tree as one string -> YAML array
    let mut ser_data: String = String::new();
    input_file.read_to_string(&mut ser_data)?;

    // Load yaml array into Vec<String> of hashes
    let tree_data: Vec<String> = serde_yaml::from_str(&ser_data).unwrap();

    // Create new VecStore and push each hash into it
    let mut v_store: VecStore<[u8; 32]> = VecStore::new(tree_data.len()).unwrap();
    tree_data.into_iter().for_each(|d| {
        // Decode hash into bytes
        let d = hex::decode(d).unwrap();

        // Load bytes into VecStore
        v_store.push(*slice_as_hash(&d)).unwrap();
    });

    // Reconstruct tree from VecStore with hashes
    let leafs = (v_store.len() + 1) / 2 as usize;
    let reconstructed: MerkleTree<[u8; 32], CryptoSha3Algorithm, VecStore<_>> = MerkleTree::from_data_store(v_store, leafs)?;

    Ok(reconstructed)
}