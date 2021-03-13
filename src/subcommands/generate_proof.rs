//! # Command: Generate proof of inclusion
//!

use super::*;
use crate::blockchain::merkle::*;

#[derive(Serialize)]
struct GeneratedProof {
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