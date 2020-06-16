#![recursion_limit="1048576"]
mod cov_test;
mod mut_test;
mod mut_gen;
mod report_gen;
mod utils;
use mut_test::TestResult;
use std::env;
use std::fs;
use syn::Result;
use mut_gen::MutantInfo;

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

    let tarpaulin_report_path;
    //if args.len() < 3 {
        tarpaulin_report_path = cov_test::run_test(args[1].clone()).unwrap();
    //} else {
    //    tarpaulin_report_path = args[2].clone();
    //}
    
    let trace_info = cov_test::parse(&tarpaulin_report_path).expect("tarpaulin report parsing error");
    let mut mutated_result: Vec<MutantInfo> = Vec::new();
    let trace_iter = trace_info.iter();
    for trace in trace_iter {
        let path = &trace.path;
        let line_list = &trace.traces;
        if path.contains("combinations"){
            println!("Generating Mutants for {}, {:?}", path, line_list);
            mutated_result.append(&mut mut_gen::mutate(path.clone(), line_list.clone()));
        }  
        //if mutated_result.len() > 20 {
        //    break;
        //}
    }

    let result = mut_test::mut_test(args[1].clone(), mutated_result);
    for _x in result.iter() {
        println!("{}, {} {} {} {}", _x.1, _x.0.source_name, _x.0.file_name, _x.0.target_line, _x.0.mutation);
    }
    report_gen::make_report(args[1].clone(), result);
}