#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LMidParen,
    RMidParen,
    Name(String),
    // table, column
    NameColon(String),
    // command, opt
    Ymd(u16, u8, u8),
    Time(u8, u8, u8),
    DateTime(u16, u8, u8, u8, u8, u8),
    Integer(i16),
    Double(f32),
    String(String),  // "..."
}

#[derive(Debug)]
pub struct Sequence {
    seq: Vec<Token>
}

impl From<Vec<Token>> for Sequence {
    fn from(vec: Vec<Token>) -> Sequence {
        return Sequence { seq: vec };
    }
}

impl Sequence {
    pub fn get_token(&self, i: usize) -> Token {
        return self.seq[i].clone();
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        return self.seq.clone();
    }

    pub fn check_syntax(&self) -> bool {
        let mut tokens = self.get_tokens();
        if tokens.len() < 4 { return false; }

        return check_syntax_rec(&mut tokens);
    }
}


fn check_syntax_rec(tokens: &mut Vec<Token>) -> bool {
    // TODO syntax pattern match by LL(1)
    let mut seq = tokens.clone();
    let b: bool = match &seq[0] {
        Token::NameColon(s) => {
            if s == "create" {
                if seq[2] == Token::LMidParen && seq[seq.len()] == Token::RMidParen {
                    let l = seq.len();
                    return check_syntax_rec(&mut seq[3..l - 1].to_vec());
                }
            }
            // TODO else pattern
            return false;
        },
        _ => { return false; },
    };
    return b;
}

