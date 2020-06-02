use std::env;
mod mutate;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let line = &args[2];

    mutate::mutate(filename.to_string(), line.to_string().parse().unwrap());
}
