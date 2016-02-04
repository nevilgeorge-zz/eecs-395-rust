// graph.rs

use std::collections::{HashMap, HashSet};

pub struct Graph {
    pub map: HashMap<String, HashSet<String>>,
}

impl Graph {
    pub fn new(mapping: HashMap<String, Vec<String>>) -> Self {

        let mut new_graph = Graph {
            map: HashMap::<String, HashSet<String>>::new(),
        };

        for (node, neighbors) in &mapping {
            new_graph.add_node(node.to_owned());
        }

        for (node, neighbors) in &mapping {
            for neighbor in neighbors {
                new_graph.add_edge(node.to_owned(), neighbor.to_owned());
            }
        }

        new_graph
    }

    pub fn add_node(&mut self, node: String) {
        self.map.insert(node.to_owned(), HashSet::<String>::new());
    }

    pub fn add_edge(&mut self, node_a: String, node_b: String) {
        if let Some(set) = self.map.get_mut(&node_a) {
            set.insert(node_b.to_owned());
        }
        if let Some(set) = self.map.get_mut(&node_b) {
            set.insert(node_a.to_owned());
        }
    }

    pub fn find_path(&mut self, src: String, dest: String) {
        if !self.map.contains_key(&src) || !self.map.contains_key(&dest) {
            panic!("A given node does not exist in the graph!");
        }

        let mut visited = HashSet::<String>::new();
        let mut queue: Vec<String> = vec![];
        let mut prev = HashMap::<String, String>::new();

        let mut current_node: String;
        let mut neighbors: &HashSet<String>;
        queue.push(src.to_owned());
        visited.insert(src.to_owned());

        while queue.len() > 0 {
            current_node = queue.remove(0 as usize);

            if current_node == dest.to_string() {
                print_path(src.to_owned(), dest.to_owned(), &prev);
                return;
            }

            neighbors = self.map.get(&current_node).unwrap();
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push(neighbor.clone());
                    prev.insert(neighbor.clone(), current_node.clone());
                }
            }

        }
        println!("Path between {} and {} does not exist.", src, dest);
    }
}

fn print_path(src: String, dest: String, prev: &HashMap<String, String>) {
    let mut path: String = dest.to_owned();
    let mut curr: String = dest;

    while curr != src {
        if let Some(predecessor) = prev.get(&curr) {
            path = predecessor.to_string() + " " + &path;
            curr = predecessor.to_string();
        }
    }
    println!("{}", path);
}
