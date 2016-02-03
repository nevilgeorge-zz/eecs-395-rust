// main.rs

use std::io::stdin;

mod reader;

fn main() {
    println!("Hello, world!");
    let result = reader::read_graph(stdin());

    for (key, list) in &result {
        println!("{}", key);
        for item in list {
            println!("{}", item);
        }
    }
}
