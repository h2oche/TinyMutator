extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
use quote::{quote};
use rand::{
    seq::SliceRandom,
    Rng,
};
use std::{
    env, fs,
    io::prelude::*,
    process::Command,
    cmp,
    vec,
    num::Wrapping,
};
use syn::{
    spanned::Spanned,
    visit_mut::{self, VisitMut},
    Expr,
    Stmt,
};

/**
 * Get smallest list of lines which is parsable with ast
*/
pub fn find_min_parsable_lines(splitted_file: Vec<&str>, num_line: usize) -> (usize, usize) {
    // println!("num line : {w}", num_line);
    if num_line >= splitted_file.len() {
        return (0, splitted_file.len());
    }
    for j in 1..cmp::max(splitted_file.len() - num_line + 1, num_line + 1) { // length
        for i in 0..j {
            if (num_line as i32) + (i as i32) - (j as i32) < 0 || (num_line as i32) + (i as i32) > (splitted_file.len() as i32) { continue; }
            // println!("{} {}", num_line + i - j, num_line + i);
            // println!("\n\n\n\n\n{:?}\n\n\n\n\n", &splitted_file[(num_line + i - j)..(num_line + i)].join("\r\n"));
            match syn::parse_str::<Stmt>(&(splitted_file[(num_line + i - j)..(num_line + i)].join("\r\n"))) {
                Ok(stmt) => {
                    return (num_line + i - j, num_line + i);
                },
                Err(error) => {},
            }
        }
    }
    return (0, splitted_file.len());
}

