use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::app::converter::parser::lexical_analyzer;
use crate::app::framework::Framework;

mod token;
mod parser;

pub fn convert_to_migration_file<'a, 'b>(
    input: PathBuf,
    output: PathBuf,
    framework: Framework,
) -> Result<&'a str, &'b str> {
    // TODO convert from input file "input" to output file "output" with target framework "framework"
    // this function is controller for convert
    let content_result = fs::read_to_string(input);
    match content_result {
        Ok(_) => {},
        Err(e) => {
            let s = e.to_string();
            return Err("failed reading in input file");
        }
    };

    let content = content_result.unwrap();

    let token_seq_result = lexical_analyzer(content);
    match token_seq_result {
        Ok(_) => {},
        Err(e) => {
            return Err(e);
        }
    };
    let token_seq = token_seq_result.unwrap();

    // TODO write token sequence in output file

    return Ok("Success!! converted!");
}
