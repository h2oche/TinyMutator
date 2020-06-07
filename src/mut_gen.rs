extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Span;
use quote::{quote, quote_spanned};
use rand::seq::SliceRandom;
use std::{
    env, fs,
    io::prelude::*,
    io::{self, BufRead},
    path::Path,
    process::Command,
    str::FromStr,
    vec,
};
use syn::{
    parse_macro_input, parse_quote,
    spanned::Spanned,
    visit::{self, Visit},
    visit_mut::{self, VisitMut},
    DeriveInput, Expr, File, Item, ItemFn, Lit, LitInt, Result, Stmt, Type,
};

/**
 * Print type of an object
 */
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

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
                    syn::Stmt::Local(local) => {
                        // let binding
                        // println!(" > {:#?}", &local);
                        // println!("This is the case:");
                        // stmt.unwrap();
                        // print_type_of(&local.init);
                    }
                    syn::Stmt::Item(item) => {
                        // constant and something
                        // match item {
                        //     syn::Item::Use(_itemuse) => {},
                        //     _ => {}

                        // }
                        println!("{:#?}", item);
                        println!("{}", line);
                        let mut constExpr: Vec<_> = line.split("=").collect();
                        println!("{}", &constExpr[1].trim_end_matches(';').trim());
                        constants.push(constExpr[1].trim_end_matches(";").trim());
                        println!("hi")
                    }
                    syn::Stmt::Expr(expr) => {
                        // println!("{:#?}", expr);
                    }
                    _ => {
                        println!("not a case");
                    }
                }
            }
            Err(error) => {
                // syntax error of target file
                println!("{}", error);
            }
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
                syn::Stmt::Local(local) => {
                    // let binding
                    ()
                }
                syn::Stmt::Item(item) => {
                    // constant (Is typecheck required?)
                    let mut new_constant_vec: Vec<_> = constants
                        .choose_multiple(&mut rand::thread_rng(), 1)
                        .collect();
                    let mut new_constant = new_constant_vec[0];
                    println!("\n\n\n\n\ngg");
                    println!("{:?}", new_constant);
                    let mut const_expr: Vec<_> = line_tmp.split("=").collect();
                    while const_expr[1].trim_end_matches(";").trim() == *new_constant {
                        new_constant_vec = constants
                            .choose_multiple(&mut rand::thread_rng(), 1)
                            .collect();
                        new_constant = new_constant_vec[0];
                    }
                    let tmp = const_expr[0].to_string();
                    let const_string =
                        tmp + &("= ".to_string()) + new_constant + &(";".to_string());
                    println!("{:#?}", const_string);
                    println!("\n\n\n\n\n");
                    lines_list[num_line - 1] = &const_string;
                    println!("{:#?}", lines_list.join("\t\r"));
                    // println!("{:#?}", syn::parse_file(&(lines_list.join("\t\r"))));
                    return lines_list.join("\t\r");
                }
                syn::Stmt::Expr(expr) => (),
                _ => {
                    println!("not a case");
                }
            }
        }
        Err(error) => {
            println!("{}", error);
        }
    }

    return "hello".to_string(); // temporary return value
}

struct BinOpVisitor;
impl VisitMut for BinOpVisitor {
    fn visit_bin_op_mut(&mut self, node: &mut syn::BinOp) {
        if let syn::BinOp::BitOr(or) = &node {
            let digits = or.span();
            *node = syn::BinOp::BitAnd(syn::token::And(node.span().clone()));
        }
        visit_mut::visit_bin_op_mut(self, node);
    }
}

pub fn mutate_file_by_line3(file: String, num_line: usize) -> String {
    let args: Vec<String> = env::args().collect();
    let file2 = &args[1];
    let example_source = fs::read_to_string(file2).expect("Something went wrong reading the file");
    let mut syntax_tree = syn::parse_file(&example_source).unwrap();
    BinOpVisitor.visit_file_mut(&mut syntax_tree);
    
    let mut fz = fs::File::create("mutated.rs").unwrap();
    let sst = quote!(#syntax_tree).to_string().as_bytes();
    fz.write_all(quote!(#syntax_tree).to_string().as_bytes());
    


    // If rustfmt doesn't exist, install it
    Command::new("rustup")
            .arg("component")
            .arg("add")
            .arg("rustfmt")
            .spawn()
            .expect("rustup command failed to start");
    
    // Format mutated source code.
    Command::new("rustfmt")
            .arg("mutated.rs")
            .spawn()
            .expect("rustfmt command failed to start");
    return "hello".to_string(); // temporary return value
}
