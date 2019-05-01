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

pub fn convert_to_migration_file<'a, 'b>(
    input: PathBuf,
    output: PathBuf,
    framework: Framework,
) -> Result<&'a str, ConverterError> {
    // TODO convert from input file "input" to output file "output" with target framework "framework"
    // this function is controller for convert
    let content_result = fs::read_to_string(input);
    match content_result {
        Ok(_) => {},
        Err(e) => {
            return Err(ConverterError::FailedReadInputFile(e));
        }
    };

    let content = content_result.unwrap();

    let token_seq = lexical_analyzer(content)?;

    // TODO write token sequence in output file

    return Ok("Success!! converted!");
}
