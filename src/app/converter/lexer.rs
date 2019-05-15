use std::error;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::Chars;

use failure::Fail;

use crate::app::converter::token::{Sequence, Token};

pub fn lexical_analyzer<'a>(input: String) -> Result<Sequence, LexerError> {
    return Lexer::new(input).parse();
}

#[derive(Debug, Eq, PartialEq)]
pub enum LexerError {
    // for Stream
    NotGetCharacter(u16, u16),
    NotAsciiCharacter(u16, u16),
    UnknownToken(u16, u16),
    NotANumber(u16, u16),
    NumberRangeError(u16, u16),
    EndOfStream,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            LexerError::NotGetCharacter(row, col) =>
                write!(f, "cannot get  character in (row, col) = ({}, {})", row, col),
            LexerError::NotAsciiCharacter(row, col) =>
                write!(f, "not ascii character in (row, col) = ({}, {})", row, col),
            LexerError::UnknownToken(row, col) =>
                write!(f, "read unknown token at the end of (row, col) = ({}, {})", row, col),
            LexerError::NotANumber(row, col) =>
                write!(f, "cannot read a number at the end of (row, col) = ({}, {})", row, col),
            LexerError::NumberRangeError(row, col) =>
                write!(f, "success parse a number, but the number is out of range.\nfinish reading at (row, col) = ({}, {})", row, col),
            LexerError::EndOfStream =>
                write!(f, "End Of input Stream"),
        }
    }
}

#[derive(Debug)]
pub struct Counter {
    // (row, col)
    cursor: (u16, u16),
    position: usize,
    is_new_line: bool,
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
    fn new(parser: &'a Lexer) -> Self {
        return Stream { chars: (*parser.src).chars(), counter: Counter::new() };
    }

