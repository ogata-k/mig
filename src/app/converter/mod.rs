use std::{fmt, fs};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::app::converter::lexer::{LexerError, lexical_analyzer};
use crate::app::converter::syntax::SyntaxError;
use crate::app::framework::Framework;

pub mod token;
pub mod lexer;
pub mod syntax;
mod ast;
mod mig;
mod generator;
mod laravel;

#[derive(Debug)]
pub enum ConverterError {
    FailedReadInputFile(std::io::Error),
    Parse(LexerError),
    Syntax(SyntaxError),
}

impl From<std::io::Error> for ConverterError {
    fn from(i_e: std::io::Error) -> Self {
        return ConverterError::FailedReadInputFile(i_e);
    }
}

impl From<LexerError> for ConverterError {
    fn from(p_e: LexerError) -> Self {
        return ConverterError::Parse(p_e);
    }
}

impl From<SyntaxError> for ConverterError {
    fn from(s_e: SyntaxError) -> Self {
        return ConverterError::Syntax(s_e);
    }
}

impl Display for ConverterError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ConverterError::FailedReadInputFile(io_e) => write!(f, "failed read input file,: {}", io_e.to_string()),
            ConverterError::Parse(p_e) => write!(f, "parse error: {}", p_e.to_string()),
            ConverterError::Syntax(s_e) => write!(f, "syntax error: {}", s_e.to_string()),
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
    let content = fs::read_to_string(input)?;
    println!("finish reading file");

    println!("parsing content...");
    let tokens = lexical_analyzer(content)?;
    // println!("{:?}", tokens);
    println!("finish parsing");

    //println!("{:?}", tokens);

    println!("analyze parsing data...");

    let mut ast = tokens.parse()?;
    println!("{:?}", ast);
    if !ast.check_syntax() {
        return Err(ConverterError::Syntax(SyntaxError::CorrectSyntax));
    }
    println!("finish parsing data");

    println!("writing data in output file");
    ast.optimize();
    let ast = ast;  // fix ast
    { // limit lifetime
        let out = output.clone();
        let name_space = out.parent().unwrap().to_str().unwrap();
        let mut output_file = File::create(output)?;
        // TODO Replace: let content = ast.gen_content_for(frame_work, config_file_of(frame_work), name_space);
        //                                                           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~ into gen_content_for function?
        let content: String = format!("{}\n\n{:?}", ast, ast);
        output_file.write_all(&content.into_bytes())?;
        output_file.flush()?;
    }
    println!("finish writing data");

    return Ok("Success!! converted!");
}
