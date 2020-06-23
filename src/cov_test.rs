//use std::io;
#[path = "./utils.rs"]
mod utils;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryFrom;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::MAIN_SEPARATOR;
use std::process::Command;
use std::str;

/**
 * Run cargo tarpaulin & Return the path of report
 */
pub fn run_test(path: String) -> Option<String> {
    // Get Absolute path
    let absolute_path = utils::get_abs_path(&path);
    // Get current working directory
    let current_working_dir = utils::get_cwd();

    println!("{}", path);
    println!("{}", current_working_dir);

    // Make a subprocess & Run 'cargo tarpaulin'
    println!("Install Tarpaulin");
    let _ = Command::new("cargo")
        .args(&["install", "cargo-tarpaulin"])
        .status(); // Install tarpaulin
    println!("Run Cargo Tarpaulin");
    let mut shell = Command::new("cargo");
    shell.args(&[
        "tarpaulin",
        "--out",
        "Json",
        "--output-dir",
        &current_working_dir,
    ]);

    shell.current_dir(absolute_path);
    let output = shell.output(); // run cargo tarpaulin
    // println!("{:#?}", output);
    let report_path = current_working_dir + "/tarpaulin-report.json";

    return Some(report_path);
}

#[derive(Debug)]
pub struct TraceInfo {
    pub path: String,
    pub traces: Vec<usize>,
}

impl TryFrom<&TarpaulinFileLog> for TraceInfo {
    type Error = &'static str;
    fn try_from(v: &TarpaulinFileLog) -> Result<Self, Self::Error> {
        Ok(Self {
            path: v.path.join(&MAIN_SEPARATOR.to_string()),
            traces: v
                .traces
                .iter()
                .map(|t_trace| t_trace.line)
                .collect::<Vec<_>>(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TarpaulinTrace {
    line: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TarpaulinFileLog {
    path: Vec<String>,
    // content: String,
    traces: Vec<TarpaulinTrace>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TarpaulinReport {
    files: Vec<TarpaulinFileLog>,
}

pub fn parse(report_path: &str) -> Result<Vec<TraceInfo>, Box<Error>> {
    // open tarpaulin report
    let report_path = utils::get_abs_path(report_path);
    let file = File::open(report_path)?;
    let reader = BufReader::new(file);

    // read tarpaulin report
    let t_report: TarpaulinReport = serde_json::from_reader(reader)?;

    // convert tarpaulin report to `Vec<TraceInfo>`
    let mut traces = Vec::new();
    for t_file_log in t_report.files.iter() {
        traces.push(TraceInfo::try_from(t_file_log)?);
    }

    Ok(traces)
}
