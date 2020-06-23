use super::mut_gen::MutantInfo;
use super::mut_test::TestResult;
use std::{
    fs,
    fs::File,
    io,
    io::prelude::*,
    path::PathBuf,
};

// MutantInfo : source_name, file_name, target_line, mutation
// TestResult : Survived, Killed, CompileError, Timeout
pub fn make_report(path: String, result: Vec<(MutantInfo, TestResult)>){
    let mut table_by_file: Vec<(String, i32, i32, i32, String)> = Vec::new(); // (filename, survived, killed, compile error, mutation score)
    let mut table_by_type: Vec<(String, i32, i32, i32, String)> = Vec::new(); // (filename, survived, killed, compile error, mutation score)
    let mut files: Vec<String> = Vec::new();
    let mut types: Vec<String> = Vec::new();

    // make tables for html
    for x in result { // table_by_file
        // println!("{:?}", x.1.clone());
        match files.iter().position(|r| *r == x.0.source_name.clone()) {
            Some(index) => {
                match x.1.clone() {
                    TestResult::Survived => {
                        table_by_file[index] = (table_by_file[index].0.clone(), table_by_file[index].1 + 1, table_by_file[index].2, table_by_file[index].3, table_by_file[index].4.clone());
                    },
                    TestResult::Killed => {
                        table_by_file[index] = (table_by_file[index].0.clone(), table_by_file[index].1, table_by_file[index].2 + 1, table_by_file[index].3, table_by_file[index].4.clone());
                    },
                    TestResult::CompileError => {
                        // table_by_file[index] = (table_by_file[index].0.clone(), table_by_file[index].1, table_by_file[index].2, table_by_file[index].3 + 1, table_by_file[index].4.clone());
                    },
                    TestResult::Timeout => {
                        table_by_file[index] = (table_by_file[index].0.clone(), table_by_file[index].1, table_by_file[index].2, table_by_file[index].3 + 1, table_by_file[index].4.clone());
                    },
                }
            }
            None => { // new filename
                files.push(x.0.source_name.clone());
                match x.1.clone() {
                    TestResult::Survived => {
                        table_by_file.push((x.0.source_name.clone(), 1, 0, 0, "".to_string()));
                    },
                    TestResult::Killed => {
                        table_by_file.push((x.0.source_name.clone(), 0, 1, 0, "".to_string()));
                    },
                    TestResult::CompileError => {
                        // table_by_file.push((x.0.source_name.clone(), 0, 0, 1, "".to_string()));
                    },
                    TestResult::Timeout => {
                        table_by_file.push((x.0.source_name.clone(), 0, 0, 1, "".to_string()));
                    },
                    
                }
            }
        }
        match types.iter().position(|r| *r == x.0.mutation.clone()) {
            Some(index) => {
                match x.1.clone() {
                    TestResult::Survived => {
                        table_by_type[index] = (table_by_type[index].0.clone(), table_by_type[index].1 + 1, table_by_type[index].2, table_by_type[index].3, table_by_type[index].4.clone());
                    },
                    TestResult::Killed => {
                        table_by_type[index] = (table_by_type[index].0.clone(), table_by_type[index].1, table_by_type[index].2 + 1, table_by_type[index].3, table_by_type[index].4.clone());
                    },
                    TestResult::CompileError => {
                        // table_by_type[index] = (table_by_type[index].0.clone(), table_by_type[index].1, table_by_type[index].2, table_by_type[index].3 + 1, table_by_type[index].4.clone());
                    },
                    TestResult::Timeout => {
                        table_by_type[index] = (table_by_type[index].0.clone(), table_by_type[index].1, table_by_type[index].2, table_by_type[index].3 + 1, table_by_type[index].4.clone());
                    },
                }    
            }
            None => { // new type of mutant
                types.push(x.0.mutation.clone());
                match x.1.clone() {
                    TestResult::Survived => {
                        table_by_type.push((x.0.mutation.clone(), 1, 0, 0, "".to_string()));
                    },
                    TestResult::Killed => {
                        table_by_type.push((x.0.mutation.clone(), 0, 1, 0, "".to_string()));
                    },
                    TestResult::CompileError => {
                        // table_by_type.push((x.0.mutation.clone(), 0, 0, 1, "".to_string()));
                    },
                    TestResult::Timeout => {
                        table_by_type.push((x.0.mutation.clone(), 0, 0, 1, "".to_string()));
                    },
                }    
            }
        }
    }

    let mut survived = 0;
    let mut killed = 0;
    let mut compile_error = 0;
    let mut mutation_score = String::new();

    let mut idx = 0;
    for row in table_by_file.clone() {
        survived += row.1;
        killed += row.2;
        compile_error += row.3;
        table_by_file[idx] = (row.0, row.1, row.2, row.3, (((row.2 as f32) * 100_f32 / (row.1 as f32 + row.2 as f32)).round() as i32).to_string() + &("%".to_string()));
        idx += 1
    }
    idx = 0;
    for row in table_by_type.clone() {
        table_by_type[idx] = (row.0, row.1, row.2, row.3, (((row.2 as f32) * 100_f32 / (row.1 as f32 + row.2 as f32)).round() as i32).to_string() + &("%".to_string()));
        idx += 1
    }

    mutation_score = (((killed as f32) * 100_f32 / (survived as f32 + killed as f32)).round() as i32).to_string() + &("%".to_string());
    table_by_file.push(("total".to_string(), survived, killed, compile_error, mutation_score.clone()));
    table_by_type.push(("total".to_string(), survived, killed, compile_error, mutation_score.clone()));

    // println!("{:?}", table_by_file);
    // println!("{:?}", table_by_type);

    let mut doc = String::from("");
    doc.push_str(
        "<html>\n\
            <head>\n\
                <title>Tiny Mutator Report</title>\n\
                <link rel=stylesheet type=text/css href=style.css></link>\n\
            </head>\n\
            <body>\n\
                <h1>Tiny Mutator Report</h1>\n\
                <h2>2020 Spring CS453 Team 7</h2>\n\
                <div class=\"container\">\n\
                    <div class=\"parent\">File Name</div>\n\
                    <div class=\"parent\">Survived Mutants</div>\n\
                    <div class=\"parent\">Killed Mutants</div>\n\
                    <div class=\"parent\">Timeout Mutants</div>\n\
                    <div class=\"parent\">Mutation Score</div>\n\
                </div>\n"
    );

    for row in table_by_file {
        doc.push_str(
            &("<div class=\"container\">\n\
                <div class=\"item\">".to_string() + &(row.0.to_string()) + &("</div>\n".to_string()) +
                &"<div class=\"item\">".to_string() + &(row.1.to_string()) + &("</div>\n".to_string()) +
                &"<div class=\"item\">".to_string() + &(row.2.to_string()) + &("</div>\n".to_string()) +
                &"<div class=\"item\">".to_string() + &(row.3.to_string()) + &("</div>\n".to_string()) +
                &"<div class=\"item\">".to_string() + &(row.4.to_string()) + &("</div>\n".to_string()) +
            &("</div>\n".to_string()))
        );
    }

    doc.push_str(
        &("<div class=\"container\">\n\
            <div class=\"parent\">Type of Mutants</div>\n\
            <div class=\"parent\">Survived Mutants</div>\n\
            <div class=\"parent\">Killed Mutants</div>\n\
            <div class=\"parent\">Timeout Mutants</div>\n\
            <div class=\"parent\">Mutation Score</div>\n\
        </div>\n")
    );

    for row in table_by_type {
        doc.push_str(
            &("<div class=\"container\">\n\
                <div class=\"item\">".to_string() + &(row.0.to_string()) + &("</div>\n".to_string()) +
                &"<div class=\"item\">".to_string() + &(row.1.to_string()) + &("</div>\n".to_string()) +
                &"<div class=\"item\">".to_string() + &(row.2.to_string()) + &("</div>\n".to_string()) +
               &"<div class=\"item\">".to_string() + &(row.3.to_string()) + &("</div>\n".to_string()) +
                &"<div class=\"item\">".to_string() + &(row.4.to_string()) + &("</div>\n".to_string()) +
            &("</div>\n".to_string()))
        );        
    }

    doc.push_str(
            "</body>\n\
        </html>"
    );

    let css = 
    "html, body {\n\
        margin: 0;\n\
        padding: 0;\n\
        border: 0;\n\
        outline: 0;\n\
        font-weight: inherit;\n\
        font-style: inherit;\n\
        font-size: 100%;\n\
        font-family: inherit;\n\
        vertical-align: baseline;\n\
    }\n\
    \n\
    h1 {\n\
        text-align: center;\n\
        margin-top: 50px;\n\
        margin-bottom: 0;\n\
    }\n\
    \n\
    h2 {\n\
        text-align: center;\n\
        margin-top: 0;\n\
    }\n\
    \n\
    .container {\n\
        text-align: center;\n\
    }\n\
    \n\
    .parent {\n\
        background: #D0E4F5;\n\
        font-weight: bold;\n\
        text-align: center;\n\
        margin: auto;\n\
        margin-top: 50px;\n\
        margin-bottom: 20px;\n\
        padding: 5px;\n\
        width: 15%;\n\
        display: inline-block;\n\
    }\n\
    \n\
    .item {\n\
        margin-bottom: 20px;\n\
        text-align: center;\n\
        display: table-cell;\n\
        vertical-align: middle;\n\
        padding: 5px;\n\
        width: 15%;\n\
        display: inline-block;\n\
    }";

    fs::create_dir(&(path.clone() + &("/Tiny_Mutator_Report")));

    let doc_name = path.clone() + &("/Tiny_Mutator_Report/report.html".to_string());
    let mut file = File::create(doc_name.clone()).unwrap();
    file.write_all(doc.as_bytes());

    let css_name = path.clone() + &("/Tiny_Mutator_Report/style.css".to_string());
    let mut file = File::create(css_name.clone()).unwrap();
    file.write_all(css.as_bytes());
}