use std::{fmt, fs, io};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use crate::app::converter::parser::{lexical_analyzer, ParserError};
use crate::app::framework::Framework;

mod token;
mod parser;

#[derive(Debug)]
pub enum ConverterError {
    FailedReadInputFile(std::io::Error),
    Parse(ParserError),
}

impl From<ParserError> for ConverterError {
    fn from(p_e: ParserError) -> Self {
        return ConverterError::Parse(p_e);
    }
}

impl Display for ConverterError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ConverterError::FailedReadInputFile(io_e) => write!(f, "failed read input file,: {}", io_e.to_string()),
            ConverterError::Parse(p_e) => write!(f, "{}", p_e.to_string()),
        }
    }
}

// this function is controller for convert
pub fn convert_to_migration_file<'a, 'b>(
    input: PathBuf,
    output: PathBuf,
    framework: Framework,
) -> Result<&'a str, ConverterError> {
    println!("start reading from input file...");
    let content = fs::read_to_string(input)
        .map_err(|e| ConverterError::FailedReadInputFile(e))?;
    println!("finish reading file");

    println!("start parsing content...");
    let tokens = lexical_analyzer(content)?;
    // println!("{:?}", tokens);
    println!("finish parsing");

    // TODO write token sequence in output file
    println!("{:?}", tokens);

    return Ok("Success!! converted!");
}
