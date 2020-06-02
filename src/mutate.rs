// use quote::quote;
extern crate proc_macro;
extern crate syn;
use syn::{parse_macro_input, DeriveInput, Type, Expr, Result, parse_str, parse};
use std::fs::File;
use std::io::prelude::*;

pub fn mutate(filename: String, line: i32) -> Result<()> {
    println!("filename : {}", filename);
    println!("line : {}", line);
  
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading the file");
    // println!("With text:\n{}", contents);

    // let code = "assert_eq!(u8::max_value(), 255)";
    let expr = syn::parse_str::<Expr>(&contents)?;
    println!("{:#?}", expr);

    // for line in expr {
    //     println!("{}", line)
    // }
    Ok(())
    // println!("{}", contents.chars().nth(line).unwrap());
    // let ast: Type = syn::parse_str(&contents).unwrap();
    // let stream: proc_macro::TokenStream = contents.parse().unwrap();
}