/**
 * Modify specific line of given file
*/
pub fn mutate_file_by_line(file: String, num_line: usize) -> String {
    // println!("filename : {}", file);
    // println!("line : {}", num_line);
    let mut constants = vec!["0", "1", "-1"];
    let mut void_functions: Vec<String> = Vec::new();

    let example_source = fs::read_to_string(file).expect("Something went wrong reading the file");
    let ast = syn::parse_file(&example_source);
    let lines = example_source.split("\r\n");
    let mut lines_vec: Vec<_> = example_source.split("\r\n").collect();

    // println!("{:?}", example_source);

    // preprocess(find all constants and functions in file, ...)
    for i in 0..lines_vec.len() {
        // let expr = syn::parse_str::<Stmt>(&lines_vec[i]);
        // println!("{:?}", expr);
        // println!("{:#?}", line);
        let (start, end) = find_min_parsable_lines(lines_vec.clone(), i + 1);
        let line_to_parse = lines_vec[start..end].join("\r\n");
        let expr = syn::parse_str::<Stmt>(&line_to_parse);
        // println!("\n\n\n{}, {}", start, end);
        // println!("{:?}", line_to_parse);
        // println!("{:?}", expr);
        // println!("{}\n\n\n", i);
        match expr {
            // statements are divided into 4 types(https://docs.rs/syn/1.0.30/syn/enum.Stmt.html)
            Ok(stmt) => {
                match stmt {
                    syn::Stmt::Item(item) => {
                        // constant statement, use statement, ...(listed here : https://docs.rs/syn/1.0.30/syn/enum.Item.html)
                        match item {
                            syn::Item::Const(itemConst) => {
                                // println!("{}", line);
                                // println!("{:#?}", &itemConst);
                                let mut const_expr: Vec<_> = lines_vec[i].split("=").collect();
                                // println!("{:?}\n\n\n", const_expr);
                                if const_expr.len() > 1 {
                                    constants.push(const_expr[1].trim_end_matches(";").trim());
                                }

                            },
                            syn::Item::Fn(itemFn) => { // get functions whose return type is not specified
                                // println!("\n\n\n{:?}", itemFn);
                                if itemFn.sig.output == syn::ReturnType::Default { // void return type
                                    // println!("\n\n\n{:?}", itemFn.sig.ident.to_string());
                                    void_functions.push(itemFn.sig.ident.to_string());
                                }
                            },
                            _ => {},
                        }
                    }
                    _ => { }
                }
            }
            Err(error) => { }
        }
    }

    // println!("{:?}", constants);
    // println!("{:?}", void_functions);

    let (start, end) = find_min_parsable_lines(lines_vec.clone(), num_line);
    let line_to_parse = lines_vec[start..end].join("\r\n");
    let expr_to_mutate = syn::parse_str::<Stmt>(&line_to_parse);
    // println!("\n\n\n{:?}\n\n\n", line_to_parse);
    // println!("{:?}", expr_to_mutate);
    match expr_to_mutate {
        Ok(stmt) => {
            // println!("{:#?}", stmt);
            match stmt {
                syn::Stmt::Local(local) => {
                    // let binding(negation, arithmetic operator deletion)
                    let mut let_binding_expr: Vec<_> = line_to_parse.split("=").collect();
                    let mut random_number = rand::thread_rng();
                    // println!("Integer: {}", random_number.gen_range(0, 2));
                    match random_number.gen_range(0, 2) {
                        0 => { // negation
                            let let_binding_string =
                                let_binding_expr[0].to_string() + &("= ".to_string()) + &("-".to_string()) + let_binding_expr[1].trim();
                            lines_vec[num_line - 1] = &let_binding_string;
                            // println!("{:?}", lines_vec[num_line - 1]);
                            return lines_vec.join("\r\n");
                        }
                        1 => { // arithmetic operator deletion

                        }
                        _ => {}
                    }
                }
                syn::Stmt::Item(item) => {
                    // constant statement, use statement, ...(listed here : https://docs.rs/syn/1.0.30/syn/enum.Item.html)
                    match item {
                        syn::Item::Const(itemConst) => { // constant replacement
                            let mut new_constant_vec: Vec<_> = constants
                                .choose_multiple(&mut rand::thread_rng(), 1)
                                .collect();
                            let mut new_constant = new_constant_vec[0];
                            let mut const_expr: Vec<_> = line_to_parse.split("=").collect();
                            while const_expr[1].trim_end_matches(";").trim() == *new_constant {
                                new_constant_vec = constants
                                    .choose_multiple(&mut rand::thread_rng(), 1)
                                    .collect();
                                new_constant = new_constant_vec[0];
                            }
                            let tmp = const_expr[0].to_string();
                            let const_string =
                                tmp + &("= ".to_string()) + new_constant + &(";".to_string());
                            lines_vec[num_line - 1] = &const_string;
                            return lines_vec.join("\r\n");
                        }
                        _ => { () },
                    }
                }
                syn::Stmt::Expr(expr) => { () },
                syn::Stmt::Semi(expr, semi) => { 
                    match expr {
                        syn::Expr::Call(exprCall) => {
                            // println!("{:?}", *(exprCall.func));
                            match *(exprCall.func) {
                                syn::Expr::Path(exprPath) => {
                                    // println!("Wow~~~");
                                    // println!("{:?}", void_functions);
                                    // println!("Wow~~~");
                                    if void_functions.contains(&exprPath.path.segments[0].ident.to_string()) {
                                        // println!("{}", " ".repeat(line_to_parse.len() - line_to_parse.trim_start().len()) + &("// ".to_string()) + line_to_parse.trim_start());
                                        let void_method_call_mutator = " ".repeat(line_to_parse.len() - line_to_parse.trim_start().len()) + &("// ".to_string()) + line_to_parse.trim_start();
                                        lines_vec[num_line - 1] = &void_method_call_mutator;
                                        // lines_vec[num_line - 1] = &
                                        // println!("{:?}", exprPath.path.segments[0].ident.to_string());
                                        // println!("Wow~~~");
                                        return lines_vec.join("\r\n");
                                    }
                                },
                                _ => {},
                            }
                        },
                        _ => { () },
                    }
                },
                _ => { () },
            }
        }
        Err(error) => {
            // println!("{}", error);
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

struct BinOpVisitor {
    vec_pos: Vec<Pos>,
    
    // vec_pos: Vec<&'ast mut syn::BinOp>,
    struct_line: usize,
    struct_column: usize,
    
    search: bool,
    target: Pos,
    
}

// impl<'ast> Young for BinOpVisitor<'ast> {
//     fn woo(&mut self, node: &mut syn::BinOp, tree: &mut File) {
//         self.visit_bin_op_mut(node: &mut syn::BinOp);
//     }
// }

impl<'ast> VisitMut for BinOpVisitor {
    fn visit_bin_op_mut(&mut self, node: &mut syn::BinOp) {
        let start = node.span().start();
        let end = node.span().end();
        if self.search {
            if start.line <= self.struct_line && self.struct_line <= end.line {
                let mut arith = vec![String::from("+"), String::from("-"), String::from("*"), String::from("/"), String::from("%")];
                let mut bit = vec![String::from("^"), String::from("&"), String::from("|")];
                let mut relational = vec![String::from("=="), String::from("<"), String::from("<="), String::from("!=") ,String::from(">="), String::from(">")];
                let type_str = match node {
                    // Arithmetic Operators
                    syn::BinOp::Add(_add) => {arith.remove(0); arith},
                    syn::BinOp::Sub(_sub) => {arith.remove(1); arith},
                    syn::BinOp::Mul(_star) => {arith.remove(2); arith},
                    syn::BinOp::Div(_div) => {arith.remove(3); arith},
                    syn::BinOp::Rem(_rem) => {arith.remove(4); arith},
                    // Bitwise Operators
                    syn::BinOp::BitXor(_caret) => {bit.remove(0); bit},
                    syn::BinOp::BitAnd(_and) => {bit.remove(1); bit},
                    syn::BinOp::BitOr(_or) => {bit.remove(2); bit},
                    // Relational Operators
                    syn::BinOp::Eq(_eqeq) => {relational.remove(0); relational},
                    syn::BinOp::Lt(_lt) => {relational.remove(1); relational},
                    syn::BinOp::Le(_le) => {relational.remove(2); relational},
                    syn::BinOp::Ne(_ne) => {relational.remove(3); relational},
                    syn::BinOp::Ge(_ge) => {relational.remove(4); relational},
                    syn::BinOp::Gt(_gt) => {relational.remove(5); relational},
                    
                    _ => vec![String::from("+")],
                };
                let node_pos= Pos {
                    start_line : start.line,
                    start_column: start.column,
                    end_line : end.line,
                    end_column: end.column,
                    start_type: type_str,
                };
                self.vec_pos.push(node_pos);    
            }
            
            visit_mut::visit_bin_op_mut(self, node);
        } else {
            if start.line == self.target.start_line &&
            start.column == self.target.start_column &&
            end.line == self.target.end_line &&
            end.column == self.target.end_column &&
            self.target.start_type.len() > 0  {
                let _op = self.target.start_type.pop().unwrap();
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

    fn visit_expr_binary_mut(&mut self, node: &mut syn::ExprBinary) {
        let start = node.span().start();
        let end = node.span().end();
        if self.search {
            if start.line <= self.struct_line && self.struct_line <= end.line {
                let type_str = vec![String::from("l"), String::from("r")];
                
                let node_pos= Pos {
                    start_line : start.line,
                    start_column: start.column,
                    end_line : end.line,
                    end_column: end.column,
                    start_type: type_str,
                };
                self.vec_pos.push(node_pos);    
            }
            
            visit_mut::visit_expr_binary_mut(self, node);
        } else {
            if start.line == self.target.start_line &&
            start.column == self.target.start_column &&
            end.line == self.target.end_line &&
            end.column == self.target.end_column &&
            self.target.start_type.len() > 0  {
                let _op = self.target.start_type.pop().unwrap();
                // let digits = Lit::Int.base10_digits();
                
                // let unsuffixed: LitInt = syn::parse_str(digits).unwrap();
                // *node.left = parse_quote!(i32::1!(#unsuffixed));
                match _op.as_str() {
                    // Arithmetic Operators
                    // "al" => {*node = syn::BinOp::Add(syn::token::Add(node.span().clone()));},
                    "l" => {*(node.left) = syn::parse_str::<Expr>("1").unwrap(); node.op = syn::BinOp::Mul(syn::token::Star(node.span().clone()));},
                    "r" => {*(node.right) = syn::parse_str::<Expr>("1").unwrap(); node.op = syn::BinOp::Mul(syn::token::Star(node.span().clone()));},

                    
                    // _ => {node.op = syn::BinOp::Mul(syn::token::Star(node.span().clone()));},
                    
                    // _ => {*node.left = Expr::Lit{attrs : vec![], lit : Lit::Int(syn::Litinit()) };}
                    _ => {},
                }
            }
        }
    }
}

pub fn mutate_file_by_line3(file: String, num_line: usize) -> String {
    let example_source = fs::read_to_string(&file).expect("Something went wrong reading the file");
    
    let mut _binopvisitor = BinOpVisitor { vec_pos: Vec::new(), struct_line: num_line, struct_column: 0, search: true, target:  Pos {
        start_line : 0,
        start_column: 0,
        end_line : 0,
        end_column: 0,
        start_type: vec![String::from("+")],
    }};
    let mut syntax_tree = syn::parse_file(&example_source).unwrap();
        
    _binopvisitor.visit_file_mut(&mut syntax_tree);
    _binopvisitor.search = false;
    // for pos in _binopvisitor.vec_pos.clone().iter() {
    let mut idx = 0;
    for _n in 0.._binopvisitor.vec_pos.len() {
        let pos = &_binopvisitor.vec_pos[_n];
        // println!("{} {} {} {} {}", pos.start_line, pos.start_column, pos.end_line, pos.end_column, pos.start_type);
        _binopvisitor.target = Pos {
            start_line: pos.start_line, 
            start_column : pos.start_column,
            end_line : pos.end_line,
            end_column : pos.end_column, 
            start_type : pos.start_type.clone(),
        };
        for _m in 0..pos.start_type.len() {
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