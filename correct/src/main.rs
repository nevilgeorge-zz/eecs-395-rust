use std::env;

mod trainer;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Missing training file argument!");
    }

    let table = trainer::read_from_file(&args[1]);
    for (word, count) in table.iter() {
        println!("{}: {}", word, count);
    }
}
