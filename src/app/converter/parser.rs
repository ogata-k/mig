use std::error;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::Chars;

use failure::Fail;

use crate::app::converter::token::{Sequence, Token};

pub fn lexical_analyzer<'a>(input: String) -> Result<Sequence, ParserError> {
    return Parser::new(input).parse();
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParserError {
    // for Stream
    NotGetCharacter(u16, u16),
    NotAsciiCharacter(u16, u16),
    UnknownToken(u16, u16),
    NotANumber(u16, u16),
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
            ParserError::NotANumber(row, col) =>
                write!(f, "cannot read a number at the end of (roe, col) = ({}, {})", row, col),
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
            return Err(ParserError::NotGetCharacter(self.get_row(), self.get_col()));
        }
        let ch = ch_opt.unwrap();
        if !ch.is_ascii() {
            return Err(ParserError::NotAsciiCharacter(self.get_row(), self.get_col()));
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
        print!("{}, ", ch);
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

    pub fn get_row(&self) -> u16 {
        return self.counter.cursor.0;
    }

    pub fn get_col(&self) -> u16 {
        return self.counter.cursor.1;
    }

    pub fn get_position(&self) -> usize {
        return self.counter.position;
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
                        parsed.push(Token::NameColon(s));
                        continue;
                    }
                    return Err(ParserError::UnknownToken(stream.get_row(), stream.get_col()));
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
                            return Err(ParserError::UnknownToken(stream.get_row(), stream.get_col()));
                        },
                        Err(e) => {
                            return Err(e);
                        },
                    }
                },
                i @ '0'...'9' => {
                    // TODO parse continue number  check!:001, 0.99, 0, 00
                    match stream.look(1) {
                        // 0 ... 9
                        Some(c) if c.is_whitespace() => {
                            let d = i.to_digit(10).ok_or(
                                ParserError::NotANumber(stream.get_row(), stream.get_col())
                            )?;
                            println!("hoge");
                            stream.skip_spaces_or_newlines();
                            parsed.push(Token::Integer(d as i16));
                            continue;
                        },
                        // [0-9]<ascii>
                        // TODO フローを変更！！ 最初に数字を全部読み取って、そのあと振り分ける. もちろんトークンになるかの数値条件チェックを忘れずに
                        Some(c) if c.is_ascii() => {
                            // Ymd is form: 0000-00-00
                            // TODO Ymd DateTime
                            // [0-9]<ascii>:
                            if let Some(':') = stream.look(2) {
                                // TIme is form:  00:00:00
                                let cs = stream.next_while(|c| c.is_ascii_digit() || c == ':');
                                // 0   0:12:12
                                //     ~~~~~~~
                                if cs.len() == 7 {
                                    let h = to_unsigned_integer([i, cs[0]].to_vec())
                                        .ok_or(ParserError::NotANumber(stream.get_row(), stream.get_col()))?;
                                    let m = to_unsigned_integer(cs[2..4].to_vec())
                                        .ok_or(ParserError::NotANumber(stream.get_row(), stream.get_col()))?;
                                    let s = to_unsigned_integer(cs[5..7].to_vec())
                                        .ok_or(ParserError::NotANumber(stream.get_row(), stream.get_col()))?;
                                    parsed.push(Token::Time(h as u8, m as u8, s as u8));
                                    continue;
                                }
                            }
                            let cs: Vec<char> = stream.next_while(|c| c.is_ascii_digit());
                            let mut dummy: Vec<char> = [i].to_vec();
                            dummy.append(&mut cs.clone());
                            let uint = to_unsigned_integer(dummy)
                                .ok_or(ParserError::NotANumber(stream.get_row(), stream.get_col()))?;
                            if stream.look(1) != Some('.') {
                                parsed.push(Token::Integer(uint as i16));
                                continue;
                            }
                            let _ = stream.next();
                            let opt_digit_vec = stream.next_while(|c| c.is_ascii_digit());
                            let l = opt_digit_vec.len();
                            let opt_digit = to_unsigned_integer(opt_digit_vec)
                                .ok_or(ParserError::NotANumber(stream.get_row(), stream.get_col()))?;
                            parsed.push(Token::Double(uint as f32 + (opt_digit as f32 / (10_usize.pow(l as u32)) as f32)));
                            continue;
                        },
                        None => {
                            parsed.push(Token::Integer(0));
                            continue;
                        },
                        Some('.') => {
                            let uint = i.to_digit(10)
                                .ok_or(ParserError::NotANumber(stream.get_row(), stream.get_col()))?;
                            let _ = stream.next();
                            let opt_digit_vec = stream.next_while(|c| c.is_ascii_digit());
                            let l = opt_digit_vec.len();
                            let opt_digit = to_unsigned_integer(opt_digit_vec)
                                .ok_or(ParserError::NotANumber(stream.get_row(), stream.get_col()))?;
                            parsed.push(Token::Double(uint as f32 + (opt_digit as f32 / (10_usize.pow(l as u32)) as f32)));
                            continue;
                        },
                        // TODO signed integer but impl only negative
                        Some(_) => {
                            return Err(ParserError::NotANumber(stream.get_row(), stream.get_col()));
                        }
                    }
                    continue;
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

// type Data = Vec<char> とか？
fn to_unsigned_integer(v: Vec<char>) -> Option<usize> {
    if v.is_empty() { return None; }
    let s = v.iter()
        .map(|c| c.to_digit(10))
        .fold(Some(0), |acc, d|
            if acc.and(d).is_some() { Some(10 * acc.unwrap() + d.unwrap()) } else { None },
        );
    return s.and_then(|s| Some(s as usize));
}
