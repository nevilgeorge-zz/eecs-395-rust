// main.rs

use std::collections::{HashMap, HashSet};
use std::env;
use std::io::stdin;

// user-defined external modules
mod trainer;
mod checker;

// tests for external modules
mod trainer_tests;
mod checker_tests;

// wrapper over HashMap, public because it is used in modules
pub type CountTable = HashMap<String, usize>;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Missing training file argument!");
    }

    let corpus: CountTable = trainer::read_from_file(&args[1]);
    let words: Vec<String> = checker::read_words(stdin());

    let mut possible_words: HashSet<String>;
    let mut newly_added: Vec<String>;

    // iterate through words entered from stdin
    for word in words {
        possible_words = HashSet::<String>::new();

        // add all possibe deletions, then perform all edits again
        newly_added = checker::add_deletions(&mut possible_words, word.clone());
        run_all_edits(&mut possible_words, newly_added);

        // add all possibe insertions, then perform all edits again
        newly_added = checker::add_insertions(&mut possible_words, word.clone());
        run_all_edits(&mut possible_words, newly_added);

        // add all possibe replacements, then perform all edits again
        newly_added = checker::add_replacements(&mut possible_words, word.clone());
        run_all_edits(&mut possible_words, newly_added);

        // add all possibe transpositions   , then perform all edits again
        newly_added = checker::add_transpositions(&mut possible_words, word.clone());
        run_all_edits(&mut possible_words, newly_added);

        // find a correct word
        let correction = checker::find_most_likely_word(possible_words, &corpus);
        print_results(word, correction);
    }
}

// run all edits on a given word and add the mutations to the set
fn run_all_edits(mut set: &mut HashSet<String>, words: Vec<String>) {
    for word in words {
        checker::add_deletions(&mut set, word.clone());
        checker::add_insertions(&mut set, word.clone());
        checker::add_replacements(&mut set, word.clone());
        checker::add_transpositions(&mut set, word);
    }
}

fn print_results(word: String, correction: String) {
    // print only one word if there word was spelled correctly
    if word == correction {
        println!("{}", word);
    } else {
        println!("{}, {}", word, correction);
    }
}
