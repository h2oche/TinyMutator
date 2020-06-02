use std::env;
mod mutate;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let line = &args[2];

    let a = 1;
    let b = 1;
    a + b;
}