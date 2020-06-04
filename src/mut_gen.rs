extern crate proc_macro;
extern crate syn;
extern crate syntex_syntax as syntax;

use proc_macro::{TokenStream, TokenTree};
use std::{fs, env, io::prelude::*, str::FromStr};
use syn::{parse_macro_input, DeriveInput, Type, Expr, Result, Stmt};
use syntax::{parse};

/**
 * Print type of an object
 */
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

/** 
 * Modify specific line of given file
*/
pub fn mutate_file_by_line(file: String, num_line: usize) -> String {
    println!("filename : {}", file);
    println!("line : {}", num_line);

    let mut vec = vec![0, 1, -1];

    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let content = fs::read_to_string(file).expect("Something went wrong reading the file");
    let ast = syn::parse_file(&content);
    // for item in ast.items.iter() {
    //     match item {
    //         _ => { 
    //             print_type_of(item)
    //         },
    //     }
    //     // println!("{:#?}", item);
    //     println!("\n\n\n\n\n\n\n\n\n\n\n\n");
    // }
    // println!("Hello");
    let lines = content.split("\r\n");
    // let mut lines: Vec<_> = content.split("\r\n").collect();
    for line in lines {
    //     println!("\n\n\n\n\n");
        println!(" > {:#?}", line);
        // print_type_of(&line);
        // print_type_of(&lines);
        let expr = syn::parse_str::<Stmt>(line);
        match expr {
            Err(e) => (),
            _ => {
                println!("statement found!");
                println!(" > {:#?}", expr);
            }
        }
        // println!(" > {:#?}", tmp_content2);
        println!("\n\n\n\n\n");
    }
    // temporary
    return "hello".to_string();
}