use crate::app::converter::token::{Sequence, Token};

pub fn lexical_analyzer<'a>(input: String) -> Result<Sequence, &'a str> {
    return Parser::new(input).parse();
}

pub struct Parser {
    src: String,
    // (row, col)
    cursor: (u16, u16),
}

impl Parser {
    pub fn new<'b>(input: String) -> Self {
        return Parser { src: input, cursor: (1, 1) };
    }

    fn get_cursor(&self) -> (u16, u16) {
        // (row, col)
        return self.cursor;
    }

    pub fn parse(&mut self) -> Result<Sequence, &'static str> {
        let mut parsed: Vec<Token> = Vec::new();

        // TODO parse

        let seq = Sequence::from(parsed);
        return Ok(seq);
    }
}