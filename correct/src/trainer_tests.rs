// trainer_tests.rs

#[cfg(test)]
mod tokenize_tests {
    use trainer::{tokenize};

    #[test]
    fn tokenize_if_empty() {
        let string = "".to_owned();
        let h = tokenize(string);

        // adds empty string
        assert_eq!(h.len(), 1);
    }

    #[test]
    fn tokenize_string() {
        let string = "hello hello hello world world".to_owned();
        let h = tokenize(string);

        assert_eq!(h.len(), 2);
        assert_eq!(h.get("hello"), Some(&3));
        assert_eq!(h.get("world"), Some(&2));
    }
}

#[cfg(test)]
mod increment_word_tests {
    use trainer::increment_word;
    use super::super::CountTable;

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
