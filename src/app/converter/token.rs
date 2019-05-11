use crate::app::converter::ConverterError::Syntax;
use crate::app::converter::mig::Mig;
use crate::app::converter::syntax::SyntaxError;
use crate::app::helper::slice_helper::split_with_head_and_separator;

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

impl Token {
    pub fn is_l_mid_paren(&self) -> bool {
        return match &self {
            Token::LMidParen => true,
            _ => false,
        };
    }

    pub fn is_r_mid_paren(&self) -> bool {
        return match &self {
            Token::RMidParen => true,
            _ => false,
        };
    }

    pub fn is_name(&self) -> bool {
        return match &self {
            Token::Name(_) => true,
            _ => false,
        };
    }

    pub fn is_name_with(&self, name: String) -> bool {
        return match &self {
            Token::Name(name) => true,
            _ => false,
        };
    }

    pub fn is_name_colon(&self) -> bool {
        return match &self {
            Token::NameColon(_) => true,
            _ => false,
        };
    }

    pub fn is_name_colon_with(&self, name_colon: String) -> bool {
        return match &self {
            &Token::NameColon(name_colon) => true,
            _ => false,
        };
    }

    pub fn is_ymd(&self) -> bool {
        return match &self {
            Token::Ymd(_, _, _) => true,
            _ => false,
        };
    }

    pub fn is_ymd_with(&self, year: u16, month: u8, day: u8) -> bool {
        return match &self {
            &Token::Ymd(year, month, day) => true,
            _ => false,
        };
    }

    pub fn is_time(&self) -> bool {
        return match &self {
            Token::Time(_, _, _) => true,
            _ => false,
        };
    }

    pub fn is_time_with(&self, hour: u8, minute: u8, second: u8) -> bool {
        return match &self {
            &Token::Time(hour, minute, second) => true,
            _ => false,
        };
    }

    pub fn is_date_time(&self) -> bool {
        return match &self {
            Token::DateTime(_, _, _, _, _, _) => true,
            _ => false,
        };
    }

    pub fn is_date_time_with(&self, year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8)
                             -> bool {
        return match &self {
            &Token::DateTime(year, month, day, hour, minute, second) => true,
            _ => false,
        };
    }

    pub fn is_integer(&self) -> bool {
        return match &self {
            Token::Integer(_) => true,
            _ => false,
        };
    }

    pub fn is_integer_with(&self, int: i16) -> bool {
        return match &self {
            &Token::Integer(int) => true,
            _ => false,
        };
    }

    pub fn is_double(&self) -> bool {
        return match &self {
            Token::Double(_) => true,
            _ => false,
        };
    }

    pub fn is_double_with(&self, dbl: f32) -> bool {
        return match &self {
            &Token::Double(dbl) => true,
            _ => false,
        };
    }

    pub fn is_string(&self) -> bool {
        return match &self {
            Token::String(_) => true,
            _ => false,
        };
    }

    pub fn is_string_with(&self, s: String) -> bool {
        return match &self {
            &Token::String(s) => true,
            _ => false,
        };
    }

    pub fn is_column_option(&self) -> bool {
        return
            self.is_date_time()
                || self.is_double()
                || self.is_integer()
                || self.is_string()
                || self.is_time()
                || self.is_ymd();
    }

    pub fn is_table_option(&self) -> bool {
        return
            self.is_date_time()
                || self.is_double()
                || self.is_integer()
                || self.is_string()
                || self.is_time()
                || self.is_ymd()
                || self.is_name();
    }
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

    // TODO 構文解析しながら必要なものをかき集めるように修正するのと分かりやすいエラーを吐くように修正する
    // 名前。。。  エラーの行数かトークン名確保。。。
    pub fn analyze_syntax(&self) -> Result<Mig, SyntaxError> {
        //println!("\nparse data is:");
        let mut tokens = self.get_tokens();
        if tokens.len() < 5 { return Err(SyntaxError::TooShort); }
        // table name check
        if tokens[0].is_name_colon_with("create".to_string())
            & &tokens[1].is_name()
            & &tokens[2].is_l_mid_paren()
            & &tokens[tokens.len() - 1].is_r_mid_paren()
        {
            let l = tokens.len();
            let mut mig = Mig {};
            let res = analyze_columns_or_table_options(&mut mig, &mut tokens[3..l - 1].to_vec());
            return res.and_then(|mig| Ok(mig.clone()));
        }
        return Err(SyntaxError::UnknownError);
    }
}

