// checker_tests.rs

#[cfg(test)]
mod read_words_tests {
    use checker::read_words;
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
        assert_read(&["Hello There Everyone"], "Hello There Everyone");
    }

    #[test]
    fn read_three_lines() {
        assert_read(&["Hello", "There", "Everyone"], "Hello\nThere\nEveryone");
    }

    #[test]
    fn read_empty_string() {
        assert_read(&[], "");
    }
}

// functions that are common to all edit tests
#[cfg(test)]
mod edit_tests {
    use std::collections::HashSet;

    pub fn initialize_set() -> HashSet<String> {
        let mut set = HashSet::<String>::new();
        assert_eq!(set.len(), 0);
        // add random words to make set non empty
        set.insert("hello".to_owned());
        set.insert("everyone".to_owned());
        set.insert("goodbye".to_owned());

        // test with duplicate (should not be added)
        set.insert("goodbye".to_owned());

        assert_eq!(set.len(), 3);
        assert_eq!(set.contains("hello"), true);
        assert_eq!(set.contains("everyone"), true);
        assert_eq!(set.contains("goodbye"), true);

        set
    }
}

#[cfg(test)]
mod add_deletions_tests {
    use checker::add_deletions;
    use super::edit_tests::initialize_set;

    #[test]
    fn add_deletions_string() {
        let word = "nevil".to_owned();
        let mut set = initialize_set();
        let set_size = set.len();

        let newly_added = add_deletions(&mut set, word);

        assert_eq!(newly_added.len(), 5);
        assert_eq!(set.len(), set_size + newly_added.len());

        for new_word in newly_added {
            assert_eq!(set.contains(&new_word), true);
        }
    }

    #[test]
    fn add_deletions_empty_string() {
        let word = "".to_owned();
        let mut set = initialize_set();

        let newly_added = add_deletions(&mut set, word);

        assert_eq!(newly_added.len(), 0);
        assert_eq!(set.len(), 3);
    }
}

#[cfg(test)]
mod add_insertions_tests {
    use checker::add_insertions;
    use super::edit_tests::initialize_set;

    #[test]
    fn add_insertions_string() {
        let word = "nevil".to_owned();
        let mut set = initialize_set();

        let newly_added = add_insertions(&mut set, word.clone());

        // 26 characters in alphabet being added to every position in 5 letter word
        assert_eq!(newly_added.len(), 26 * (word.len() + 1));

        for new_word in newly_added {
            assert_eq!(set.contains(&new_word), true);
        }
    }

    #[test]
    fn add_insertions_empty_string() {
        let word = "".to_owned();
        let mut set = initialize_set();
        let set_size = set.len();

        let newly_added = add_insertions(&mut set, word);

        // add 26 alphabets
        assert_eq!(newly_added.len(), 26);
        assert_eq!(set.len(), set_size + 26);

        for new_word in newly_added {
            assert_eq!(set.contains(&new_word), true);
        }
    }
}

#[cfg(test)]
mod add_replacement_tests {
    use checker::add_replacements;
    use super::edit_tests::initialize_set;

    #[test]
    fn add_replacements_string() {
        let word = "nevil".to_owned();
        let mut set = initialize_set();

        let newly_added = add_replacements(&mut set, word.clone());

        // 26 characters in alphabet replacing every position in 5 letter word
        assert_eq!(newly_added.len(), 26 * word.len());

        for new_word in newly_added {
            assert_eq!(set.contains(&new_word), true);
        }
    }

    #[test]
    fn add_replacements_empty_string() {
        let word = "".to_owned();
        let mut set = initialize_set();
        let set_size = set.len();

        let newly_added = add_replacements(&mut set, word);

        // nothing to replace so nothing added
        assert_eq!(newly_added.len(), 0);
        assert_eq!(set.len(), set_size);
    }
}

#[cfg(test)]
mod add_transpositions_tests {
    use checker::add_transpositions;
    use super::edit_tests::initialize_set;

    #[test]
    fn add_transpositions_string() {
        let word = "nevil".to_owned();
        let mut set = initialize_set();
        let set_size = set.len();

        let newly_added = add_transpositions(&mut set, word.clone());

        assert_eq!(newly_added.len(), word.len() - 1);
        assert_eq!(set.len(), set_size + newly_added.len());

        for new_word in newly_added {
            assert_eq!(set.contains(&new_word), true);
        }
    }

    #[test]
    fn add_transpositions_empty_string() {
        let word = "".to_owned();
        let mut set = initialize_set();
        let set_size = set.len();

        let newly_added = add_transpositions(&mut set, word);

        // nothing to swap so nothing added
        assert_eq!(newly_added.len(), 0);
        assert_eq!(set.len(), set_size);
    }
}
