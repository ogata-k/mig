use std::error;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::Chars;

use failure::Fail;

use crate::app::converter::token::{Sequence, Token};
use crate::app::converter::token::Token::NameColon;

pub fn lexical_analyzer<'a>(input: String) -> Result<Sequence, ParserError> {
    return Parser::new(input).parse();
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParserError {
    // for Stream
    NotGetCharacter(u16, u16),
    NotAsciiCharacter(u16, u16),
    UnknownToken(u16, u16),
    EndOfStream,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ParserError::NotGetCharacter(row, col) =>
                write!(f, "cannot get  character in (row, col) = ({}, {})", row, col),
            ParserError::NotAsciiCharacter(row, col) =>
                write!(f, "not ascii character in (row, col) = ({}, {})", row, col),
            ParserError::UnknownToken(row, col) =>
                write!(f, "read unknown token at the end of (row, col) = ({}, {})", row, col),
            ParserError::EndOfStream =>
                write!(f, "End Of input Stream"),
        }
    }
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

    fn next(&mut self) -> Result<char, ParserError> {
        if self.counter.is_new_line {
            // if before char is \n, now reading char is head of newline
            self.counter.cursor.0 += 1;
            self.counter.cursor.1 = 0;
            self.counter.is_new_line = false;
        }
        let ch_opt = self.chars.next();
        if ch_opt.is_none() {
            if self.chars.as_str() == "" {
                return Err(ParserError::EndOfStream);
            }
            return Err(ParserError::NotGetCharacter(self.counter.cursor.0, self.counter.cursor.1));
        }
        let ch = ch_opt.unwrap();
        if !ch.is_ascii() {
            return Err(ParserError::NotAsciiCharacter(self.counter.cursor.0, self.counter.cursor.1));
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

    // better? return type is Chars
    pub fn next_while<F>(&mut self, check: F) -> Vec<char>
        where F: Fn(char) -> bool {
        let mut v = Vec::new();
        // look next char
        while let Some(c) = self.look(1) {
            // c satisfy with check
            if check(c) {
                // consume stream's next char
                v.push(self.next().unwrap());
                continue;
            }
            break;
        }
        return v;
    }

    pub fn skip_spaces_or_newlines(&mut self) {
        let cs = self.next_while(|c| c.is_whitespace());
    }

    /// 1 origin
    pub fn look(&mut self, n: usize) -> Option<char> {
        let mut i = 1usize;
        if n == 0 { return None }
        let mut n_dummy = n;
        while i != n_dummy {
            let nth_opt = self.chars.clone().nth(i - 1);
            if nth_opt.is_none() {
                return None;
            }
            let nth_char = nth_opt.unwrap();
            if '\r' == nth_char {
                n_dummy += 1;
                i += 1;
                continue;
            }
            if nth_char.is_ascii() {
                i += 1;
                continue;
            }
            return None;
        }
        let c = self.chars.clone().nth(n_dummy - 1);
        return c;
    }
}

// check the character which we can use for option name for mig-file
fn is_mig_opt_name_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-'
}

// check the string which we can use for option name for mig-file
fn is_mig_opt_name(cs: &Vec<char>) -> bool {
    if cs.is_empty() { return false; }
    if cs[0].is_ascii_digit() || cs[0] == '-' { return false; }
    return cs.iter().fold(true, |acc, c| acc && is_mig_opt_name_char(*c));
}

#[derive(Debug, Clone)]
pub struct Parser {
    src: String,
}

impl Parser {
    pub fn new(input: String) -> Self {
        return Parser { src: input };
    }

    pub fn parse(&mut self) -> Result<Sequence, ParserError> {
        // cf: https://qiita.com/agatan/items/8a097ead46df1c1659ff
        let mut parsed: Vec<Token> = Vec::new();

        // init
        let parser_clone = self.clone();
        let mut stream = Stream::new(&parser_clone);

        println!("\n-------------");
        // TODO parse
        loop {
            stream.skip_spaces_or_newlines();
            let c_opt = stream.next();
            match c_opt {
                Ok(_) => {},
                Err(e) => {
                    if e != ParserError::EndOfStream {
                        return Err(e);
                    }
                    break;
                },
            }
            let c = c_opt.unwrap();
            print!("{}: ", c);
            // TODO calc sequence
            match c {
                '{' => {
                    parsed.push(Token::LMidParen);
                    continue;
                },
                '}' => {
                    parsed.push(Token::RMidParen);
                    continue;
                },
                ':' => {
                    let cs = stream.next_while(|c| is_mig_opt_name_char(c));
                    if is_mig_opt_name(&cs) {
                        let cs_dummy = cs.clone();
                        let s = cs_dummy.iter().collect();
                        parsed.push(NameColon(s));
                        continue;
                    }
                    return Err(ParserError::UnknownToken(stream.counter.cursor.0, stream.counter.cursor.1));
                },
                '"' => {
                    let cs = stream.next_while(|c| c != '"');
                    let checker = stream.next();
                    match checker {
                        Ok('"') => {
                            parsed.push(Token::String(cs.iter().collect()));
                            continue;
                        },
                        Ok(_) => {
                            return Err(ParserError::UnknownToken(stream.counter.cursor.0, stream.counter.cursor.1));
                        },
                        Err(e) => {
                            return Err(e);
                        },
                    }
                },
                _ => { continue; /* change to ParseError::UnknownToken*/ },
            }
        }
        println!("\n-------------");
        println!("{:?}", stream);

        let seq = Sequence::from(parsed);
        println!("{:?}", seq);
        return Ok(seq);
    }
}