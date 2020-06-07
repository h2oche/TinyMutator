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
    collections::HashSet,
    rc::{Rc, Weak},
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
/*fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}*/

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

struct Pos {
    start_line: usize,
    start_column: usize,
    end_line: usize,
    end_column: usize,
    start_type: Vec<String>,
}

struct BinOpVisitor<'ast> {
    BinOps: Vec<Pos>,
    // BinOps: Vec<&'ast mut syn::BinOp>,
    Line: usize,
    Column: usize,
    Covered: HashSet<usize>,
    Prevsize: usize,
    Strong: Rc<File>,
    Weak: Weak<File>,
    Ptr: &'ast File,
    Search: bool,
    Target: Pos,
    
}

// impl<'ast> Young for BinOpVisitor<'ast> {
//     fn woo(&mut self, node: &mut syn::BinOp, tree: &mut File) {
//         self.visit_bin_op_mut(node: &mut syn::BinOp);
//     }
// }


impl<'ast> VisitMut for BinOpVisitor<'ast> {
    fn visit_bin_op_mut(&mut self, node: &mut syn::BinOp) {
        let start = node.span().start();
        let end = node.span().end();
        let mut isTarget = true;
        if self.Search {
            if start.line <= self.Line && self.Line <= end.line {
                let mut arith = vec![String::from("+"), String::from("-"), String::from("*"), String::from("/"), String::from("%")];
                let mut bit = vec![String::from("^"), String::from("&"), String::from("|")];
                let mut relational = vec![String::from("=="), String::from("<"), String::from("<="), String::from("!=") ,String::from(">="), String::from(">")];
                let type_str = match node {
                    // Arithmetic Operators
                    syn::BinOp::Add(Add) => {arith.remove(0); arith},
                    syn::BinOp::Sub(Sub) => {arith.remove(1); arith},
                    syn::BinOp::Mul(Star) => {arith.remove(2); arith},
                    syn::BinOp::Div(Div) => {arith.remove(3); arith},
                    syn::BinOp::Rem(Rem) => {arith.remove(4); arith},
                    // Bitwise Operators
                    syn::BinOp::BitXor(Caret) => {bit.remove(0); bit},
                    syn::BinOp::BitAnd(And) => {bit.remove(1); bit},
                    syn::BinOp::BitOr(Or) => {bit.remove(2); bit},
                    // Relational Operators
                    syn::BinOp::Eq(EqEq) => {relational.remove(0); relational},
                    syn::BinOp::Lt(Lt) => {relational.remove(1); relational},
                    syn::BinOp::Le(Le) => {relational.remove(2); relational},
                    syn::BinOp::Ne(Ne) => {relational.remove(3); relational},
                    syn::BinOp::Ge(Ge) => {relational.remove(4); relational},
                    syn::BinOp::Gt(Gt) => {relational.remove(5); relational},
                    
                    _ => vec![String::from("+")],
                };
                let node_pos= Pos {
                    start_line : start.line,
                    start_column: start.column,
                    end_line : end.line,
                    end_column: end.column,
                    start_type: type_str,
                };
                self.BinOps.push(node_pos);    
            }
            
            visit_mut::visit_bin_op_mut(self, node);
        } else {
            if start.line == self.Target.start_line &&
            start.column == self.Target.start_column &&
            end.line == self.Target.end_line &&
            end.column == self.Target.end_column &&
            self.Target.start_type.len() > 0  {
                let _op = self.Target.start_type.pop().unwrap();
                match _op.as_str() {
                    // Arithmetic Operators
                    "+" => {*node = syn::BinOp::Add(syn::token::Add(node.span().clone()));},
                    "-" => {*node = syn::BinOp::Sub(syn::token::Sub(node.span().clone()));},
                    "*" => {*node = syn::BinOp::Mul(syn::token::Star(node.span().clone()));},
                    "/" => {*node = syn::BinOp::Div(syn::token::Div(node.span().clone()));},
                    "%" => {*node = syn::BinOp::Rem(syn::token::Rem(node.span().clone()));},
                    // Bitwise Operators
                    "^" => {*node = syn::BinOp::BitXor(syn::token::Caret(node.span().clone()));},
                    "&" => {*node = syn::BinOp::BitAnd(syn::token::And(node.span().clone()));},
                    "|" => {*node = syn::BinOp::BitOr(syn::token::Or(node.span().clone()));},
                    // Relational Operators
                    "==" => {*node = syn::BinOp::Eq(syn::token::EqEq(node.span().clone()));},
                    "<" => {*node = syn::BinOp::Lt(syn::token::Lt(node.span().clone()));},
                    "<=" => {*node = syn::BinOp::Le(syn::token::Le(node.span().clone()));},
                    "!=" => {*node = syn::BinOp::Ne(syn::token::Ne(node.span().clone()));},
                    ">=" => {*node = syn::BinOp::Ge(syn::token::Ge(node.span().clone()));},
                    ">" => {*node = syn::BinOp::Gt(syn::token::Gt(node.span().clone()));},

                    _ => {},
                }               
            }
        }
    }
}

pub fn mutate_file_by_line3(file: String, num_line: usize) -> String {
    let args: Vec<String> = env::args().collect();
    let example_source = fs::read_to_string(&file).expect("Something went wrong reading the file");
    let mut for_strong = syn::parse_file(&example_source).unwrap();
    let for_weak = syn::parse_file(&example_source).unwrap();
    let mut _binopvisitor = BinOpVisitor { BinOps: Vec::new(), Line: num_line, Column: 0, Covered: HashSet::new(), Prevsize: 0, Strong: Rc::new(for_strong), Weak: Weak::new(), Ptr: &for_weak, Search: true, Target:  Pos {
        start_line : 0,
        start_column: 0,
        end_line : 0,
        end_column: 0,
        start_type: vec![String::from("+")],
    }};
    // _binopvisitor.Weak = Rc::downgrade(&_binopvisitor.Strong);
    let mut syntax_tree = syn::parse_file(&example_source).unwrap();
        
    _binopvisitor.visit_file_mut(&mut syntax_tree);
    _binopvisitor.Search = false;
    // for pos in _binopvisitor.BinOps.clone().iter() {
    let mut idx = 0;
    for n in 0.._binopvisitor.BinOps.len() {
        let pos = &_binopvisitor.BinOps[n];
        // println!("{} {} {} {} {}", pos.start_line, pos.start_column, pos.end_line, pos.end_column, pos.start_type);
        _binopvisitor.Target = Pos {
            start_line: pos.start_line, 
            start_column : pos.start_column,
            end_line : pos.end_line,
            end_column : pos.end_column, 
            start_type : pos.start_type.clone(),
        };
        for m in 0..pos.start_type.len() {
            let mut new_syntax_tree = syn::parse_file(&example_source).unwrap();
            _binopvisitor.visit_file_mut(&mut new_syntax_tree);
            let mut fz = fs::File::create(format!("{}{}{}{}{}", "mutated",num_line,"_",idx,".rs")).unwrap();
        
            fz.write_all(quote!(#new_syntax_tree).to_string().as_bytes());
                
        
            // Format mutated source code.
            Command::new("rustfmt")
                    .arg(format!("{}{}{}{}{}", "mutated",num_line,"_",idx,".rs"))
                    .spawn()
                    .expect("rustfmt command failed to start");
            idx += 1;
        }
        
    }
    return "hello".to_string(); // temporary return value
}
