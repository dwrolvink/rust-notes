// pub fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

/*
    This file is an example for updating a default hashmap through a hashmap with
    custom values.
    
    [SPEC]
    (1) The custom hashmap has a supposedly similar (but possibly arbitrarily 
    different) structure, compared to default.

    (2) We will iterate through default. Any value where the key is also present
    in custom will be overwritten with the value in custom, this includes Arrays.

    (2.1) Type of the value is derrived from the side of the default value.
    If the values do not agree (e.q. String vs Hash), the copy is skipped, 
    and an error is printed to stderr.
    
    (3) Any key present in the custom hashmap not present in the default hashmap 
    is ignored. (This is easy to check for by recursing over the custom keys and 
    checking for each if they are present in the default hashmap).
*/

pub fn test() {

/*
    Define the two configs, default first, and custom config second.
    See bottom of file for expected output.
*/    

    let config = YamlLoader::load_from_str("
a:
  a1: old (should be overwritten)
  a2: old (should not be overwritten)
b: old (should be overwritten)
c: old (only in default)
---
a: 
  a1: new
  a2: 
    invalid: these should error
    invalid2: as type is not string for a2!
b: new
d: new (only in custom, should be ignored) 
").unwrap();

    // YamlLoader accepts multiple documents
    // extract first as we have only one
    let doc1 = &config[0];
    let doc2 = &config[1];
    
    // convert doc to a hash
    let mut defaults = doc1.as_hash().unwrap().clone();   
    let custom = doc2;  // don't unwrap yet, we need this functionality;


/*
    As Hashmaps may be arbitrarily deeply nested, we need to recurse every time
    we find a hashmap. If type is not a hash, compare the old and new value types
    (Yaml::<Variant>), if the variants are the same, the old value is overwritten
    with the new value.
*/

    fn rec(mut default: Hash, custom: &Yaml) -> Hash {// -> Yaml {
        for entry in default.iter_mut() {
            // unpack
            let key = &entry.0;
            let val = &entry.1;
            let key_str = key.as_str().unwrap();

            // skip entry if custom config does not have the key that default has
            // (e.g. "c")
            if variant_eq(&custom[key_str], &Yaml::BadValue) {
                continue;
            }

            // recurse and exit this branch if val = hash
            match val {
                Yaml::Hash(inner) => {
                    *entry.1 = Yaml::Hash(
                        rec(inner.clone(), &custom[key_str])
                    );
                    continue;
                },
                _ => (),
            };

            // update value only if types match
            if ! variant_eq(entry.1, &custom[key_str]) {
                eprintln!("Error: value types do not agree for key {}", key_str);
                continue;
            }

            // Update value
            *entry.1 = custom[key_str].clone();
        }
        return default
    }

    // overwrite defaults with the new hashmap values
    defaults = rec(defaults, custom);

    // Dump the YAML object for our viewing pleasure
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&Yaml::Hash(defaults)).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);

    /*
    Error: value types do not agree for key a2
    ---
    a:
    a1: new
    a2: old (should not be overwritten)
    b: new
    c: old (only in default)
    */

    /* 
    when we run it like this: ./target/release/binary 2> err.txt
    We lose the error:

    ---
    a:
    a1: new
    a2: old (should not be overwritten)
    b: new
    c: old (only in default)
    */
}