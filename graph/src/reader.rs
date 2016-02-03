// reader.rs

use std::io::{BufRead, BufReader, Read};

pub fn read_query<R: Read>(reader: R) -> Vec<(String, String)> {
    let mut nodes = vec![];
    let lines = BufReader::new(reader).lines();

    for line in lines {
        if let Ok(v) = line {
            let mut tokens = v.trim().split(" ");

            let (size, _) = tokens.size_hint();
            if size > (2 as usize) {
                println!("{}", "Too many nodes entered!");
                break;
            }

            let pair: (String, String) = (tokens.next().unwrap().to_owned(), tokens.next().unwrap().to_owned());
            nodes.push(pair);
        }
    }

    nodes
}
