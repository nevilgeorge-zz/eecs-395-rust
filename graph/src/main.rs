// main.rs
#[doc="
graph

Nevil George nsg622
Diane Liu dlq200

Finds paths in graphs.
Reads graph specifications file from command line argument and
answers routing queries typed by user.

Usage: cargo run <graphfile.dat>

ASSUMPTIONS:
* The program runs indefinitely until the user types 'quit'.
* Query/answer pairs are separated by new lines because we couldn't figure out
  how to display arrows '->' in the same line that users type their queries.
"]


use std::env;
use std::io::stdin;

mod reader;
mod graph;
mod graph_tests;

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
                    let found_path = new_graph.find_path(nodes[0].clone(), nodes[1].clone());
                    println!("{}", found_path);
                }
            }
            Err(error) => println!("Error reading input: {}", error)
        }
    }
}

fn verify_input(input: String) -> Vec<String>  {
    let mut nodes: Vec<String> = vec![];
    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len() != 2 {
        println!("Incorrect input format. Please enter two nodes separated by whitespace.\nUsage: <src> <dest>\n");
        return nodes;
    }
    else {
        nodes.push(tokens[0].to_owned());
        nodes.push(tokens[1].to_owned());
    }
    nodes
}
