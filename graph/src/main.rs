// main.rs
#[doc="
graph

Nevil George nsg622
Diane Liu dlq200

Finds paths in graphs. Reads graph specifications file from command line argument
answers routing queries typed by user.

Usage: cargo run <graphfile.dat>

ASSUMPTIONS:
* 'quit' quits the program.
*
"]


use std::env;
use std::io::stdin;

mod reader;
mod graph;

fn main() {

    // read command line input and find graph file
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Missing graph description file!");
    }

    // construct graph
    let graph_mappings = reader::read_graph(&args[1]);
    let mut new_graph = graph::Graph::new(graph_mappings);

    // listen for stdin
    let mut input = String::new();
    while input != "quit\n".to_owned() {
        input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                let nodes = verify_input(input.clone());
                if nodes.len() == 2 {
                    new_graph.find_path(nodes[0].clone(), nodes[1].clone());
                }
            }
            Err(error) => println!("Error reading input: {}", error)
        }
    }
}

fn verify_input(input: String) -> Vec<String>  {
    let mut nodes: Vec<String> = vec![];
    let mut tokens: Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len() != 2 {
        println!("Incorrect input format. Please enter two nodes separated by whitespace.\nUsage: <src> <dest>");
        return nodes;
    }
    else {
        nodes.push(tokens[0].to_owned());
        nodes.push(tokens[1].to_owned());
    }
    nodes
}