fn analyze_columns_or_table_options<'a>(mig: &'a mut Mig, tokens: &mut Vec<Token>) -> Result<&'a mut Mig, SyntaxError> {
    let seq = tokens.clone();
    match &seq[0] {
        // columns is Name { many1 option }
        t @ Token::Name(_) => {
            if seq[1].is_l_mid_paren() {
                let mut separated: Vec<Vec<Token>> = vec!();
                // split at last of first option from first left mid -paren
                for group in seq[2..].splitn(2, |t| t.is_r_mid_paren()) {
                    separated.push(group.to_vec());
                }

                println!("  {:?}:  {:?}", t, separated[0]);

                let column_options = &(separated[0]);
                let others = &(separated[1]);

                if column_options.len() == 0 {
                    return Err(SyntaxError::NoOption(t.clone()));
                }

                if others.len() == 0 {
                    return analyze_column_options(mig, &mut column_options.clone());
                }
                return
                    analyze_column_options(mig, &mut column_options.clone())
                        .and_then(|mig| analyze_columns_or_table_options(mig, &mut others.clone()));
            }
            return Err(SyntaxError::UnknownError);
        }
        // table_option is NameColon { many1 option and option has Name } or NameColon
        t @ Token::NameColon(_) => {
            if seq[1].is_l_mid_paren() {
                let mut separated: Vec<Vec<Token>> = vec!();
                for group in seq[2..].splitn(2, |t| t.is_r_mid_paren()) {
                    separated.push(group.to_vec());
                }

                println!("  {:?}:  {:?}", t, separated[0]);

                let table_options = &(separated[0]);
                let others = &(separated[1]);

                if table_options.len() == 0 {
                    return Err(SyntaxError::NoOption(t.clone()));
                }

                if others.len() == 0 {
                    return analyze_table_options(mig, &mut table_options.clone());
                }
                return
                    analyze_table_options(mig, &mut table_options.clone())
                        .and_then(|mig| analyze_columns_or_table_options(mig, &mut others.clone()));
            }
            println!("  {:?}:  No Options", t);
            let mut seq_dummy = seq[1..].to_vec().clone();
            return analyze_columns_or_table_options(mig, &mut seq_dummy);
        }
        _ => { Err(SyntaxError::UnknownError) }
    }
}

fn analyze_column_options<'a>(mig: &'a mut Mig, column_options: &mut Vec<Token>) -> Result<&'a mut Mig, SyntaxError> {
    let r = split_with_head_and_separator(&column_options, |t| t.is_name_colon());
    //println!("{:?}", r);
    let mut options = r.1.clone();
    let head = r.0;
    if head.is_empty() {
        return analyze_options(mig, &mut options, |t| t.is_column_option());
    }
    return Err(SyntaxError::UnknownOptionName(head[0].clone()));
}

fn analyze_table_options<'a>(mig: &'a mut Mig, table_options: &mut Vec<Token>) -> Result<&'a mut Mig, SyntaxError> {
    let r = split_with_head_and_separator(&table_options, |t| t.is_name_colon());
    //println!("{:?}", r);
    let mut options = r.1.clone();
    let head = r.0;

    if head.is_empty() {
        return analyze_options(mig, &mut options, |t| t.is_table_option());
    }
    return Err(SyntaxError::UnknownOptionName(head[0].clone()));
}

fn analyze_options<'a, F>(mig: &'a mut Mig, options: &mut Vec<(Token, Vec<Token>)>, mut f: F) -> Result<&'a mut Mig, SyntaxError>
    where F: FnMut(&Token) -> bool {
    for (name, option_params) in options.iter() {
        //println!("{:?}", (name, option_params));
        if name.is_name_colon() {
            for param in option_params.iter() {
                if f(&param.clone()) {
                    // TODO store param in mig
                    continue;
                }
                //println!("{:?}", param);
                return Err(SyntaxError::UnknownOptionParam(param.clone()));
            }
            continue;
        }
        return Err(SyntaxError::UnknownOptionName(name.clone()));
    }
    return Ok(mig);
}