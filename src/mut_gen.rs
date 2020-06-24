use quote::quote;
use rand::{seq::SliceRandom, Rng};
use regex::Regex;
use rustc_errors::registry;
use rustc_hir::intravisit::{self, NestedVisitorMap, Visitor};
use rustc_hir::Expr as HirExpr;
use rustc_middle::hir::map::Map;
use rustc_middle::ty::TyCtxt;
use rustc_session::config::{self, CrateType};
use rustc_span::{source_map, Span};
use std::{cmp, fs, fs::File, io::Write, path, process, process::Command, str, vec};
use syn::{
    spanned::Spanned,
    visit_mut::{self, VisitMut},
    Expr, Pat, Stmt,
};

pub struct OptionCollector<'tcx> {
    tcx: TyCtxt<'tcx>,
    spans: Vec<Span>,
}
impl<'tcx> Visitor<'tcx> for OptionCollector<'tcx> {
    type Map = Map<'tcx>;
    fn nested_visit_map(&mut self) -> NestedVisitorMap<Self::Map> {
        NestedVisitorMap::All(self.tcx.hir())
    }
    fn visit_expr(&mut self, expr: &'tcx HirExpr) {
        let table = self.tcx.typeck_tables_of(expr.hir_id.owner);

        if let Some(ty) = table.expr_ty_opt(expr) {
            let ty_str = ty.to_string();
            let span = expr.span;
            // check if ty is std::option::Option<T>
            lazy_static! {
                static ref OPTION_TYPE_RE: Regex =
                    Regex::new(r"^std::option::Option<.+>$").unwrap();
            }
            if OPTION_TYPE_RE.is_match(&ty_str) {
                println!("***********************************");
                println!("Span : {:#?}, Type: {}", span, ty_str);
                println!("***********************************");
                self.spans.push(span);
            }
            // println!("{:#?}", expr);
            intravisit::walk_expr(self, expr);
        }

        // match table.tainted_by_errors {
        //     Some(err) => {
        //         println!("ERROR");
        //         println!("{:#?}", err);
        //         intravisit::walk_expr(self, expr);
        //     }
        //     None => {
        //         let ty = table.expr_ty(expr);
        //         let ty_str = ty.to_string();
        //         let span = expr.span;
        //         // check if ty is std::option::Option<T>
        //         lazy_static! {
        //             static ref OPTION_TYPE_RE: Regex =
        //                 Regex::new(r"^std::option::Option<.+>$").unwrap();
        //         }
        //         if OPTION_TYPE_RE.is_match(&ty_str) {
        //             println!("***********************************");
        //             println!("Span : {:#?}, Type: {}", span, ty_str);
        //             println!("***********************************");
        //             self.spans.push(span);
        //         }
        //         // println!("{:#?}", expr);
        //         intravisit::walk_expr(self, expr);
        //     }
        // }
    }
}

