use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::app::converter::parser::lexical_analyzer;
use crate::app::framework::Framework;

mod token;
mod parser;

pub fn convert_to_migration_file(
    input: &PathBuf,
    output: &PathBuf,
    framework: Framework,
) -> Result<&'static str, &'static str> {
    // TODO convert from input file "input" to output file "output" with target framework "framework"
    // this function is controller for convert
    let content_result = fs::read_to_string(input);
    if content_result.is_err() {
        // I want to return original error msg
        return Err("failed read to end in input file");
    }
    let content = content_result.unwrap();

    let token_seq_result = lexical_analyzer(content);
    if token_seq_result.is_err() {
        return Err(&token_seq_result.err().unwrap());
    }
    let token_seq = token_seq_result.unwrap();

    // TODO write token sequence in output file

    return Ok("Success!! converted!");
}
