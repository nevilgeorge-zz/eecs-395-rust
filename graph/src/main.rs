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

    let result = reader::read_graph(&args[1]);

    // for (key, val) in &result {
    //     println!("{}", key);
    //     for v in val {
    //         println!("{}", v);
    //     }
    //     println!("---------");
    // }
}
