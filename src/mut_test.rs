//use std::io;
#[path = "./utils.rs"]
mod utils;
use std::process::Command;
use std::str;

/**
 * Do 'cargo test' with the given list of tests.
 * Do all available tests whenever None is given.
 */ 
pub fn run_mut_test(path: String, tests: Option<Vec<String>>) {
    // Get Absolute path
    let absolute_path = utils::get_abs_path(&path);

    // Make a subprocess & Run 'cargo test'
    let mut shell = Command::new("cargo");
    let mut arg_vec : Vec<String> = match tests{
        Some(v) => v,
        None => Vec::new()
    };
    arg_vec.insert(0, String::from("test"));
    shell.args(&arg_vec);
    shell.current_dir(absolute_path);
    let output = match shell.output() {
        Ok(v) => v,
        Err(_) => panic!("Cargo Test Failed"),
    };
    let output = String::from_utf8(output.stdout).unwrap();
    let parsed = parse_result(output).unwrap();
    println!("{:?}", parsed);
    return;
}

/**
 * Return the list of the test names and their results
 */
pub fn parse_result(result: String) -> Option<Vec<(String, bool)>>{
    let v : Vec<&str> = result.split('\n').collect();
    let v_iter = v.iter();
    let mut result : Vec<(String, bool)> = Vec::new();
    for s in v_iter {
        if !s.contains("test result:") && !s.contains("running") && s.contains("test") {
            if s.contains("(line") {
                let vs : Vec<&str> = s.split(" ").collect();
                if vs[7].contains("ok") {
                    result.push((String::from(vs[3]), true));
                } else {
                    result.push((String::from(vs[3]), false));
                }
            } else {
                let vs : Vec<&str> = s.split(" ").collect();
                if vs[3].contains("ok") {
                    result.push((String::from(vs[1]), true));
                } else {
                    result.push((String::from(vs[1]), false));
                }
            }
            
        }
    }
    return Some(result);
}