impl<'tcx> OptionCollector<'tcx> {
    fn new(tcx: TyCtxt<'tcx>) -> OptionCollector<'tcx> {
        OptionCollector {
            tcx,
            spans: Vec::new(),
        }
    }
    // fn collect(&mut self, )
}

pub fn collect_option_expr_position(target_file: String) -> Vec<String> {
    let source_code =
        fs::read_to_string(&target_file.clone()).expect("Something went wrong reading the file");

    let out = process::Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .unwrap();
    let sysroot = str::from_utf8(&out.stdout).unwrap().trim();

    let mut spans = Vec::new();

    let crate_types = vec![CrateType::Staticlib];
    let config = rustc_interface::Config {
        opts: config::Options {
            maybe_sysroot: Some(path::PathBuf::from(sysroot)),
            crate_types,
            ..config::Options::default()
        },
        input: config::Input::Str {
            // name: source_map::FileName::Custom("main.rs".to_string()),
            name: source_map::FileName::Custom(target_file.clone()),
            input: source_code.to_owned(),
        },
        diagnostic_output: rustc_session::DiagnosticOutput::Default,
        crate_cfg: rustc_hash::FxHashSet::default(),
        input_path: None,
        output_dir: None,
        output_file: None,
        file_loader: None,
        stderr: None,
        crate_name: None,
        lint_caps: rustc_hash::FxHashMap::default(),
        register_lints: None,
        override_queries: None,
        registry: registry::Registry::new(&rustc_error_codes::DIAGNOSTICS),
    };

    rustc_interface::run_compiler(config, |compiler| {
        compiler.enter(|queries| {
            // let parse = queries.parse().unwrap().take();
            // println!("=====================================");
            // println!("{:#?}", parse);
            // println!("=====================================");

            // Analyze the crate and inspect the types under the cursor.
            queries.global_ctxt().unwrap().take().enter(|tcx| {
                // Every compilation contains a single crate.
                let krate = tcx.hir().krate();
                let mut collector = OptionCollector::new(tcx);
                intravisit::walk_crate(&mut collector, krate);
                // println!("{:#?}", collector.spans);
                for span in collector.spans {
                    let span_str = format!("{:?}", span);
                    spans.push(span_str);
                    // let lo = span.lo();
                    // let hi = span.hi();
                    // println!("lo : {:?}", lo);
                    // println!("hi : {:?}", hi);
                }
                // spans.append(&mut collector.spans);
            })
        });
    });
    // print spans
    // println!("{:#?}", spans);

    spans
}

/**
 * Get smallest list of lines which is parsable with ast
*/
pub fn find_min_parsable_lines(splitted_file: Vec<&str>, num_line: usize) -> (usize, usize) {
    if num_line >= splitted_file.len() {
        return (0, splitted_file.len());
    }
    // for j in 1..cmp::max(splitted_file.len() - num_line + 1, num_line + 1) {
    for j in 1..10 {
        for i in 0..j {
            // j is number of lines
            if (num_line as i32) + (i as i32) - (j as i32) < 0
                || (num_line as i32) + (i as i32) > (splitted_file.len() as i32)
            {
                continue;
            }
            match syn::parse_str::<Stmt>(
                &(splitted_file[(num_line + i - j)..(num_line + i)].join("\n")),
            ) {
                Ok(stmt) => {
                    return (num_line + i - j, num_line + i);
                }
                Err(error) => {}
            }
        }
    }
    return (0, 0); // not parsable within 10 lines
}

pub fn get_constants_and_void_functions(file: String) -> (Vec<String>, Vec<String>) {
    let mut constants: Vec<String> = vec!["0".to_string(), "1".to_string(), "-1".to_string()];
    let mut void_functions: Vec<String> = Vec::new();

    let example_source = fs::read_to_string(file).expect("Something went wrong reading the file");
    let lines = example_source.split("\r\n");
    let mut lines_vec: Vec<_> = example_source.split("\r\n").collect();

    // preprocess(find all constants and void functions in file, ...)
    for i in 0..lines_vec.len() {
        let (start, end) = find_min_parsable_lines(lines_vec.clone(), i + 1);
        let line_to_parse = lines_vec[start..end].join("\r\n");
        let expr = syn::parse_str::<Stmt>(&line_to_parse);
        match expr {
            // statements are divided into 4 types(https://docs.rs/syn/1.0.30/syn/enum.Stmt.html)
            Ok(stmt) => {
                match stmt {
                    syn::Stmt::Item(item) => {
                        // constant statement, use statement, ...(listed here : https://docs.rs/syn/1.0.30/syn/enum.Item.html)
                        match item {
                            syn::Item::Const(item_const) => {
                                let mut const_expr: Vec<_> = lines_vec[i].split("=").collect();
                                if const_expr.len() > 1 {
                                    constants.push(
                                        const_expr[1].trim_end_matches(";").trim().to_string(),
                                    );
                                }
                            }
                            syn::Item::Fn(itemFn) => {
                                // get functions whose return type is not specified
                                if itemFn.sig.output == syn::ReturnType::Default {
                                    // void return type
                                    void_functions.push(itemFn.sig.ident.to_string());
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Err(error) => {}
        }
    }
    return (constants, void_functions);
}

/**
 * Modify specific line of given file using string modification
*/
pub fn mutate_file_by_string(
    file: String,
    num_line: usize,
    constants: Vec<String>,
    void_functions: Vec<String>,
) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let example_source = fs::read_to_string(file).expect("Something went wrong reading the file");
    let lines = example_source.split("\n");
    let mut lines_vec: Vec<_> = example_source.split("\n").collect();
    let (start, end) = find_min_parsable_lines(lines_vec.clone(), num_line);
    if (start, end) == (0, 0) {
        return result;
    }

    let line_to_parse = lines_vec[start..end].join("\n");
    let expr_to_mutate = syn::parse_str::<Stmt>(&line_to_parse);
    match expr_to_mutate {
        Ok(stmt) => {
            // println!("{:#?}", stmt);
            match stmt {
                syn::Stmt::Local(local) => {
                    // let binding(negation, arithmetic operator deletion)
                    let mut let_binding_expr: Vec<_> = line_to_parse.split("=").collect();

                    // negation
                    let let_binding_string = let_binding_expr[0].to_string()
                        + &("= ".to_string())
                        + &("-(".to_string())
                        + let_binding_expr[1].trim().trim_end_matches(";")
                        + &(");".to_string());

                    let mut result_lines_vec = lines_vec.clone();
                    for i in start..end {
                        result_lines_vec.remove(start);
                    }
                    result_lines_vec.insert(start, &let_binding_string);
                    result.push(String::from("negation:") + &result_lines_vec.join("\n"));

                    // arithmetic operator deletion
                    let arithmetic_operators = vec![
                        "+".to_string(),
                        "-".to_string(),
                        "*".to_string(),
                        "/".to_string(),
                        "%".to_string(),
                    ];
                    let mut arithmetic_indices = Vec::new();
                    for (i, c) in lines_vec[start..end].join("\n").chars().enumerate() {
                        if arithmetic_operators.contains(&&c.to_string()) {
                            arithmetic_indices.push(i);
                        }
                    }
                    for index in arithmetic_indices {
                        let tmp = lines_vec[start..end].join("\n")[..(index as usize)]
                            .trim_end()
                            .to_string()
                            + &(";".to_string());

                        let mut result_lines_vec = lines_vec.clone();
                        for i in start..end {
                            result_lines_vec.remove(start);
                        }
                        result_lines_vec.insert(start, &tmp);
                        result_lines_vec.insert(start, &let_binding_string);
                        result.push(
                            String::from("arithmetic_operator_deletion:")
                                + &result_lines_vec.join("\n"),
                        );
                    }
                }
                syn::Stmt::Item(item) => {
                    // constant statement, use statement, ...(listed here : https://docs.rs/syn/1.0.30/syn/enum.Item.html)
                    match item {
                        syn::Item::Const(item_const) => {
                            // constant replacement
                            for new_constant in constants {
                                let mut const_expr: Vec<_> = line_to_parse.split("=").collect();
                                if const_expr[1].trim_end_matches(";").trim() == new_constant {
                                    continue;
                                }
                                let tmp = const_expr[0].to_string();
                                let const_string =
                                    tmp + &("= ".to_string()) + &new_constant + &(";".to_string());
                                let mut result_lines_vec = lines_vec.clone();
                                for i in start..end {
                                    result_lines_vec.remove(start);
                                }
                                result_lines_vec.insert(start, &const_string);
                                result.push(
                                    String::from("constant_replacement:")
                                        + &result_lines_vec.join("\n"),
                                );
                            }
                        }
                        _ => {}
                    }
                }
                syn::Stmt::Semi(expr, semi) => {
                    match expr {
                        syn::Expr::Call(expr_call) => {
                            match *(expr_call.func) {
                                syn::Expr::Path(expr_path) => {
                                    // if void_functions
                                    //     .contains(&expr_path.path.segments[0].ident.to_string())
                                    // {
                                        let leading_spaces =
                                            line_to_parse.len() - line_to_parse.trim_start().len();
                                        // println!("{}", " ".repeat(line_to_parse.len() - line_to_parse.trim_start().len()) + &("// ".to_string()) + line_to_parse.trim_start());
                                        let void_method_call_string = " ".repeat(leading_spaces)
                                            + &("// ".to_string())
                                            + line_to_parse.trim_start();
                                        let mut result_lines_vec = lines_vec.clone();
                                        for i in start..end {
                                            result_lines_vec.remove(start);
                                        }
                                        result_lines_vec.insert(start, &void_method_call_string);
                                        result.push(
                                            String::from("void_method_call:")
                                                + &result_lines_vec.join("\n"),
                                        );
                                    // }
                                }
                                _ => {}
                            }
                        }
                        syn::Expr::Return(expr_return) => {
                            let mut return_expr = line_to_parse
                                .trim_start()
                                .trim_start_matches("return")
                                .trim()
                                .trim_end_matches(";");
                            let leading_spaces =
                                line_to_parse.len() - line_to_parse.trim_start().len();

                            // negation
                            let return_string = " ".repeat(leading_spaces)
                                + &("return ".to_string())
                                + &("-(".to_string())
                                + return_expr
                                + &(");".to_string());
                            // println!("{:?}", return_string);
                            let mut result_lines_vec = lines_vec.clone();
                            for i in start..end {
                                result_lines_vec.remove(start);
                            }
                            result_lines_vec.insert(start, &return_string);
                            result.push(String::from("negation:") + &result_lines_vec.join("\n"));

                            // arithmetic operator deletion
                            let arithmetic_operators = vec![
                                "+".to_string(),
                                "-".to_string(),
                                "*".to_string(),
                                "/".to_string(),
                                "%".to_string(),
                            ];
                            let mut arithmetic_indices = Vec::new();
                            for (i, c) in lines_vec[start..end].join("\n").chars().enumerate() {
                                if arithmetic_operators.contains(&&c.to_string()) {
                                    arithmetic_indices.push(i);
                                }
                            }
                            for index in arithmetic_indices {
                                let tmp = lines_vec[start..end].join("\n")[..(index as usize)]
                                    .trim_end()
                                    .to_string()
                                    + &(";".to_string());

                                let mut result_lines_vec = lines_vec.clone();
                                for i in start..end {
                                    result_lines_vec.remove(start);
                                }
                                result_lines_vec.insert(start, &tmp);
                                result.push(
                                    String::from("arithmetic_operator_deletion:")
                                        + &result_lines_vec.join("\n"),
                                );
                            }
                        }
                        _ => {}
                    }
                }
                syn::Stmt::Expr(expr) => (),
            }
        }
        Err(error) => {}
    }
    return result;
}

pub struct Pos {
    start_line: usize,
    start_column: usize,
    end_line: usize,
    end_column: usize,
    start_type: Vec<String>,
    mut_type: String,
}

struct BinOpVisitor {
    vec_pos: Vec<Pos>,
    struct_line: usize,
    struct_column: usize,
    search: bool,
    target: Pos,
}
use std::rc::Rc;
impl<'ast> VisitMut for BinOpVisitor {
    fn visit_expr_match_mut(&mut self, node: &mut syn::ExprMatch) {
        let start = node.span().start();
        let end = node.span().end();
        if self.search {
            if start.line <= self.struct_line && self.struct_line <= end.line && node.arms.len() > 0
            {
                let ll = node.arms.len();
                let mut type_str = Vec::new();
                for _i in 0..ll {
                    for _j in 0.._i {
                        type_str.push(format!("{},{}", _j, _i));
                    }
                }
                let node_pos = Pos {
                    start_line: start.line,
                    start_column: start.column,
                    end_line: end.line,
                    end_column: end.column,
                    start_type: type_str,
                    mut_type: String::from("match"),
                };
                self.vec_pos.push(node_pos);
            }
            for a in &mut node.attrs {
                self.visit_attribute_mut(a);
            }
            self.visit_expr_mut(&mut *node.expr);
            for a in &mut node.arms {
                self.visit_arm_mut(a);
            }
        } else {
            if start.line == self.target.start_line
                && start.column == self.target.start_column
                && end.line == self.target.end_line
                && end.column == self.target.end_column
                && self.target.start_type.len() > 0
            {
                let _op = self.target.start_type.pop().unwrap();
                let tokens: Vec<&str> = _op.split(",").collect();
                let _x = tokens[0].parse::<usize>().unwrap();
                let _y = tokens[1].parse::<usize>().unwrap();

                let temp = node.arms[_x].pat.clone();
                node.arms[_x].pat = node.arms[_y].pat.clone();
                node.arms[_y].pat = temp;
            }
        }
    }
    fn visit_bin_op_mut(&mut self, node: &mut syn::BinOp) {
        let start = node.span().start();
        let end = node.span().end();
        if self.search {
            if start.line <= self.struct_line && self.struct_line <= end.line {
                let mut arith = vec![
                    String::from("+"),
                    String::from("-"),
                    String::from("*"),
                    String::from("/"),
                    String::from("%"),
                ];
                let mut bit = vec![String::from("^"), String::from("&"), String::from("|")];
                let mut relational = vec![
                    String::from("=="),
                    String::from("<"),
                    String::from("<="),
                    String::from("!="),
                    String::from(">="),
                    String::from(">"),
                ];
                let mut _muttype = String::from("");
                let type_str = match node {
                    // Arithmetic Operators
                    syn::BinOp::Add(_add) => {
                        _muttype = String::from("arithmetic_operator_replacement");
                        arith.remove(0);
                        arith
                    }
                    syn::BinOp::Sub(_sub) => {
                        _muttype = String::from("arithmetic_operator_replacement");
                        arith.remove(1);
                        arith
                    }
                    syn::BinOp::Mul(_star) => {
                        _muttype = String::from("arithmetic_operator_replacement");
                        arith.remove(2);
                        arith
                    }
                    syn::BinOp::Div(_div) => {
                        _muttype = String::from("arithmetic_operator_replacement");
                        arith.remove(3);
                        arith
                    }
                    syn::BinOp::Rem(_rem) => {
                        _muttype = String::from("arithmetic_operator_replacement");
                        arith.remove(4);
                        arith
                    }
                    // Bitwise Operators
                    syn::BinOp::BitXor(_caret) => {
                        _muttype = String::from("bitwise_operator_replacement");
                        bit.remove(0);
                        bit
                    }
                    syn::BinOp::BitAnd(_and) => {
                        _muttype = String::from("bitwise_operator_replacement");
                        bit.remove(1);
                        bit
                    }
                    syn::BinOp::BitOr(_or) => {
                        _muttype = String::from("bitwise_operator_replacement");
                        bit.remove(2);
                        bit
                    }
                    // Relational Operators
                    syn::BinOp::Eq(_eqeq) => {
                        _muttype = String::from("relational_operator_replacement");
                        relational.remove(0);
                        relational
                    }
                    syn::BinOp::Lt(_lt) => {
                        _muttype = String::from("relational_operator_replacement");
                        relational.remove(1);
                        relational
                    }
                    syn::BinOp::Le(_le) => {
                        _muttype = String::from("relational_operator_replacement");
                        relational.remove(2);
                        relational
                    }
                    syn::BinOp::Ne(_ne) => {
                        _muttype = String::from("relational_operator_replacement");
                        relational.remove(3);
                        relational
                    }
                    syn::BinOp::Ge(_ge) => {
                        _muttype = String::from("relational_operator_replacement");
                        relational.remove(4);
                        relational
                    }
                    syn::BinOp::Gt(_gt) => {
                        _muttype = String::from("relational_operator_replacement");
                        relational.remove(5);
                        relational
                    }
                    _ => vec![String::from("+")],
                };
                let node_pos = Pos {
                    start_line: start.line,
                    start_column: start.column,
                    end_line: end.line,
                    end_column: end.column,
                    start_type: type_str,
                    mut_type: _muttype,
                };
                self.vec_pos.push(node_pos);
            }
            visit_mut::visit_bin_op_mut(self, node);
        } else {
            if start.line == self.target.start_line
                && start.column == self.target.start_column
                && end.line == self.target.end_line
                && end.column == self.target.end_column
                && self.target.start_type.len() > 0
            {
                let _op = self.target.start_type.pop().unwrap();
                match _op.as_str() {
                    // Arithmetic Operators
                    "+" => {
                        *node = syn::BinOp::Add(syn::token::Add(node.span().clone()));
                    }
                    "-" => {
                        *node = syn::BinOp::Sub(syn::token::Sub(node.span().clone()));
                    }
                    "*" => {
                        *node = syn::BinOp::Mul(syn::token::Star(node.span().clone()));
                    }
                    "/" => {
                        *node = syn::BinOp::Div(syn::token::Div(node.span().clone()));
                    }
                    "%" => {
                        *node = syn::BinOp::Rem(syn::token::Rem(node.span().clone()));
                    }
                    // Bitwise Operators
                    "^" => {
                        *node = syn::BinOp::BitXor(syn::token::Caret(node.span().clone()));
                    }
                    "&" => {
                        *node = syn::BinOp::BitAnd(syn::token::And(node.span().clone()));
                    }
                    "|" => {
                        *node = syn::BinOp::BitOr(syn::token::Or(node.span().clone()));
                    }
                    // Relational Operators
                    "==" => {
                        *node = syn::BinOp::Eq(syn::token::EqEq(node.span().clone()));
                    }
                    "<" => {
                        *node = syn::BinOp::Lt(syn::token::Lt(node.span().clone()));
                    }
                    "<=" => {
                        *node = syn::BinOp::Le(syn::token::Le(node.span().clone()));
                    }
                    "!=" => {
                        *node = syn::BinOp::Ne(syn::token::Ne(node.span().clone()));
                    }
                    ">=" => {
                        *node = syn::BinOp::Ge(syn::token::Ge(node.span().clone()));
                    }
                    ">" => {
                        *node = syn::BinOp::Gt(syn::token::Gt(node.span().clone()));
                    }

                    _ => {}
                }
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct MutantInfo {
    pub source_name: String,
    pub file_name: String,
    pub target_line: usize,
    pub mutation: String,
}

pub fn mutate(file: String, num_line: Vec<usize>) -> Vec<MutantInfo> {
    let mut ret = Vec::new();
    let example_source =
        fs::read_to_string(&file.clone()).expect("Something went wrong reading the file");
    let syntax_tree_origin = syn::parse_file(&example_source).unwrap();
    let mut idx = 0;
    let num_line_iter = num_line.iter();
    for num_line in num_line_iter {
        let mut syntax_tree = syntax_tree_origin.clone();
        let mut _binopvisitor = BinOpVisitor {
            vec_pos: Vec::new(),
            struct_line: *num_line,
            struct_column: 0,
            search: true,
            target: Pos {
                start_line: 0,
                start_column: 0,
                end_line: 0,
                end_column: 0,
                start_type: vec![String::from("+")],
                mut_type: String::from(""),
            },
        };
        _binopvisitor.visit_file_mut(&mut syntax_tree);
        _binopvisitor.search = false;
        
        let mut cutted = file.clone().to_string();
        let mut using = &cutted[0..cutted.len() - 3];
        for _n in 0.._binopvisitor.vec_pos.len() {
            let pos = &_binopvisitor.vec_pos[_n];
            let _muttype = pos.mut_type.clone();
            _binopvisitor.target = Pos {
                start_line: pos.start_line,
                start_column: pos.start_column,
                end_line: pos.end_line,
                end_column: pos.end_column,
                start_type: pos.start_type.clone(),
                mut_type: _muttype.clone(),
            };
            for _m in 0..pos.start_type.len() {
                let mut new_syntax_tree = syn::parse_file(&example_source).unwrap();
                _binopvisitor.visit_file_mut(&mut new_syntax_tree);
                let mut fz = fs::File::create(format!(
                    "{}{}{}{}{}{}",
                    using.to_string().clone(),
                    "_",
                    num_line,
                    "_",
                    idx,
                    ".rs"
                ))
                .unwrap();
                fz.write_all(quote!(#new_syntax_tree).to_string().as_bytes());
                // Format mutated source code.
                Command::new("rustfmt")
                    .arg(format!(
                        "{}{}{}{}{}{}",
                        using.to_string().clone(),
                        "_",
                        num_line,
                        "_",
                        idx,
                        ".rs"
                    ))
                    .spawn()
                    .expect("rustfmt command failed to start");

                ret.push(MutantInfo {
                    source_name: using.to_string().clone(),
                    file_name: format!(
                        "{}{}{}{}{}{}",
                        using.to_string().clone(),
                        "_",
                        num_line,
                        "_",
                        idx,
                        ".rs"
                    ),
                    target_line: *num_line,
                    mutation: _muttype.clone(),
                });
                idx += 1;
            }
        }
        let woo = idx;

        let constants = get_constants_and_void_functions(file.clone()).0;
        let void_functions = get_constants_and_void_functions(file.clone()).1;

        let mutated_files_with_muttype: Vec<String> = mutate_file_by_string(
            file.clone(),
            num_line.clone(),
            constants.clone(),
            void_functions.clone(),
        )
        .clone();
        for mutated_file_with_muttype in mutated_files_with_muttype {
            let mutated_result: Vec<&str> = mutated_file_with_muttype.splitn(2, ":").collect();
            let mutated_file = mutated_result[1].clone();
            let _muttype = mutated_result[0].to_string().clone();

            let mut fz = fs::File::create(format!(
                "{}{}{}{}{}{}",
                using.to_string().clone(),
                "_",
                num_line,
                "_",
                idx,
                ".rs"
            ))
            .unwrap();
            fz.write_all(mutated_file.as_bytes());
            Command::new("rustfmt")
                .arg(format!(
                    "{}{}{}{}{}{}",
                    using.to_string().clone(),
                    "_",
                    num_line,
                    "_",
                    idx,
                    ".rs"
                ))
                .spawn()
                .expect("rustfmt command failed to start");

            ret.push(MutantInfo {
                source_name: using.to_string().clone(),
                file_name: format!(
                    "{}{}{}{}{}{}",
                    using.to_string().clone(),
                    "_",
                    num_line,
                    "_",
                    idx,
                    ".rs"
                ),
                target_line: *num_line,
                mutation: _muttype.clone(),
            });
            idx += 1;
            println!(
                "{}, {}, {}, {}",
                using.to_string().clone(),
                format!(
                    "{}{}{}{}{}{}",
                    using.to_string().clone(),
                    "_",
                    num_line,
                    "_",
                    idx,
                    ".rs"
                ),
                *num_line,
                _muttype.clone()
            );
        }

        println!(
            "For debug : using AST = {} mutants, using String = {} mutants",
            woo,
            idx - woo
        );
        if ret.len() > 20 {
            break;
        }
    }

    // add option mutator
    let option_positions = collect_option_expr_position(file.clone());
    // println!("option_pos : {:?}", option_positions);

    let original_source =
        fs::read_to_string(&file.clone()).expect("Something went wrong reading the file");

    let original_lines = original_source
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<_>>();
    let mut option_idx = idx;

    // println!("OPTIONS\n {:?}", option_positions);

    for op_pos in option_positions {
        // parse op_pos
        lazy_static! {
            static ref SPAN_EXTRACT_RE: Regex = Regex::new(
                r"^<.+>:(?P<start_line>\d+):(?P<start_col>\d+): (?P<end_line>\d+):(?P<end_col>\d+)"
            )
            .unwrap();
        }
        let captures = match SPAN_EXTRACT_RE.captures(&op_pos) {
            Some(v) => v,
            None => continue,
        };
        let start_line = captures["start_line"].parse::<usize>().unwrap() - 1;
        let start_col = captures["start_col"].parse::<usize>().unwrap() - 1;
        let end_line = captures["end_line"].parse::<usize>().unwrap() - 1;
        let end_col = captures["end_col"].parse::<usize>().unwrap() - 1;

        // println!(
        //     "OPTION {}:{}, {}:{}",
        //     start_line, start_col, end_line, end_col
        // );

        // ignore multi-line
        if start_line != end_line {
            continue;
        }

        let target_expr_str = original_lines[start_line][start_col..end_col].to_owned();
        // let target_expr_str = target_line[start_col..end_col].to_owned();
        let NONE_STR: &'static str = "None";

        // ignore `None`
        if target_expr_str == NONE_STR.clone() {
            continue;
        }

        // replace option pos to None
        let mut original_lines_copy = original_lines
            .iter()
            .map(|line| line.to_owned().clone())
            .collect::<Vec<_>>();

        let template_line = original_lines_copy[start_line].clone();
        let prefix = template_line[0..start_col].to_owned();
        let suffix = template_line[end_col..template_line.len()].to_owned();
        let mut mutated_line = String::new();
        mutated_line.push_str(&prefix);
        mutated_line.push_str(NONE_STR);
        mutated_line.push_str(&suffix);
        // println!("target_expr({}): {}", start_line + 1, target_expr_str);
        // println!("mutated : {}", mutated_line);

        original_lines_copy[start_line] = mutated_line;
        let mutated_source = original_lines_copy.join("\n");

        // save mutated file
        let cutted = file.clone().to_owned();
        let using = &cutted[0..cutted.len() - 3];
        let mutated_path = format!(
            "{}{}{}{}{}{}",
            using.clone(),
            "_",
            start_line,
            "_",
            option_idx,
            ".rs"
        );
        let mut fz = fs::File::create(mutated_path.clone()).unwrap();
        fz.write_all(mutated_source.as_bytes()).expect("write fail");
        // Format mutated source code
        Command::new("rustfmt")
            .arg(mutated_path.clone())
            .spawn()
            .expect("rustfmt command failed to start");
        // create mutant info
        ret.push(MutantInfo {
            source_name: using.to_owned().clone(),
            file_name: mutated_path.clone(),
            target_line: start_line,
            mutation: "option_to_none".to_owned(),
        });

        // println!(
        //     "OPTION MUTATOR CREATED\n{:#?}",
        //     MutantInfo {
        //         source_name: using.to_owned().clone(),
        //         file_name: mutated_path.clone(),
        //         target_line: start_line,
        //         mutation: "option_to_none".to_owned(),
        //     }
        // );

        // update option_idx
        option_idx += 1;
    }

    return ret;
}
