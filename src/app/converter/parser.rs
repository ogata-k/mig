extern crate combine;

use combine::Parser;
use combine::parser::char::digit;

use crate::app::converter::token::{Sequence, Token};

use self::combine::token;

pub fn lexical_analyzer(input: String) -> Result<Sequence, &'static str> {
    let mut seq: Vec<Token> = vec!();

    // TODO use parsers

    return Ok(Sequence::from(seq));
}
