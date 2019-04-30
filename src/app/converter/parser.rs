use std::str::Chars;

use crate::app::converter::token::{Sequence, Token};

pub fn lexical_analyzer<'a>(input: String) -> Result<Sequence, &'a str> {
    return Parser::new(input).parse();
}

#[derive(Debug)]
pub struct Counter {
    // (row, col)
    cursor: (u16, u16),
    position: usize,
}

impl Counter {
    pub fn new() -> Self {
        return Counter { cursor: (1, 1), position: 1 };
    }
}

#[derive(Debug)]
struct Stream<'a> {
    chars: Chars<'a>,
    counter: Counter,
}

impl<'a> Stream<'a> {
    fn new(parser: &'a Parser) -> Self {
        return Stream { chars: (*parser.src).chars(), counter: Counter::new() };
    }
}

// TODO Parse Error Struct

#[derive(Debug, Clone)]
pub struct Parser {
    src: String,
}

impl Parser {
    pub fn new(input: String) -> Self {
        return Parser { src: input };
    }

    pub fn parse(&mut self) -> Result<Sequence, &'static str> {
        // cf: https://qiita.com/agatan/items/8a097ead46df1c1659ff
        let mut parsed: Vec<Token> = Vec::new();

        // init
        let parser_clone = self.clone();
        let mut stream = Stream::new(&parser_clone);

        println!("\n-------------");
        // TODO parse
        loop {
            let c_opt = stream.chars.next(); // TODO update cursor and position method when next
            if c_opt.is_none() { break; }
            stream.counter.position += 1; // ***
            let c = c_opt.unwrap();
            print!("{}", c);
            // TODO calc sequence
            continue;
        }
        println!("\n-------------");
        println!("{:?}", stream);

        let seq = Sequence::from(parsed);
        return Ok(seq);
    }
}