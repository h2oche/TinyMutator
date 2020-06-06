extern crate proc_macro;
extern crate syn;

use std::{fs, env, io::prelude::*, str::FromStr};
use syn::{parse_macro_input, DeriveInput, Type, Expr, Result, Stmt, spanned::Spanned};
use proc_macro::{TokenStream, TokenTree};
use quote::quote_spanned;
use rand::seq::SliceRandom;

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

    let mut constants = vec!["0", "1", "-1"];

    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let content = fs::read_to_string(file).expect("Something went wrong reading the file");

    println!("{:#?}", content);
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
        // println!("{:#?}", line);
        let mut expr = syn::parse_str::<Stmt>(line);
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
                        println!("{:#?}", item);
                        println!("{}", line);
                        let mut const_expr: Vec<_> = line.split("=").collect();
                        println!("{}", &const_expr[1].trim_end_matches(';').trim());
                        constants.push(const_expr[1].trim_end_matches(";").trim());
                        println!("hi")
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
    println!("\n\n\n\ntotal constants:");
    println!("{:?}", constants);
    println!("\n\n\n\n");
    
    let mut lines_list: Vec<_> = content.split("\r\n").collect();
    let line_tmp = lines_list[num_line - 1];
    let expr_tmp = syn::parse_str::<Stmt>(line_tmp);
    println!("{:?}", expr_tmp);
    match expr_tmp {
        Ok(stmt) => {
            println!("{:#?}", stmt);
            match stmt {
                syn::Stmt::Local(local) => { // let binding
                    ()
                }
                syn::Stmt::Item(item) => { // constant (Is typecheck required?)
                    let mut new_constant_vec: Vec<_> = constants.choose_multiple(&mut rand::thread_rng(), 1).collect();
                    let mut new_constant = new_constant_vec[0];
                    println!("\n\n\n\n\ngg");
                    println!("{:?}", new_constant);
                    let mut const_expr: Vec<_> = line_tmp.split("=").collect();
                    while const_expr[1].trim_end_matches(";").trim() == *new_constant {
                        new_constant_vec = constants.choose_multiple(&mut rand::thread_rng(), 1).collect();
                        new_constant = new_constant_vec[0];
                    }
                    let tmp = const_expr[0].to_string();
                    let const_string = tmp + &("= ".to_string()) + new_constant + &(";".to_string());
                    println!("{:#?}", const_string);
                    println!("\n\n\n\n\n");
                    lines_list[num_line - 1] = &const_string;
                    println!("{:#?}", lines_list.join("\t\r"));
                    // println!("{:#?}", syn::parse_file(&(lines_list.join("\t\r"))));
                    return lines_list.join("\t\r");
                }
                syn::Stmt::Expr(expr) => {
                    ()
                }
                _ => {
                    println!("not a case");
                }
            }
        },
        Err(error) => {
            println!("{}", error);
        },
    }

    return "hello".to_string(); // temporary return value
}