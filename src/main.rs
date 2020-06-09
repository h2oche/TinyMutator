mod cov_test;
mod mut_test;
mod mut_gen;
mod report_gen;
mod utils;
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
    let tarpaulin_report_path = cov_test::run_test(args[1].clone()).unwrap();

    let trace_info = cov_test::parse(&tarpaulin_report_path).expect("tarpaulin report parsing error");
    let mut mutated_result: Vec<MutantInfo> = Vec::new();
    let trace_iter = trace_info.iter();
    let mut counter = 0;
    for trace in trace_iter {
        let path = &trace.path;
        let line_list = &trace.traces;
        let line_iter = line_list.iter();
        for line in line_iter {
            println!("Generating Mutants for {}, {}", path, *line);
            mutated_result.append(&mut mut_gen::mutate(path.clone(), *line));
            counter += 1;
            if counter > 20 {
                break;
            }
        }
        if counter > 20 {
            break;
        }
    }

    let result = mut_test::mut_test(args[1].clone(), mutated_result);
    for _x in result.iter() {
        println!("{}, {} {} {} {}", _x.1, _x.0.source_name, _x.0.file_name, _x.0.target_line, _x.0.mutation);
    }
}