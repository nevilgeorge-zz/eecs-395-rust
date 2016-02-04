// graph.rs

use std::collections::{HashMap, HashSet};

pub struct Graph {
    graph: HashMap<String, HashSet<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            graph: HashMap::<String, HashSet<String>>::new(),
        }
    }

    pub fn add_node(&mut self, node: String) {
        self.graph.insert(node.to_owned(), HashSet::<String>::new());
    }

    pub fn add_edge(&mut self, node_a: String, node_b: String) {
        if let Some(set) = self.graph.get_mut(&node_a) {
            set.insert(node_b.to_owned());
        }
        if let Some(set) = self.graph.get_mut(&node_b) {
            set.insert(node_a.to_owned());
        }
    }
}
