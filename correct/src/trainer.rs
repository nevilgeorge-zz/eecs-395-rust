use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

pub type CountTable = HashMap<String, usize>;

pub fn read_from_file(filename: &str) -> CountTable {

    let mut f = File::open(filename).expect("Error opening training file!");
    let mut corpus_string = String::new();
    let result = f.read_to_string(&mut corpus_string);

    match result {
        Err(_) => panic!("Failed to read from training file!"),
        Ok(_) => {
            let word_counts = tokenize(corpus_string);
            word_counts
        }
    }
}

fn tokenize(corpus_string: String) -> CountTable {
    let mut table = CountTable::new();

    for word in corpus_string.split(" ") {
        increment_word(&mut table, word.trim().to_owned());
    }

    table
}

fn increment_word(mut map: &mut CountTable, word: String) {
    *map.entry(word).or_insert(0) += 1;
}
