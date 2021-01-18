#[macro_use]
extern crate anyhow;
extern crate serde_json;

use std::fs;
use std::path;

use anyhow::Result;
use clap::{App, Arg};

use clarity_repl::{clarity, repl};

#[derive(Debug)]
struct Position {
    line: u64,
    character: u64,
}

#[derive(Debug)]
struct Range {
    start: Position,
    end: Position,
}

fn main() -> Result<()> {
    let file_arg = "file";

    let matches = App::new("clarity-lint")
        .version("0.1.1")
        .arg(
            Arg::new(file_arg)
                .short('f')
                .long(file_arg)
                .takes_value(true)
                .value_name("PATH")
                .about("Species the clarity file to be linted")
                .required(true),
        )
        .get_matches();
    let mut clarity_interpreter = repl::ClarityInterpreter::new();

    let file_path_option = matches.value_of(file_arg);
    if file_path_option.is_none() {
        return Err(anyhow!("File path was not passed in!"));
    }
    let file_path = path::Path::new(file_path_option.expect("Hi"));
    let content = fs::read_to_string(file_path)?;

    let contract_identifier = clarity::types::QualifiedContractIdentifier::transient();

    // Parse Contract
    let mut contract_ast =
        match clarity_interpreter.build_ast(contract_identifier.clone(), content.clone()) {
            Ok(res) => res,
            // Parse diagnotic and return error
            Err((_, Some(parsing_diag))) => {
                let mut parsed_diag = clarity::diagnostic::Diagnostic {
                    level: parsing_diag.level,
                    message: parsing_diag.message.clone(),
                    spans: parsing_diag.spans.clone(),
                    suggestion: parsing_diag.suggestion.clone(),
                };
                parsed_diag.add_span(0, 0, 0, 0);
                let diag = serde_json::to_string_pretty(&parsed_diag);
                // Print Diagnotic for now
                println!("{}", diag.unwrap());
                return Ok(());
            }
            _ => {
                let diag = serde_json::to_string_pretty(&clarity::diagnostic::Diagnostic {
                    level: clarity::diagnostic::Level::Error,
                    message: String::from("No Diagnostic"),
                    spans: vec![],
                    suggestion: None,
                });
                println!("{}", diag.unwrap());
                return Ok(());
            }
        };

    let diags =
        match clarity_interpreter.run_analysis(contract_identifier.clone(), &mut contract_ast) {
            Ok(_) => Ok(String::from("")),
            Err((_, Some(analysis_diag))) => serde_json::to_string_pretty(&analysis_diag),
            _ => {
                let diag = serde_json::to_string_pretty(&clarity::diagnostic::Diagnostic {
                    level: clarity::diagnostic::Level::Error,
                    message: String::from("No Diagnostic"),
                    spans: vec![],
                    suggestion: None,
                });
                println!("{}", diag.unwrap());
                return Ok(());
            }
        };

    println!("{}", diags.unwrap());

    return Ok(());
}
