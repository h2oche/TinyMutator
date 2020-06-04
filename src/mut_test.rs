//use std::io;
use std::process::Command;
use std::str;

pub fn run_test() {
    // Make a subprocess
    let _ = Command::new("cd").arg("..").output();
    let output = Command::new("cargo").arg("test").output().unwrap();
    println!("=====");
    let s: &str = match str::from_utf8(output.stdout.as_slice()) {
        Ok(v) => v,
        Err(_e) => panic!("Not"),
    };
    println!("{}", s);
    let string: String = String::from(s);
    println!("=====");
    // Run 'cargo test'
    // Parse the result
    //
}
