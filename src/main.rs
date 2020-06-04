mod cov_test;
mod utils;
use std::env;
use std::fs;
use syn::Result;

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
    if args.len() < 2 {
        panic!("No Specified Project Directory");
    }
    let tarpaulin_report_path = cov_test::run_test(args[1].clone()).unwrap().as_str();

    let trace_info =
        cov_test::parse(tarpaulin_report).expect("tarpaulin report parsing error");
    println!("{:?}", trace_info);
    // print_ast_from_file(); // cargo run ./src/examples/guessing_game.rs
}