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

mod add_deletions_tests {
    use checker::add_deletions;
    use std::collections::HashSet;

    fn initialize() -> HashSet<String> {
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

    #[test]
    fn add_deletions_string() {
        let word = "nevil".to_owned();
        let mut set = initialize();

        let newly_added = add_deletions(&mut set, word);

        assert_eq!(newly_added.len(), 5);
        assert_eq!(set.len(), 8);

        for new_word in newly_added {
            assert_eq!(set.contains(&new_word), true);
        }
    }

    #[test]
    fn add_deletions_empty_string() {
        let word = "".to_owned();
        let mut set = initialize();

        let newly_added = add_deletions(&mut set, word);

        assert_eq!(newly_added.len(), 0);
        assert_eq!(set.len(), 3);
    }
}
