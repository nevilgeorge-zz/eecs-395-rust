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
}
