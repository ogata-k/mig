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
        println!("\nparse data is:");
        let mut tokens = self.get_tokens();
        if tokens.len() < 5 { return false; }
        // table name check
        if tokens[0].is_name_colon_with("create".to_string())
            & &tokens[1].is_name()
            & &tokens[2].is_l_mid_paren()
            & &tokens[tokens.len() - 1].is_r_mid_paren()
        {
            let l = tokens.len();
            return check_columns_or_table_options(&mut tokens[3..l - 1].to_vec());
        }
        return false;
    }
}

fn check_columns_or_table_options(tokens: &mut Vec<Token>) -> bool {
    let mut seq = tokens.clone();
    let b: bool = match &seq[0] {
        // TODO columns or table_options
        // columns is Name { many1 option }
        t @ Token::Name(_) => {
            if seq[1].is_l_mid_paren() {
                let mut separated: Vec<Vec<Token>> = vec!();
                for group in seq[2..].splitn(2, |t| t.is_r_mid_paren()) {
                    separated.push(group.to_vec());
                }

                println!("{:?}: {:?}", t, separated[0]);

                let column_options = &(separated[0]);
                let others = &(separated[1]);

                if column_options.len() == 0 {
                    return false;
                }

                if others.len() == 0 {
                    return check_column_options(&mut column_options.clone());
                }
                return
                    check_column_options(&mut column_options.clone())
                        && check_columns_or_table_options(&mut others.clone());
            }
            false
        }
        // table_option is NameColon { many1 option and option has Name } or NameColon
        t @ Token::NameColon(_) => {
            if seq[1].is_l_mid_paren() {
                let mut separated: Vec<Vec<Token>> = vec!();
                for group in seq[2..].splitn(2, |t| t.is_r_mid_paren()) {
                    separated.push(group.to_vec());
                }

                println!("{:?}: {:?}", t, separated[0]);

                let table_options = &(separated[0]);
                let others = &(separated[1]);

                if table_options.len() == 0 {
                    return false;
                }

                if others.len() == 0 {
                    return check_table_options(&mut table_options.clone());
                }
                return
                    check_table_options(&mut table_options.clone())
                        && check_columns_or_table_options(&mut others.clone());
            }
            println!("{:?}: No Options", t);
            let mut seq_dummy = seq[1..].to_vec().clone();
            return check_columns_or_table_options(&mut seq_dummy);
        }
        _ => { false }
    };
    return b;
}

fn check_column_options(column_options: &mut Vec<Token>) -> bool {
    // TODO check options
    return true;
}

fn check_table_options(column_options: &mut Vec<Token>) -> bool {
    // TODO check options
    return true;
}
