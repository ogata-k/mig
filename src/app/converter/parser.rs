use crate::app::converter::token::{Sequence, Token};

pub fn lexical_analyzer(input: String) -> Result<Sequence, &'static str> {
    let mut seq: Vec<Token> = vec!();

    // TODO use parsers
    // https://qiita.com/agatan/items/8a097ead46df1c1659ff
    return Ok(Sequence::from(seq));
}
