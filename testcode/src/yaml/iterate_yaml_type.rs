extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};
use yaml_rust::Yaml;

pub fn test() {
    let config = YamlLoader::load_from_str("
settinga: bla
settingb: bleh
").unwrap();

    // YamlLoader accepts multiple documents
    // extract first as we have only one
    let doc = &config[0];
    
    // convert doc to a hash
    let hash = doc.as_hash().unwrap();   
    
    // iterate over (key, value) in hash
    for entry in hash.iter() {
        println!("> {:?}", entry);
    }
    /* 
    > (String("settinga"), String("bla"))
    > (String("settingb"), String("bleh"))
    */
}