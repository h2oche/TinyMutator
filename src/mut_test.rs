//use std::io;
#[path = "./utils.rs"]
mod utils;
#[path = "./mut_gen.rs"]
mod mut_gen;
use std::process::Command;
use std::str;
use std::fs::File;
use std::io::prelude::*;

pub enum TestResult {
    Survived,
    Killed,
    CompileError,
}

/**
 * Run tests and check whether mutants are killed.
 * Return lists of mutants and its results.
 */
pub fn mut_test(path: String, list_of_mutants: Vec<&mut_gen::MutantInfo>) -> Vec<(String, TestResult)> {
    let mutants_iter = list_of_mutants.iter();
    let mut result : Vec<(String, TestResult)> = Vec::new();
    let original_test_result = run_mut_test(&path, None).unwrap();
    for mutant in mutants_iter {
        let original_source_code = replace_source(mutant);
        let mut_test_result = match run_mut_test(&path, None) {
            Some(v) => v,
            None => {
                result.push((mutant.file_name.clone(), TestResult::CompileError));
                restore_source(original_source_code);
                continue;
            }
        };
        if check_survive(&mut_test_result, &original_test_result){
            result.push((mutant.file_name.clone(), TestResult::Survived));
        } else {
            result.push((mutant.file_name.clone(), TestResult::Killed));
        }
        restore_source(original_source_code);
    }
    return result;
}

/**
 * Replace source file to mutated source file
 * Return original original source file (source_code, file_name)
 */
fn replace_source(mutate: &mut_gen::MutantInfo) -> (String, &String) {
    let current_dir = utils::get_cwd();
    let file_name : &String = &mutate.file_name;
    let original_file_name = &mutate.source_name;

    // Read original source code and save it.
    let mut f = File::open(String::new() + &current_dir + "/src/" + original_file_name).expect("File Not Found");
    let mut source_code = String::new();
    f.read_to_string(&mut source_code).expect("Failed to Read File");

    // Read mutated source code and write it to the original file.
    let mut m = File::open(String::new() + &current_dir + "/src/" + file_name).expect("File Not Found");
    let mut mutated_code = String::new();
    m.read_to_string(&mut mutated_code).expect("Failed to Read File");
    f.write(mutated_code.as_bytes()).expect("Failed to Write File");

    return (source_code, original_file_name);
}

/**
 * Restore original source file
 */
fn restore_source(original_source_code: (String, &String)) {
    let current_dir = utils::get_cwd();
    let source_code = original_source_code.0;
    let file_name = original_source_code.1;

    // Open original file and Write original source code.
    let mut f = File::open(current_dir + "/src/" + file_name).expect("File Not Found");
    f.write(source_code.as_bytes()).expect("Failed to Write File");
    return;
}

/**
 * Compare the result of mutation test
 * Return true if survived, false if killed
 */
pub fn check_survive(mut_test_result: &Vec<(String, bool)>, origin_test_result: &Vec<(String, bool)>) -> bool {
    // @TODO: This function is not tested(May have some bugs)
    if mut_test_result == origin_test_result{
        // @TODO: Check equivalence mutant
        return true;
    }
    return false;
}

/**
 * Do 'cargo test' with the given list of tests.
 * Do all available tests whenever None is given.
 * Return list of the test results
 */ 
pub fn run_mut_test(path: &String, tests: Option<Vec<String>>) -> Option<Vec<(String, bool)>> {
    // Get Absolute path
    let absolute_path = utils::get_abs_path(path);

    // Make a subprocess & Run 'cargo test'
    let mut shell = Command::new("cargo");
    let mut arg_vec : Vec<String> = match tests{
        Some(v) => v,
        None => Vec::new(),
    };
    arg_vec.insert(0, String::from("test"));
    shell.args(arg_vec);
    shell.current_dir(absolute_path);
    let output = match shell.output() {
        Ok(v) => v,
        Err(_) => panic!("Cargo Test Failed"),
    };
    if !output.status.success() {
        // Compile failed
        return None;
    }
    let output = String::from_utf8(output.stdout).unwrap();
    let parsed = parse_result(output).unwrap();
    //println!("{:?}", parsed);
    return Some(parsed);
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
