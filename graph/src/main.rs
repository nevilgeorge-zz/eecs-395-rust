// main.rs

use std::env;
use std::io::stdin;

mod reader;
mod graph;

fn main() {
    println!("Hello, world!");
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Missing graph description file!");
    }

    let graph_mappings = reader::read_graph(&args[1]);
    let new_graph = graph::Graph::new(graph_mappings);

    for (node, neighbors) in &new_graph.map {
        println!("Node: {}", node);
        for neighbor in neighbors {
            println!("Neighbor: {}", neighbor);
        }
        println!("---------");
    }
}
