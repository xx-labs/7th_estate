// use seventh_estate::blockchain::*;
use seventh_estate::blockchain::merkle::*;

#[tokio::test]
async fn test_post() {
    let data = vec![String::from("This is a unit test")];
    let mut data = CryptoHashData::new(data);    
    data.pad();
    
    let _tree = new_tree(data).unwrap();
    // TODO: Futures not resolving in test
    // assert_eq!((), post(tree.root()).unwrap());
}