    fn next(&mut self) -> Result<char, LexerError> {
        if self.counter.is_new_line {
            // if before char is \n, now reading char is head of newline
            self.counter.cursor.0 += 1;
            self.counter.cursor.1 = 0;
            self.counter.is_new_line = false;
        }
        let ch_opt = self.chars.next();
        if ch_opt.is_none() {
            if self.chars.as_str() == "" {
                return Err(LexerError::EndOfStream);
            }
            return Err(LexerError::NotGetCharacter(self.get_row(), self.get_col()));
        }
        let ch = ch_opt.unwrap();
        if !ch.is_ascii() {
            return Err(LexerError::NotAsciiCharacter(self.get_row(), self.get_col()));
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
        // print!("{}, ", ch);
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
        if n == 0 { return None; }
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
pub struct Lexer {
    src: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        return Lexer { src: input };
    }

    pub fn parse(&mut self) -> Result<Sequence, LexerError> {
        let mut parsed: Vec<Token> = Vec::new();

        // init
        let parser_clone = self.clone();
        let mut stream = Stream::new(&parser_clone);

        loop {
            stream.skip_spaces_or_newlines();
            let c_opt = stream.next();
            match c_opt {
                Ok(_) => {}
                Err(e) => {
                    if e != LexerError::EndOfStream {
                        return Err(e);
                    }
                    break;
                }
            }
            let c = c_opt.unwrap();
            match c {
                '{' => {
                    parsed.push(Token::LMidParen);
                    continue;
                }
                '}' => {
                    parsed.push(Token::RMidParen);
                    continue;
                }
                ':' => {
                    let cs = stream.next_while(|c| is_mig_opt_name_char(c));
                    if is_mig_opt_name(&cs) {
                        let cs_dummy = cs.clone();
                        let s = cs_dummy.iter().collect();
                        parsed.push(Token::NameColon(s));
                        continue;
                    }
                    return Err(LexerError::UnknownToken(stream.get_row(), stream.get_col()));
                }
                '"' => {
                    let cs = stream.next_while(|c| c != '"');
                    let checker = stream.next();
                    match checker {
                        Ok('"') => {
                            parsed.push(Token::String(cs.iter().collect()));
                            continue;
                        }
                        Ok(_) => {
                            return Err(LexerError::UnknownToken(stream.get_row(), stream.get_col()));
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                i @ '0'...'9' => {
                    let mut v = vec!(i);
                    let mut v_tail: Vec<char> = stream.next_while(|c| c.is_ascii_digit());
                    v.append(&mut v_tail);
                    // digits is unsigned integer in head of stream
                    let digits = v;
                    let look = stream.look(1);
                    match digits.len() {
                        // Time
                        2 if look == Some(':') => {
                            // Time is form:  00:00:00

                            // stream head is ':'
                            let _ = stream.next();
                            let cs = stream.next_while(|c| c.is_ascii_digit() || c == ':');
                            if !(cs.len() == 5 && cs[2] == ':') {
                                return Err(LexerError::UnknownToken(stream.get_row(), stream.get_col()));
                            }
                            let h = to_unsigned_integer(digits)
                                .ok_or(LexerError::UnknownToken(stream.get_row(), stream.get_col()))?;
                            if h >= 24 {
                                return Err(LexerError::NumberRangeError(stream.get_row(), stream.get_col()));
                            }
                            let m = to_unsigned_integer(cs[0..2].to_vec())
                                .ok_or(LexerError::UnknownToken(stream.get_row(), stream.get_col()))?;
                            if m >= 60 {
                                return Err(LexerError::NumberRangeError(stream.get_row(), stream.get_col()));
                            }
                            let s = to_unsigned_integer(cs[3..5].to_vec())
                                .ok_or(LexerError::UnknownToken(stream.get_row(), stream.get_col()))?;
                            if s >= 60 {
                                return Err(LexerError::NumberRangeError(stream.get_row(), stream.get_col()));
                            }
                            parsed.push(Token::Time(h as u8, m as u8, s as u8));
                            continue;
                        }
                        // Ymd or DateTime
                        4 if look == Some('-') => {
                            let _ = stream.next();
                            // md's form is 00-00
                            let md = stream.next_while(|c| c.is_ascii_digit() || c == '-');
                            if md.len() != 5 {
                                return Err(LexerError::UnknownToken(stream.get_row(), stream.get_col()));
                            }
                            if md[2] != '-' {
                                return Err(LexerError::UnknownToken(stream.get_row(), stream.get_col()));
                            }
                            let y = to_unsigned_integer(digits.to_vec())
                                .ok_or(LexerError::UnknownToken(stream.get_row(), stream.get_col()))?;
                            let m = to_unsigned_integer(md[0..2].to_vec())
                                .ok_or(LexerError::UnknownToken(stream.get_row(), stream.get_col()))?;
                            let d = to_unsigned_integer(md[3..5].to_vec())
                                .ok_or(LexerError::UnknownToken(stream.get_row(), stream.get_col()))?;
                            if m >= 13 || d >= 32 {
                                return Err(LexerError::NumberRangeError(stream.get_row(), stream.get_col()));
                            }

                            let look = stream.look(1);
                            match look {
                                None => {
                                    parsed.push(Token::Ymd(y as u16, m as u8, d as u8));
                                    continue;
                                }
                                Some(ch) if ch.is_whitespace() || ch == '{' || ch == '}' => {
                                    parsed.push(Token::Ymd(y as u16, m as u8, d as u8));
                                    continue;
                                }
                                Some('_') => {
                                    // stream head is '_'
                                    let _ = stream.next();

                                    // Time is form:  00:00:00
                                    let cs = stream.next_while(|c| c.is_ascii_digit() || c == ':');
                                    if !(cs.len() == 8 && cs[2] == ':' && cs[5] == ':') {
                                        return Err(LexerError::UnknownToken(stream.get_row(), stream.get_col()));
                                    }
                                    let h = to_unsigned_integer(cs[0..2].to_vec())
                                        .ok_or(LexerError::UnknownToken(stream.get_row(), stream.get_col()))?;
                                    if h >= 25 {
                                        return Err(LexerError::NumberRangeError(stream.get_row(), stream.get_col()));
                                    }
                                    let mi = to_unsigned_integer(cs[3..5].to_vec())
                                        .ok_or(LexerError::UnknownToken(stream.get_row(), stream.get_col()))?;
                                    if mi >= 60 {
                                        return Err(LexerError::NumberRangeError(stream.get_row(), stream.get_col()));
                                    }
                                    let s = to_unsigned_integer(cs[6..8].to_vec())
                                        .ok_or(LexerError::UnknownToken(stream.get_row(), stream.get_col()))?;
                                    if s >= 60 {
                                        return Err(LexerError::NumberRangeError(stream.get_row(), stream.get_col()));
                                    }
                                    parsed.push(Token::DateTime(y as u16, m as u8, d as u8, h as u8, mi as u8, s as u8));
                                    continue;
                                }
                                Some(_) => {
                                    return Err(LexerError::UnknownToken(stream.get_row(), stream.get_col()));
                                }
                            }
                        }
                        // Double
                        _ if look == Some('.') => {
                            let _ = stream.next();
                            let opt_digit = stream.next_while(|c| c.is_ascii_digit());
                            if opt_digit.is_empty() {
                                return Err(LexerError::NotANumber(stream.get_row(), stream.get_col()));
                            }
                            if let Some(sym) = stream.look(1) {
                                if !(!sym.is_ascii() || sym.is_whitespace() || sym == '{' || sym == '}') {
                                    return Err(LexerError::NotANumber(stream.get_row(), stream.get_col()));
                                }
                            }
                            let d = to_unsigned_f32(digits, opt_digit)
                                .ok_or(LexerError::NotANumber(stream.get_row(), stream.get_col()))?;
                            parsed.push(Token::Double(d));
                            continue;
                        }
                        // 0-9
                        _ if look == None => {
                            let uint = to_unsigned_integer(digits).ok_or(
                                LexerError::NotANumber(stream.get_row(), stream.get_col())
                            )?;
                            parsed.push(Token::Integer(uint as i16));
                            continue;
                        }
                        // Integer
                        _ => {
                            let uint = to_unsigned_integer(digits).ok_or(
                                LexerError::NotANumber(stream.get_row(), stream.get_col())
                            )?;
                            if let Some(sym) = look {
                                if !sym.is_ascii() || sym.is_whitespace() || sym == '{' || sym == '}' {
                                    parsed.push(Token::Integer(uint as i16));
                                    continue;
                                }
                                return Err(LexerError::NotANumber(stream.get_row(), stream.get_col()));
                            }
                            parsed.push(Token::Integer(uint as i16));
                            continue;
                        }
                    }
                }

                '-' => {
                    let digits: Vec<char> = stream.next_while(|c| c.is_ascii_digit());
                    if digits.is_empty() {
                        return Err(LexerError::UnknownToken(stream.get_row(), stream.get_col()));
                    }
                    let look = stream.look(1);
                    match look {
                        // negative double
                        Some('.') => {
                            let _ = stream.next();
                            let digits_opt = stream.next_while(|c| c.is_ascii_digit());
                            if digits_opt.is_empty() {
                                return Err(LexerError::UnknownToken(stream.get_row(), stream.get_col()));
                            }
                            let d = to_unsigned_f32(digits, digits_opt)
                                .ok_or(LexerError::NumberRangeError(stream.get_row(), stream.get_col()))?;
                            if let Some(sym_snd) = stream.look(1) {
                                if !sym_snd.is_ascii() || sym_snd.is_whitespace() || sym_snd == '{' || sym_snd == '}' {
                                    parsed.push(Token::Double(-d));
                                    continue;
                                }
                            }
                            parsed.push(Token::Double(-d));
                            continue;
                        }
                        // negative integer
                        None => {
                            let uint = to_unsigned_integer(digits).ok_or(
                                LexerError::NotANumber(stream.get_row(), stream.get_col())
                            )?;
                            parsed.push(Token::Integer(-(uint as i16)));
                            continue;
                        }
                        // negative integer
                        Some(sym) => {
                            if !sym.is_ascii() || sym.is_whitespace() || sym == '{' || sym == '}' {
                                let uint = to_unsigned_integer(digits).ok_or(
                                    LexerError::NotANumber(stream.get_row(), stream.get_col())
                                )?;
                                parsed.push(Token::Integer(-(uint as i16)));
                                continue;
                            }
                            return Err(LexerError::NotANumber(stream.get_row(), stream.get_col()));
                        }
                    }
                }
                // string for user
                ch if ch.is_ascii_alphabetic() => {
                    let mut v = vec!(ch);
                    v.append(&mut stream.next_while(|ch| ch.is_ascii_alphabetic()));
                    parsed.push(Token::Name(v.iter().collect()))
                }
                _ => { continue; /* change to ParseError::UnknownToken*/ }
            }
            continue;
        }


        let seq = Sequence::from(parsed);
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

fn to_unsigned_f32(head: Vec<char>, tail: Vec<char>) -> Option<f32> {
    let d_h = to_unsigned_integer(head);
    let l = tail.len();
    let d_opt = to_unsigned_integer(tail);
    if d_h.and(d_opt) != None {
        let d = d_h.unwrap() as f32 + (d_opt.unwrap() as f32 / 10_i32.pow(l as u32) as f32);
        return Some(d);
    }
    return None;
}
