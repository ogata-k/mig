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
    is_new_line: bool
}

impl Counter {
    pub fn new() -> Self {
        return Counter { cursor: (0, 0), position: 0, is_new_line: true };
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

    fn next(&mut self) -> Result<char, &str> {
        if self.counter.is_new_line {
            // if before char is \n, now reading char is head of newline
            self.counter.cursor.0 += 1;
            self.counter.cursor.1 = 0;
            self.counter.is_new_line = false;
        }
        let ch_opt = self.chars.next();
        if ch_opt.is_none() {
            if self.chars.as_str() == "" {
                return Err("EOS");
            }
            return Err("cannot get next char");
        }
        let ch = ch_opt.unwrap();
        if !ch.is_ascii() {
            return Err("next char is not ascii character")
        }

        // update is next line
        if ch == '\r' {  // for windows
            self.counter.position += 1;
            return self.next();
        }
        if ch == '\n' {
            self.counter.is_new_line = true;
        }

        // update counter
        self.counter.position += 1;
        self.counter.cursor.1 += 1;
        return Ok(ch);
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
            let c_opt = stream.next();
            match c_opt {
                Ok(_) => {},
                Err(e) => {
                    if e != "EOS" {
                        return Err("Error!!");// TODO use c_opt.error
                    }
                    break;
                },
            }
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