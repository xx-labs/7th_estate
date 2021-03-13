//! # Command: Generate proof of inclusion
//!

use super::*;
use crate::blockchain::merkle::*;

pub fn generate_proof(path: &str, data: &str) -> Result<()>{
    // Load tree from YAML file
    let tree: MerkleRoot = load_tree(String::from(path))?;

    // Generate proof of inclusion for data
    let m_path = get_path(tree, String::from(data).to_owned())?;

    // Get lemma and path
    let lemma = m_path.lemma();
    let p_path = m_path.path();

    // Print hex encoded lemma for usability
    println!("Mekle Path for {}:", data);
    lemma.into_iter().for_each(|l| {
        let encoded = hex::encode(l);
        println!("{}", encoded);
    });
    println!("{:?}", p_path);

    Ok(())
}