// use quote::quote;
// use syn::{parse_macro_input, DeriveInput};
use std::fs::File;
use std::io::prelude::*;

pub fn mutate(filename: String, line: i32) {
    println!("filename : {}", filename);
    println!("line : {}", line);
  
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
    println!("{}", contents.chars().nth(line).unwrap());
}