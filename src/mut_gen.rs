extern crate proc_macro;
extern crate syn;

use syn::Item;
use std::{fs, env, io::prelude::*, str::FromStr, vec};
use syn::{parse_macro_input, DeriveInput, Type, Expr, Result, Stmt, spanned::Spanned};
use proc_macro::{TokenStream, TokenTree};
use quote::quote_spanned;
use rand::seq::SliceRandom;

use quote::quote;
use syn::visit::{self, Visit};
use syn::{File, ItemFn};

use std::io::{self, BufRead};
use std::path::Path;
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
                    syn::Stmt::Local(local) => { // let binding
                        // println!(" > {:#?}", &local);
                        // println!("This is the case:");
                        // stmt.unwrap();
                        // print_type_of(&local.init);
                    }
                    syn::Stmt::Item(item) => { // constant and something
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


extern crate proc_macro2;
use proc_macro2::Span;

struct StmtVisitor<'ast> {
    statements: Vec<&'ast Stmt>,
}

impl<'ast> Visit<'ast> for StmtVisitor<'ast> {
    fn visit_stmt(&mut self, node: &'ast Stmt) {
        self.statements.push(node);
        visit::visit_stmt(self, node);
    }
}

struct ItemVisitor<'ast> {
    items: Vec<&'ast Item>,
}

impl<'ast> Visit<'ast> for ItemVisitor<'ast> {
    fn visit_item(&mut self, node: &'ast Item) {
        self.items.push(node);
        visit::visit_item(self, node);
    }
}
pub fn mutate_file_by_line3(file: String, num_line: usize) -> String {
    let args: Vec<String> = env::args().collect();
    let file2 = &args[1];
    let example_source = fs::read_to_string(file2).expect("Something went wrong reading the file");

    let file = syn::parse_file(&example_source).unwrap();
    println!("{:#?}", file);
    let mut _Stmtvisitor = StmtVisitor { statements: Vec::new() };
    _Stmtvisitor.visit_file(&file);
    for stmt in _Stmtvisitor.statements {
        let span = stmt.span();
        let start = span.start();
        let end = span.end();
        if start.line <= num_line && num_line <= end.line {
            println!("Find statement in line {} \n {:#?}", num_line, stmt);
        }
    }
    let mut _Itemvisitor = ItemVisitor { items: Vec::new() };
    _Itemvisitor.visit_file(&file);
    for item in _Itemvisitor.items {
        let span = item.span();
        let start = span.start();
        let end = span.end();
        if start.line <= num_line && num_line <= end.line {
            println!("Find item in line {} \n {:#?}", num_line, item);
        }
    }
    return "hello".to_string(); // temporary return value   
}