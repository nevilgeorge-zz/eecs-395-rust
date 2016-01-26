// checker.rs

use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

pub fn read_words<R: Read>(reader: R) -> Vec<String> {
    let mut all_words: Vec<String> = vec![];
    let lines = BufReader::new(reader).lines();

    for line in lines {
        all_words.push(line.unwrap().trim().to_owned());
    }

    all_words
}

pub fn add_deletions(set: &mut HashSet<String>, word: String) {
    let mut sub_word: String;
    for i in 0..word.len() {
        sub_word = (&word[..i]).to_string() + &word[i + 1..];
        set.insert(sub_word);
    }
}

pub fn add_insertions(set: &mut HashSet<String>, word: String) {
    let alphabet = ALPHABET.to_owned();
    let mut sub_word: String;

    for i in 0..word.len() {
        for c in alphabet.chars() {
            sub_word = (&word[..i]).to_string() + &c.to_string() + &word[i..];
            set.insert(sub_word);
        }
    }

    for c in alphabet.chars() {
        sub_word = (&word).to_string() + &c.to_string();
        set.insert(sub_word);
    }
}
