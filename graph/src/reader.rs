// reader.rs

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

pub fn read_graph<R: Read>(reader: R) -> HashMap<String, Vec<String>> {
    let mut mapping = HashMap::<String, Vec<String>>::new();
    let lines = BufReader::new(reader).lines();
    let mut adjacent_nodes: Vec<String>;

    for line in lines {
        // println!("{}", line.unwrap());
        if let Ok(v) = line {
            let mut tokens = v.trim().split(" ");
            let key = tokens.next();

            adjacent_nodes = vec![];
            for token in tokens {
                adjacent_nodes.push(token.to_owned());
            }

            if let Some(node_key) = key {
                mapping.insert(node_key.to_owned(), adjacent_nodes);
            } else {
                break;
            }
        }
    }

    mapping
}
