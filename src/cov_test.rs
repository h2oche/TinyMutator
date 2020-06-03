//use std::io;
use std::process::Command;
use std::str;
use std::path::PathBuf;
use std::fs::canonicalize;
use std::env::current_dir;

pub fn run_test(path: String) {
    // Get Absolute path
    let path = PathBuf::from(&path);
    let absolute_path = match canonicalize(path) {
        Ok(v)   => v,
        Err(_) => panic!("Failed to Get Absolute Path")
    };
    let absolute_path : &str = match absolute_path.to_str() {
        Some(x) => x,
        None    => panic!("Failed to Get Absolute Path String")
    };

    // Get current working directory
    let current_working_dir = match current_dir() {
        Ok(v)   => v,
        Err(_) => panic!("Failed to Get Current Working Directory")
    };
    let current_working_dir = match current_working_dir.to_str()  {
        Some(v)  => v,
        None     => panic!("Failed to Get Current Working Directory Path String")
    };

    // Make a subprocess & Run 'cargo test'
    let _ = Command::new("cargo").args(&["install", "cargo-tarpaulin"]).status(); // Install tarpaulin
    let mut shell = Command::new("cargo");
    shell.args(&["tarpaulin", "--out", "Json", "--output-dir", current_working_dir]);
    shell.current_dir(absolute_path);
    let output = shell.output().expect("failed to execute process");    // stdout

    // These are debug functions
    println!("=====");
    let s: &str = match str::from_utf8(output.stdout.as_slice()) {
        Ok(v) => v,
        Err(_e) => panic!("Not"),
    };
    println!("{}", s);
    println!("=====");

    // @TODO: Parse the result
}
