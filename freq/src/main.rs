use std::io::{BufRead, BufReader, Read, stdin};

#[doc="
    Counts the frequencies of words read from stdin, and prints a
    sorted frequency table.
"]

type CountTable = std::collections::HashMap<String, usize>;

#[derive(Clone)]
struct Pair {
    word: String,
    count: usize,
}

fn main() {
    let mut count_table = CountTable::new();
    // read words from specified reader
    let all_words = read_words(stdin());

    // count word frequencies and put them in the count table
    count_words(all_words, &mut count_table);

    // convert count table entries into a vector of pairs
    let mut pairs: Vec<Pair> = get_pairs(count_table);
    // sort the pairs in decreasing order
    sort_pairs(&mut pairs);
    // print the sorted pairs
    print_table(pairs);
}

fn read_words<R: Read>(reader: R) -> Vec<String> {
    let mut all_words: Vec<String> = vec![];
    let lines = BufReader::new(reader).lines();

    for line in lines {
        for word in line.unwrap().split(" ") {
            all_words.push(word.trim().to_owned());
        }
    }

    all_words
}

fn count_words(all_words: Vec<String>, mut map: &mut CountTable) {
    for word in all_words {
        increment_word(&mut map, word);
    }
}

fn increment_word(mut map: &mut CountTable, word: String) {
    *map.entry(word).or_insert(0) += 1;
}

fn get_pairs(map: CountTable) -> Vec<Pair> {
    let mut pairs: Vec<Pair> = vec![];
    let mut pair: Pair;
    let mut word_copy: String;

    for (word, count) in map.iter() {
        word_copy = word.clone().to_owned();
        pair = Pair {
            word: word_copy,
            count: *count
        };
        pairs.push(pair);
    }

    pairs
}

fn sort_pairs(mut pairs: &mut Vec<Pair>) {
    pairs.sort_by(|a, b| b.count.cmp(&a.count));
}

fn print_table(pairs: Vec<Pair>) {
    for pair in pairs {
        println!("{}: {}", pair.word, pair.count);
    }
}


/*
The following are tests written for each function.
Each module describes tests for one function.

*/

#[cfg(test)]
mod read_words_tests {
    use super::{read_words};
    use std::io::{Read, Result};

    struct StringReader {
        contents: Vec<u8>,
        position: usize,
    }

    impl StringReader {
        fn new(s: String) -> Self {
            StringReader {
                contents: s.into_bytes(),
                position: 0,
            }
        }
    }

    impl Read for StringReader {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let mut count = 0;

            while self.position < self.contents.len() && count < buf.len() {
                buf[count] = self.contents[self.position];
                count += 1;
                self.position += 1;
            }

            return Ok(count);
        }
    }

    fn assert_read(expected: &[&str], input: &str) {
        let mock_read = StringReader::new(input.to_owned());
        let all_words = read_words(mock_read);

        assert_eq!(expected.to_owned(), all_words);
    }

    #[test]
    fn read_one_line() {
        assert_read(&["Hello", "World", "Goodbye"], "Hello World Goodbye");
    }

    #[test]
    fn read_three_lines() {
        assert_read(&["Hello", "World", "Goodbye"], "Hello\nWorld\nGoodbye");
    }

    #[test]
    fn read_empty_string() {
        assert_read(&[], "");
    }
}

#[cfg(test)]
mod increment_word_tests {
    use super::{increment_word, CountTable};

    fn initialize() -> CountTable {
        let mut h = CountTable::new();

        h.insert("one".to_owned(), 1);
        h.insert("two".to_owned(), 2);

        assert_eq!(h.get("one"), Some(&1));
        assert_eq!(h.get("two"), Some(&2));
        assert_eq!(h.get("three"), None);
        assert_eq!(h.len(), 2);

        h

    }

    #[test]
    fn insert_if_empty() {
        let mut h = CountTable::new();
        increment_word(&mut h, "hello".to_owned());

        assert_eq!(h.get("hello"), Some(&1));
        assert_eq!(h.len(), 1);
    }

    #[test]
    fn insert_if_absent() {
        let mut tester = initialize();
        let mut expected = initialize();

        increment_word(&mut tester, "three".to_owned());
        expected.insert("three".to_owned(), 1);

        assert_eq!(expected, tester);
    }

    #[test]
    fn insert_if_present() {
        let mut tester = initialize();
        let mut expected = initialize();

        increment_word(&mut tester, "two".to_owned());
        expected.insert("two".to_owned(), 3);

        assert_eq!(expected, tester);
    }
}


#[cfg(test)]
mod count_words_tests {
    use super::{count_words, CountTable};

    #[test]
    fn count_all_words() {
        let all_words: Vec<String> = vec!["Hello".to_owned(), "World".to_owned(), "Hello".to_owned(), "Goodbye".to_owned()];
        let mut tester = CountTable::new();

        count_words(all_words, &mut tester);

        let mut expected = CountTable::new();
        expected.insert("Hello".to_owned(), 2);
        expected.insert("World".to_owned(), 1);
        expected.insert("Goodbye".to_owned(), 1);

        assert_eq!(expected, tester);
        assert_eq!(expected.len(), 3);
    }

    #[test]
    fn count_words_empty() {
        let all_words: Vec<String> = vec![];
        let mut tester = CountTable::new();

        count_words(all_words, &mut tester);

        let expected = CountTable::new();

        assert_eq!(expected, tester);
        assert_eq!(expected.len(), 0);
    }
}

#[cfg(test)]
mod get_pairs_tests {
    use super::{get_pairs, CountTable, Pair};

    #[test]
    fn get_pairs_many() {
        let mut h = CountTable::new();

        h.insert("Hello".to_owned(), 2);
        h.insert("World".to_owned(), 5);
        h.insert("Goodbye".to_owned(), 3);
        h.insert("Mars".to_owned(), 4);
        let map_length = h.len();

        let pairs: Vec<Pair> = get_pairs(h);

        assert_eq!(map_length, pairs.len());
    }

    #[test]
    fn get_pairs_none() {
        let h = CountTable::new();
        let pairs = get_pairs(h);

        assert_eq!(pairs.len(), 0);
    }
}

#[cfg(test)]
mod sort_pairs_tests {
    use super::{sort_pairs, Pair};

    #[test]
    fn sort_pairs_many() {
        let first = Pair { word: "First".to_owned(), count: 1 };
        let second = Pair { word: "Second".to_owned(), count: 2 };
        let third = Pair { word: "Third".to_owned(), count: 3 };
        let fourth = Pair { word: "Fourth".to_owned(), count: 4 };

        let mut pairs: Vec<Pair> = vec![second.clone(), fourth.clone(), third.clone(), first.clone()];

        sort_pairs(&mut pairs);

        assert_eq!(pairs[0].word, fourth.word);
        assert_eq!(pairs[1].word, third.word);
        assert_eq!(pairs[2].word, second.word);
        assert_eq!(pairs[3].word, first.word);
    }
}
