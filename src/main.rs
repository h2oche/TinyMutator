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
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("No Specified Project Directory or Line");
    }
    // let tarpaulin_report_path = cov_test::run_test(args[1].clone()).unwrap();
    // let trace_info = cov_test::parse(&tarpaulin_report_path).expect("tarpaulin report parsing error");
    // println!("{:?}", trace_info);

    let mutated_result = mut_gen::mutate(args[1].clone(), args[2].clone().parse().unwrap());
        for _x in mutated_result.iter() {
       println!("{} {} {} {}",_x.source_name, _x.file_name, _x.target_line, _x.mutation);
    }

    let test_list = vec![String::from("combinations_no_collect")];
    mut_test::run_mut_test(&args[1], Some(test_list));
}