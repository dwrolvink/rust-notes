extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};
use yaml_rust::Yaml;

pub fn test() {
    let config = YamlLoader::load_from_str("
settinga: bla
settingb: bleh
---
settingc: blergh
").unwrap();

    // YamlLoader accepts multiple documents
    let doc1 = &config[0];
    let doc2 = &config[1];
    
    // convert doc to a hash
    let mut hash1 = doc1.as_hash().unwrap().clone();   
    let hash2 = doc2.as_hash().unwrap();   

    hash1.extend(hash2.clone());
    
    // iterate over (key, value) in hash
    for entry in hash1.iter() {
        println!("> {:?}", entry);
    }
    /* 
    > (String("settinga"), String("bla"))
    > (String("settingb"), String("bleh"))
    > (String("settingc"), String("blergh"))
    */
}