use std::env;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut hashpos: HashMap<String, String> = HashMap::new();

    // Work with argv[1] and argv[2]

    // Read contents from argv[1]
    let f1 = File::open(&argv[1]).unwrap();
    let f1 = BufReader::new(&f1);

    // Fill in hashpos with these contents
    eprintln!("Building map");

    for line in f1.lines() {
        // split
        let cts: Vec<String> = line.unwrap().split(' ')
            .map(|x| String::from(x))
            .collect();
        // add to hashmap
        hashpos.insert(cts[0].clone(), cts[1].clone());
    }

    // Read contents from argv[2]
    let f2 = File::open(&argv[2]).unwrap();
    let f2 = BufReader::new(&f2);

    // Query hashmap with strings from the other file
    for line in f2.lines() {
        let owned_line = String::from(line.unwrap());
        match hashpos.get(&owned_line) {
            Some(val) => println!("{} {}", &owned_line, val),
            None => (),
        }
    }
}
