use std::{fmt, fs, io};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use crate::app::converter::parser::{lexical_analyzer, ParserError};
use crate::app::framework::Framework;

pub mod token;
pub mod parser;

#[derive(Debug)]
pub enum ConverterError {
    FailedReadInputFile(std::io::Error),
    Parse(ParserError),
    SyntaxError,
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
            ConverterError::SyntaxError => write!(f, "not syntax of mig-file"),
        }
    }
}

// this function is controller for convert
pub fn convert_to_migration_file<'a, 'b>(
    input: PathBuf,
    output: PathBuf,
    framework: Framework,
) -> Result<&'a str, ConverterError> {
    println!("reading from input file...");
    let content = fs::read_to_string(input)
        .map_err(|e| ConverterError::FailedReadInputFile(e))?;
    println!("finish reading file");

    println!("parsing content...");
    let tokens = lexical_analyzer(content)?;
    // println!("{:?}", tokens);
    println!("finish parsing");

    //println!("{:?}", tokens);

    println!("checking parsing data...");
    if !tokens.check_syntax() {
        return Err(ConverterError::SyntaxError);
    }
    println!("finish checking data");

    // TODO convert from tokens to code of target's framework
    println!("converting checked data...");
    println!("finish converting data");

    // TODO write token sequence in output file
    println!("writing data in output file");
    println!("finish writing data");

    return Ok("Success!! converted!");
}
