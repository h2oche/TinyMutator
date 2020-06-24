//use std::io;
//pub mod mut_gen;
use super::utils;
use super::mut_gen::MutantInfo;
use std::process::Command;
use std::str;
use std::fs::{remove_file, OpenOptions};
use std::io::prelude::*;
use std::fmt;
use shared_child::SharedChild;
//use mut_gen::MutantInfo;

#[derive(Debug)]
pub enum TestResult {
    Survived,
    Killed,
    CompileError,
    Timeout,
}

impl fmt::Display for TestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TestResult::Survived => write!(f, "Survived"),
            TestResult::Killed => write!(f, "Killed"),
            TestResult::CompileError => write!(f, "CompileError"),
            TestResult::Timeout => write!(f, "Timeout"),
        }
    }
}

impl Clone for TestResult {
    fn clone(&self) -> TestResult {
        match self {
            TestResult::Survived => TestResult::Survived,
            TestResult::Killed => TestResult::Killed,
            TestResult::CompileError => TestResult::CompileError,
            TestResult::Timeout =>  TestResult::Timeout,
        }
    }
}

/**
 * Run tests and check whether mutants are killed.
 * Return lists of mutants and its results.
 */
pub fn mut_test(path: String, list_of_mutants: Vec<MutantInfo>) -> Vec<(MutantInfo, TestResult)> {
    let mut result : Vec<(MutantInfo, TestResult)> = Vec::new();
    if list_of_mutants.len() == 0 {
        return result;
    }
    let mutants_iter = list_of_mutants.iter();
    println!("Generating Original Testing Data");
    let original_test_result = run_mut_test(&path, None, true).unwrap();
    
    for mutant in mutants_iter {
        let original_source_code = replace_source(&path, mutant);
        let mut_test_result = match run_mut_test(&path, None, false) {
            Ok(v) => v,
            Err(code) => {
                if code == 0 {
                    result.push((MutantInfo {
                        source_name: mutant.source_name.clone(),
                        file_name: mutant.file_name.clone(),
                        target_line: mutant.target_line,
                        mutation: mutant.mutation.clone(),
                    }, TestResult::CompileError));
                } else if code == 1 {
                    result.push((MutantInfo {
                        source_name: mutant.source_name.clone(),
                        file_name: mutant.file_name.clone(),
                        target_line: mutant.target_line,
                        mutation: mutant.mutation.clone(),
                    }, TestResult::Timeout));
                }
                restore_source(&path, original_source_code);
                continue;
            }
        };
        if check_survive(&mut_test_result, &original_test_result){
            result.push((MutantInfo {
                source_name: mutant.source_name.clone(),
                file_name: mutant.file_name.clone(),
                target_line: mutant.target_line,
                mutation: mutant.mutation.clone(),
                
            }, TestResult::Survived));
        } else {
            result.push((MutantInfo {
                source_name: mutant.source_name.clone(),
                file_name: mutant.file_name.clone(),
                target_line: mutant.target_line,
                mutation: mutant.mutation.clone(),
                
            }, TestResult::Killed));
        }
        restore_source(&path, original_source_code);
        // remove_file(mutant.file_name.clone());
    }
    // println!("{:?}", result);
    return result;
}

/**
 * Replace source file to mutated source file
 * Return original original source file (source_code, file_name)
 */
fn replace_source(path: &String, mutate: &MutantInfo) -> (String, String) {
    let file_name : &String = &mutate.file_name;
    let original_file_name = String::new() + &mutate.source_name + ".rs";
    println!("\nTesting : {}", file_name);
    println!("Original_file_name : {}", original_file_name);

    // Read original source code and save it.
    let mut f = OpenOptions::new().read(true).open(original_file_name.clone()).expect("File Not Found");
    let mut source_code = String::new();
    f.read_to_string(&mut source_code).expect("Failed to Read File");

    // Read mutated source code and write it to the original file.
    let mut m = OpenOptions::new().write(true).read(true).open(file_name).expect("File Not Found");
    let mut mutated_code = String::new();
    m.read_to_string(&mut mutated_code).expect("Failed to Read File");
    let _ = remove_file(file_name);

    let mut f = OpenOptions::new().write(true).truncate(true).read(true).open(original_file_name.clone()).expect("File Not Found");
    f.write(mutated_code.as_bytes()).expect("Failed to Write File");

    return (source_code, original_file_name.clone());
}

