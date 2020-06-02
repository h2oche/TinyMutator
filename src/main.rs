use std::env;
use std::fs;
fn print_file() {
    let args: Vec<String> = env::args().collect();
    
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

fn main() {
    println!("Hello, world!");
    print_file(); // cargo run ./src/guessing_game.rs
}
