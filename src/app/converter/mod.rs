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

    let token_seq_result = lexical_analyzer("");
    if token_seq_result.is_err() {
        return Err(&token_seq_result.err().unwrap());
    }
    let token_seq = token_seq_result.unwrap();
    // return Ok("Success converted");
    return Err("failed convert");
}
