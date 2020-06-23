#![recursion_limit = "1048576"]
#![feature(rustc_private)]

#[macro_use]
extern crate lazy_static;
extern crate proc_macro;
extern crate proc_macro2;
extern crate regex;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_ty;
extern crate rustc_typeck;
extern crate syn;
extern crate timer;
extern crate chrono;
extern crate shared_child;

mod cov_test;
mod mut_gen;
mod mut_test;
mod report_gen;
mod utils;
use mut_gen::MutantInfo;
use std::env;
use std::fs;
use std::fs::File;
use syn::Result;
use std::time::SystemTime;

fn print_ast_from_file() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let ast = syn::parse_file(&content)?;
    for x in ast.items.iter() {
        println!(" > {:#?}", x)
    }
    println!("{} items", ast.items.len());
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect(); // args[1] : directory for mutating
    if args.len() < 2 {
        panic!("No Specified Project Directory or Line");
    }

    // run tarpaulin to get coverage report for given project
    let tarpaulin_report_path;
    if args.len() < 3 {
        tarpaulin_report_path = cov_test::run_test(args[1].clone()).unwrap();
    } else {
        tarpaulin_report_path = args[2].clone();
    }

    // parse tarpaulin report to get code coverage
    let trace_info =
        cov_test::parse(&tarpaulin_report_path).expect("tarpaulin report parsing error");
    let mut mutated_result: Vec<MutantInfo> = Vec::new();
    let trace_iter = trace_info.iter();
    let before_mutate = SystemTime::now();

    // try to mutate executed lines in all executed files 
    let mut cnt = 0;
    for trace in trace_iter {
        if cnt >= 4 {
            break;
        }
        let path = &trace.path;
        let line_list = &trace.traces;
        println!("Generating Mutants for {}, {:?}", path, line_list);
        mutated_result.append(&mut mut_gen::mutate(path.clone(), line_list.clone()));
        cnt += 1;
    }
    let mutation_time = before_mutate.elapsed().unwrap().as_secs();
    println!("Time Elapsed for Generating Mutants : {}s", mutation_time);

    // run mutation testing
    let before_mut_test = SystemTime::now();
    let result = mut_test::mut_test(args[1].clone(), mutated_result);
    let mutation_time = before_mut_test.elapsed().unwrap().as_secs();
    println!("Time Elapsed for Testing Mutants : {}s", mutation_time);
    
    for _x in result.iter() {
        println!(
            "{}, {} {} {} {}",
            _x.1, _x.0.source_name, _x.0.file_name, _x.0.target_line, _x.0.mutation
        );
    }

    // make report for mutation testing
    report_gen::make_report(args[1].clone(), result);
}
