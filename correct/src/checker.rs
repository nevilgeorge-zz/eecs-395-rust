// checker.rs

use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

pub fn read_words<R: Read>(reader: R) -> Vec<String> {
    let mut all_words: Vec<String> = vec![];
    let lines = BufReader::new(reader).lines();

    for line in lines {
        all_words.push(line.unwrap().trim().to_owned());
    }

    all_words
}

pub fn insert_deletions(set: &mut HashSet<String>, word: String) {
    let mut sub_word: String;
    for i in 0..word.len() {
        sub_word = (&word[..i]).to_string() + &word[i + 1..];
        set.insert(sub_word);
    }
}
