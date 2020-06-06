extern crate proc_macro;
extern crate syn;

use std::{fs, env, io::prelude::*, str::FromStr};
use syn::{parse_macro_input, DeriveInput, Type, Expr, Result, Stmt, spanned::Spanned};
use proc_macro::{TokenStream, TokenTree};
use quote::quote_spanned;

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

    let mut constants = vec![0, 1, -1];

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
    // }
    // println!("Hello");

    let lines = content.split("\r\n");
    for line in lines {
        let expr = syn::parse_str::<Stmt>(line);
        match expr {
            Ok(stmt) => {
                match stmt {
                    syn::Stmt::Local(local) => { // let binding
                        // println!(" > {:#?}", &local);
                        // println!("This is the case:");
                        // stmt.unwrap();
                        // print_type_of(&local.init);
                    }
                    syn::Stmt::Item(item) => { // constant and something                        
                        match item {
                            syn::Item::Use(_itemuse) => {},
                            _ => {}

                        }
                        // println!("{:#?}", item);
                        // println!("{}", line);
                        let mut constexpr: Vec<_> = line.split("=").collect();
                        println!("{:#?}", constexpr);
                        // println!("{}", &constexpr[1])
                    }
                    syn::Stmt::Expr(expr) => {
                        // println!("{:#?}", expr);
                    }
                    _ => {
                        println!("not a case");
                    }
                }
            },
            Err(error) => { // syntax error of target file
                println!("{}", error);
            },
        }
        // println!("\n\n\n\n\n");
    }
 
    let mut lines: Vec<_> = content.split("\r\n").collect();
    
    println!("{}", &lines[num_line - 1]);

    return "hello".to_string(); // temporal return value
}