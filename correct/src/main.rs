// main.rs

use std::collections::HashSet;
use std::env;
use std::io::{stdin};

mod trainer;
mod checker;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Missing training file argument!");
    }

    let corpus = trainer::read_from_file(&args[1]);
    let words = checker::read_words(stdin());

    let mut possible_words: HashSet<String>;

    // add all possible deletions
    for word in words {
        possible_words = HashSet::<String>::new();

        checker::add_deletions(&mut possible_words, word.clone());
        checker::add_insertions(&mut possible_words, word.clone());
        checker::add_replacements(&mut possible_words, word.clone());
        checker::add_transpositions(&mut possible_words, word.clone());

        let text = checker::find_most_likely_word(possible_words, &corpus);
        println!("{}, {}", word, text);
    }
}
