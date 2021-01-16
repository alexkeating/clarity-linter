// Requirements
//
// Input a file
// And the return the linting errors in that file
// It will be very basic for now, basically should match the VS Code
// plugin

// clarlint <file_path>.clar

// Make App
// Get arge matches
// If file specified then run through the file in the clarity repl
#[macro_use]
extern crate anyhow;
extern crate serde_json;

use std::fs;
use std::path;

use anyhow::Result;
use clap::{App, Arg};

use clarity_repl::{clarity, repl};

struct Position {
    line: u64,
    character: u64,
}

struct Range {
    start: Position,
    end: Position,
}

impl Range {
    fn default() -> Range {
        Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 0,
                character: 0,
            },
        }
    }
}

fn main() -> Result<()> {
    let file_arg = "file";

    let matches = App::new("clarlint")
        .version("0.1")
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
                let _ = match parsing_diag.spans.len() {
                    0 => Range::default(),
                    _ => Range {
                        start: Position {
                            line: parsing_diag.spans[0].start_line as u64 - 1,
                            character: parsing_diag.spans[0].start_column as u64,
                        },
                        end: Position {
                            line: parsing_diag.spans[0].end_line as u64 - 1,
                            character: parsing_diag.spans[0].end_column as u64,
                        },
                    },
                };
                // Print Diagnotic for now
                //
                // I have to figure out how to send this to my vim command
                println!("Parsing Diagonistic {:?}", parsing_diag);
                return Ok(());
            }
            _ => {
                println!("Error returned without diagnotic");
                return Ok(());
            }
        };

    let diags =
        match clarity_interpreter.run_analysis(contract_identifier.clone(), &mut contract_ast) {
            Ok(_) => Ok(String::from("")),
            Err((_, Some(analysis_diag))) => serde_json::to_string_pretty(&analysis_diag),
            _ => {
                println!("Error returned without diagnotic");
                return Ok(());
            }
        };

    // Lint Contract

    // Open file and pass into clarity repl

    // println!("Hello, world!");
    // Reformat output to be like flake8
    // and then hook into the vim plugin
    // Also, my return types will probably change
    println!("{}", diags.unwrap());

    return Ok(());
}
