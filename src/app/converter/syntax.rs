use std::fmt::{Display, Error, Formatter};

use crate::app::converter::token::Token;

#[derive(Debug)]
pub enum SyntaxError {
    NoOption(Token),
    UnknownOptionName(Token),
    UnknownOptionParam(Token),
    TooShort,
    CorrectSyntax,
    UnknownError,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            SyntaxError::NoOption(t) => write!(f, "{:?} has no option", t),
            SyntaxError::TooShort => write!(f, "input file has not enough num of tokens"),
            SyntaxError::UnknownError => write!(f, "occurred unknown syntax error"),
            SyntaxError::UnknownOptionName(t) => write!(f, "{:?} is not option name", t),
            SyntaxError::CorrectSyntax => write!(f, "not correct syntax "),
            SyntaxError::UnknownOptionParam(t) => write!(f, "{:?} is not option parameter", t),
        }
    }
}
