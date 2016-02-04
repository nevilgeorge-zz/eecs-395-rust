// reader.rs

use std::collections::{HashMap, HashSet};
use std::fs::File;
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

pub fn read_graph(filename: &str) -> HashMap<String, HashSet<String>> {

    let mut mapping = HashMap::<String, HashSet<String>>::new();
    let f = File::open(filename).expect("Error opening graph description file!");
    let lines = BufReader::new(&f).lines();
    let mut adjacent_nodes: HashSet<String>;

    for line in lines {
        if let Ok(text) = line {
            let mut tokens = text.trim().split(" ");
            let key = tokens.next();

            if let Some(node_key) = key {
                if mapping.contains_key(node_key) {
                    println!("Graph cannot have duplicate nodes!");
                    break;
                }

                adjacent_nodes = HashSet::<String>::new();
                for token in tokens {
                    adjacent_nodes.insert(token.to_owned());
                }

                mapping.insert(node_key.to_owned(), adjacent_nodes);
            } else {
                break;
            }
        }
    }

    create_full_graph(&mut mapping);
    mapping
}

fn create_full_graph(mapping: &mut HashMap<String, HashSet<String>>) {
    for (node, neighbors) in mapping {
        for neighbor in neighbors.iter() {
            println!("{}", neighbor);
            // let set = mapping.get_mut(&neighbor).unwrap();
            let mut set = mapping.get_mut(neighbor).unwrap();

            // *set.insert(node);
            // new_mapping.insert(node, *set);
            // for item in set.iter() {
            //     println!("{}", item);
            // }
        }
    }
}
