extern crate proc_macro;
extern crate syn;

use std::{fs, env, cmp, io::prelude::*, str::FromStr};
use syn::{parse_macro_input, DeriveInput, Type, Expr, Result, Stmt, spanned::Spanned};
use proc_macro::{TokenStream, TokenTree};
use quote::quote_spanned;
use rand::seq::SliceRandom;

/**
 * Print type of an object
 */
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

/** 
 * Modify specific line of given file
*/
pub fn mutate_file_by_line(file: String, num_line: usize) -> String {
    let mut constants = vec!["0", "1", "-1"];
    let content = fs::read_to_string(file).expect("Something went wrong reading the file");

    let ast = syn::parse_file(&content);
    let lines = content.split("\r\n");
    let mut lines_list: Vec<_> = content.split("\r\n").collect();

    for line in lines {
        let mut expr = syn::parse_str::<Stmt>(line);
        match expr {
            Ok(stmt) => {
                match stmt {
                    syn::Stmt::Local(local) => { // let binding
                        // println!(" > {:#?}", &local);
                        // print_type_of(&local.init);
                    }
                    syn::Stmt::Item(item) => { // constant statement, use statement, ...(listed here : https://docs.rs/syn/1.0.30/syn/enum.Item.html)
                        match item {
                            syn::Item::Const(itemConst) => {
                                // println!("{}", line);
                                // println!("{:#?}", &itemConst);
                                let mut const_expr: Vec<_> = line.split("=").collect();
                                constants.push(const_expr[1].trim_end_matches(";").trim());
                            },
                            _ => { }
                        }
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
    }
    
    let (start, end) = find_min_parsable_lines(lines_list.clone(), num_line);
    let line_to_parse = lines_list[start..end].join("\t\n");
    let expr_to_mutate = syn::parse_str::<Stmt>(&line_to_parse);
    // println!("{:?}", expr_tmp);
    match expr_to_mutate {
        Ok(stmt) => {
            match stmt {
                syn::Stmt::Local(local) => { // let binding
                    ()
                }
                syn::Stmt::Item(item) => { // constant statement, use statement, ...(listed here : https://docs.rs/syn/1.0.30/syn/enum.Item.html)
                    match item {
                        syn::Item::Const(itemConst) => {
                            let mut new_constant_vec: Vec<_> = constants.choose_multiple(&mut rand::thread_rng(), 1).collect();
                            let mut new_constant = new_constant_vec[0];
                            let mut const_expr: Vec<_> = line_to_parse.split("=").collect();
                            while const_expr[1].trim_end_matches(";").trim() == *new_constant {
                                new_constant_vec = constants.choose_multiple(&mut rand::thread_rng(), 1).collect();
                                new_constant = new_constant_vec[0];
                            }
                            let tmp = const_expr[0].to_string();
                            let const_string = tmp + &("= ".to_string()) + new_constant + &(";".to_string());
                            lines_list[num_line - 1] = &const_string;
                            return lines_list.join("\t\r");
                        },
                        _ => { },
                    }
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
    return "hello".to_string(); // temporary value
}

/** 
 * Get smallest list of lines which is parsable with ast
*/
pub fn find_min_parsable_lines(splitted_file: Vec<&str>, num_line: usize) -> (usize, usize) {
    for j in 1..cmp::max(splitted_file.len() - num_line, num_line - 0) { // length
        for i in 0..j {
            if num_line + i - j <= 0 || num_line + i > splitted_file.len() { continue; }
            // println!("{:#?}", &splitted_file[(num_line + i - j)..(num_line + i)].join("\t\r"));
            // println!("{} {}", num_line + i - j, num_line + i);
            match :: syn::parse_str::<Stmt>(&splitted_file[(num_line + i - j)..(num_line + i)].join("\t\r")) {
                Ok(stmt) => {
                    return (num_line + i - j, num_line + i);
                },
                Err(error) => {},
            }
        }
    }
    return (0, splitted_file.len());
}