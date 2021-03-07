use seventh_estate::blockchain::merkle::*;

#[test]
fn test_validate() {
    let lemma = vec![
    "fc5b5f4e58fcb551bdfcb729d5fc7c5cc80b5fb990758a50bb0827ee79effee1".to_string(),
    "d21154d769c7d5856960ddf30fc86b01385b27ca8760b608332f2d9e7b99a388".to_string(),
    "95fffb71ae3c85686532169334512226eaf55dc6f94333d2f5f6e950812c08fd".to_string(),
    "c1840a2abbd13f9b624ed37f6646da6c824324d17d06a039a1dacb79c62fa52f".to_string(),
    "5873a2bee359c763dc3adb0d44d72fe4fff49114f5fc02a7505c8ee61aa4c184".to_string(),
    ];
    let path = vec![0, 1, 0];
    let data = "Not Voted".to_string();

    assert_eq!(true, validate(lemma, path, data).unwrap());
}

#[test]
fn test_root() {

    let data = vec![
        "Colombier,Gerri,7 Del Sol Lane,Philadelphia,PA,19160"                                                    .to_string(),
        "64: 86961-67106-91541-74973"                                                                             .to_string(),
        "Not Voted"                                                                                               .to_string(),
        "$chacha20_poly1305_aead$GZm76RMgPAkMQMki$R1ptNzZSTWdQQWtNUU1raQ==$OFz4Z9GNmg==$6MzPD1MV07tqNG+JCYkp6Q==$".to_string(),
        "13, 20, 35, 43, 58, 69, 73, 77, 81, 88, 93, 96"                                                          .to_string(),
    ];

    let mut data = CryptoHashData::new(data);
    data.pad();

    let t = new_tree(data).unwrap();

    assert_eq!("5873a2bee359c763dc3adb0d44d72fe4fff49114f5fc02a7505c8ee61aa4c184", hex::encode(t.root()));

}

#[test]
fn test_get_path() {
    let data = vec![
        "Colombier,Gerri,7 Del Sol Lane,Philadelphia,PA,19160"                                                    .to_string(),
        "64: 86961-67106-91541-74973"                                                                             .to_string(),
        "Not Voted"                                                                                               .to_string(),
        "$chacha20_poly1305_aead$GZm76RMgPAkMQMki$R1ptNzZSTWdQQWtNUU1raQ==$OFz4Z9GNmg==$6MzPD1MV07tqNG+JCYkp6Q==$".to_string(),
        "13, 20, 35, 43, 58, 69, 73, 77, 81, 88, 93, 96"                                                          .to_string(),
    ];

    let mut data = CryptoHashData::new(data);
    data.pad();

    let t = new_tree(data).unwrap();

    let p = get_path(t, "Not Voted".to_string()).unwrap();
    let lemma = p.lemma().to_owned();
    let path = p.path().to_owned();

    let c_lemma = vec![
        "fc5b5f4e58fcb551bdfcb729d5fc7c5cc80b5fb990758a50bb0827ee79effee1".to_string(),
        "d21154d769c7d5856960ddf30fc86b01385b27ca8760b608332f2d9e7b99a388".to_string(),
        "95fffb71ae3c85686532169334512226eaf55dc6f94333d2f5f6e950812c08fd".to_string(),
        "c1840a2abbd13f9b624ed37f6646da6c824324d17d06a039a1dacb79c62fa52f".to_string(),
        "5873a2bee359c763dc3adb0d44d72fe4fff49114f5fc02a7505c8ee61aa4c184".to_string(),
        ];
    let c_lemma: Vec<CryptoSHA3256Hash> = c_lemma.into_iter().map(|l|{
        let decode = hex::decode(l).unwrap();
        *slice_as_hash(&decode)
    }).collect();

    assert_eq!(c_lemma, lemma);
    assert_eq!(vec![0, 1, 0], path);
    
}