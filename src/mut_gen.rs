extern crate proc_macro;
extern crate syn;
use syn::{parse_macro_input, DeriveInput, Type, Expr, Result, parse_str, parse};
use std::fs::File;
use std::io::prelude::*;


/** 
 * Modify specific line of given file
*/
fn mutate(filename: String, line: i32) -> Result<()> {
    println!("filename : {}", filename);
    println!("line : {}", line);
  
    let mut f = File::open(filename).expect("file not found");
    let mut contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let ast = syn::parse_str::(&contents)?;
    for x in ast.items.iter() {
        println!(" > {:#?}", x)
    }
    println!("{} items", ast.items.len());
    Ok(())
}