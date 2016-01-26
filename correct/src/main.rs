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

    let mut possible_words = HashSet::<String>::new();

    // add all possible deletions
    for word in words {
        checker::insert_deletions(&mut possible_words, word);
    }

    for word in &possible_words {
        println!("{}", word);
    }

}
