//! # Command: Generate proof of inclusion
//!

use super::*;
use crate::blockchain::merkle::*;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct GeneratedProof {
    data: String,
    lemma: Vec<String>,
    path: Vec<usize>
}

pub fn generate_proof(path: &str, data: &str) -> Result<()>{
    // Load tree from YAML file
    let tree: MerkleRoot = load_tree(String::from(path))?;

    // Generate proof of inclusion for data
    let m_path = get_path(tree, String::from(data).to_owned())?;

    // Get lemma and path
    let lemma = m_path.lemma();
    let p_path = m_path.path();

    // Print hex encoded lemma for usability
    let mut ser_lemma: Vec<String> = Vec::new();
    lemma.into_iter().for_each(|l| {
        let encoded = hex::encode(l);
        ser_lemma.push(encoded);
    });
    
    let ser_data = GeneratedProof {
        data: data.to_string(),
        lemma: ser_lemma,
        path: p_path.to_vec()
    };
    let ser_data = serde_yaml::to_string(&ser_data)?;
    println!("{}", ser_data);

    Ok(())
}

pub fn validate_proof(proof_path: &str) -> Result<()> {
    // Open file for reading
    let mut input_file = File::open(String::from(proof_path))?;


    // Load tree as one string -> YAML array
    let mut ser_data: String = String::new();
    input_file.read_to_string(&mut ser_data)?;

    // Load yaml array into Vec<String> of hashes
    let tree_data: GeneratedProof = serde_yaml::from_str(&ser_data).unwrap();

    if !validate(tree_data.lemma, tree_data.path, tree_data.data)?{
        panic!("Wrong proof of inclusion");
    }

    println!("Proof of inclusion validated correctly");
    Ok(())
}