extern crate combine;

use combine::Parser;
use combine::parser::char::digit;

use crate::app::converter::token::{Sequence, Token};

use self::combine::token;

pub fn lexical_analyzer(input: &str) -> Sequence {
    let mut v: Vec<Token> = vec!();

    // TODO use parsers

    return Sequence::from(v);
}