/**
 * Restore original source file
 */
fn restore_source(path: &String, original_source_code: (String, String)) {
    let source_code = original_source_code.0;
    let file_name = original_source_code.1;

    // Open original file and Write original source code.
    let mut f = OpenOptions::new().write(true).truncate(true).read(true).open(file_name).expect("File Not Found");
    //println!("{}", source_code);
    f.write(source_code.as_bytes()).expect("Failed to Write File");
    return;
}

/**
 * Compare the result of mutation test
 * Return true if survived, false if killed
 */
pub fn check_survive(mut_test_result: &Vec<(String, bool)>, origin_test_result: &Vec<(String, bool)>) -> bool {
    // @TODO: This function is not tested(May have some bugs)
    if mut_test_result.len() != origin_test_result.len(){
        //println!("length differ");
        return false;
    }
    for i in 0..mut_test_result.len(){
        if mut_test_result[i].0 != origin_test_result[i].0 {
            //println!("String differ {}", i);
            return false;
        }
        if mut_test_result[i].1 != origin_test_result[i].1 {
            //println!("Result differ {}", i);
            return false;
        } 
    }
    
    return true;
}

/**
 * Do 'cargo test' with the given list of tests.
 * Do all available tests whenever None is given.
 * Return list of the test results
 * Return Err code if failed
 * 0 : Compile Err
 * 1 : Timeout
 */ 
pub fn run_mut_test(path: &String, tests: Option<Vec<String>>, option: bool) -> Result<Vec<(String, bool)>, u32> {
    // Get Absolute path
    let absolute_path = utils::get_abs_path(path);

    // Make a subprocess & Run 'cargo test'
    let mut shell = Command::new("cargo");
    let mut arg_vec : Vec<String> = match tests {
        Some(v) => v,
        None => Vec::new(),
    };
    arg_vec.insert(0, String::from("test"));
    shell.args(arg_vec);
    shell.current_dir(absolute_path);
    let file = OpenOptions::new().write(true).truncate(true).create(true).open("_test_output").expect("File Not Found");
    shell.stdout(file);
    shell.stderr(std::process::Stdio::null());
    let child = SharedChild::spawn(&mut shell).expect("Failed to Run Tests");
    let child_arc = std::sync::Arc::new(child);
    let child_arc_clone = child_arc.clone();
    
    let timer = timer::Timer::new();
    let _guard = timer.schedule_with_delay(chrono::Duration::seconds(30), move || {
        let _ = child_arc_clone.kill();
    }); // Make the timeout(30s) for cargo test
    let output;
    if option {
        drop(_guard);   // This is for original test, no timeout
        output = child_arc.wait().unwrap();
    } else { 
        let wait = child_arc.wait();
        if wait.is_ok(){
            output = wait.unwrap();
        } else {
            return Err(1);
        }
        drop(_guard);
    }

    if !output.success() {
        if output.code().is_none() {
            return Err(1);
        }
        if output.code().unwrap() != 101 {
            // Compile failed
            println!("{}", output);
            return Err(0);
        }
        // Test failed
    }

    let mut output = String::new();
    let mut f = OpenOptions::new().read(true).open("_test_output").expect("File Not Found");
    f.read_to_string(&mut output).expect("Failed to Read File");
    let parsed = parse_result(output).unwrap();
    let _ = remove_file("_test_output");
    return Ok(parsed);
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
                let final_index = vs.len() - 1;
                if vs[final_index].contains("ok") {
                    result.push((String::from(vs[final_index - 2]), true));
                } else {
                    result.push((String::from(vs[final_index - 2]), false));
                }
            } else {
                let vs : Vec<&str> = s.split(" ").collect();
                let final_index = vs.len() - 1;
                if vs[final_index].contains("ok") {
                    result.push((String::from(vs[1]), true));
                } else {
                    result.push((String::from(vs[1]), false));
                }
            }
        }
    }
    result.sort_by(|a, b| b.0.cmp(&a.0));
    return Some(result);
}