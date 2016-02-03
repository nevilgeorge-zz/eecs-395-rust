// main.rs

use std::io::stdin;

mod reader;

fn main() {
    println!("Hello, world!");
    let result = reader::read_query(stdin());

    for input in result {
        let (src, dest) = input;
        println!("source: {}", src);
        println!("destination: {}", dest);
    }
}
