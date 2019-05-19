use crate::app::converter::ast::{Ast, ToAst};
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

impl ToAst for Token {
    fn to_ast(&self) -> Ast {
        match self {
            Token::LMidParen => { Ast::None }
            Token::RMidParen => { Ast::None }
            Token::Name(s) => { Ast::ColumnString(s.to_string()) }
            Token::NameColon(s) => { s.to_string().to_ast() }
            Token::Ymd(y, m, d) => {
                Ast::Ymd(Box::new(y.to_ast()), Box::new(m.to_ast()), Box::new(d.to_ast()))
            }
            Token::Time(h, m, s) => {
                Ast::Time(Box::new(h.to_ast()), Box::new(m.to_ast()), Box::new(s.to_ast()))
            }
            Token::DateTime(y, m, d, hour, min, sec) => {
                Ast::DateTime(
                    Box::new(y.to_ast()),
                    Box::new(m.to_ast()),
                    Box::new(d.to_ast()),
                    Box::new(hour.to_ast()),
                    Box::new(min.to_ast()),
                    Box::new(sec.to_ast()),
                )
            }
            Token::Integer(i) => { i.to_ast() }
            Token::Double(d) => { d.to_ast() }
            Token::String(s) => { s.to_ast() }
        }
    }
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
            Token::Name(s) if *s == name => true,
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
            &Token::NameColon(s) if *s == name_colon => true,
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
            &Token::Ymd(y, m, d)
            if (*y == year && *m == month && *d == day)
            => true,
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
            &Token::Time(h, m, s)
            if (*h == hour && *m == minute && *s == second)
            => true,
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
            &Token::DateTime(y, mon, d, h, min, s)
            if (*y == year && *mon == month && *d == day && *h == hour && *min == minute && *s == second)
            => true,
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
            &Token::Integer(i) if *i == int => true,
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
            &Token::Double(d) if *d == dbl => true,
            _ => false,
        };
    }

    pub fn is_string(&self) -> bool {
        return match &self {
            Token::String(_) => true,
            _ => false,
        };
    }

    pub fn is_string_with(&self, string: String) -> bool {
        return match &self {
            &Token::String(s) if *s == string => true,
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

#[derive(Debug, Clone)]
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

    // TODO to change for Ast
    // TODO 雑多な解析木->構造確認->最適化->AST->(Migオプション名, 引数の個数, FW用の名前)からなるJsonを利用した最終確認->Mig
    pub fn parse(&self) -> Result<Ast, SyntaxError> {
        let tokens = self.get_tokens().clone();
        if tokens.len() < 5 { return Err(SyntaxError::TooShort); }
        // table name check
        match (&tokens[0], &tokens[1], &tokens[2], &tokens[tokens.len() - 1])
            {
                (Token::NameColon(method), Token::Name(table_name), Token::LMidParen, Token::RMidParen)
                => {
                    // set table params of Mig
                    let l = tokens.len();
                    let ast = Ast::new(
                        method.clone(),
                        table_name.clone(),
                        parse_options(&tokens[3..l - 1].to_vec())?,
                    );
                    return Ok(ast);
                }
                _ => {
                    return Err(SyntaxError::UnknownError);
                }
            }
    }
}

fn to_vec_of_ast(v: Vec<(Token, Vec<Token>)>) -> Vec<Box<Ast>> {
    return v.into_iter().map(|(param_name, params)| {
        let params_ast = (&params).into_iter().map(|t| Box::new(t.to_ast())
        ).collect::<Vec<Box<Ast>>>();
        return (param_name.to_ast(), params_ast);
    }).collect::<Vec<(Ast, Vec<Box<Ast>>)>>()
        .into_iter().map(|(name, options)| (
        Box::new(Ast::Param { param_name: Box::new(name), param_options: Box::new(Ast::Set(options)) })
    )).collect::<Vec<Box<Ast>>>();
}

fn parse_options_recursive<'a>(tokens: &Vec<Token>, options: &'a mut Vec<Box<Ast>>) -> Result<&'a mut Vec<Box<Ast>>, SyntaxError> {
    let mut stream = tokens.iter().peekable();
    let token_opt = stream.next();
    if token_opt.is_none() { return Ok(options); }
    let token = token_opt.unwrap();
    let other_options = match &token {
        // TODO I want to separate these match patterns as function
        _ if token.is_name() => {
            let body_empty_ok = false;

            // the token-column-option's option-params
            let column_name = token.clone();
            if let Some(Token::LMidParen) = stream.peek() {
                stream.next();
                // split to (target column and the options, others)
                let mut separated: Vec<Vec<Token>> = vec!();
// split at last of first option from first left mid -paren
                let stream_dummy: Vec<Token> = stream.clone().map(|t| t.clone()).collect();
                for group in stream_dummy.splitn(2, |t| t.is_r_mid_paren()) {
                    separated.push(group.to_vec());
                }
                let mut column_body = separated[0].clone();
                let mut other_options = separated[1].clone();
                if column_body.len() == 0 {
                    return Err(SyntaxError::NoOption(column_name));
                }


                // to Ast from checked body
                let (head, body) = split_with_head_and_separator(&mut column_body, |t| t.is_name_colon());
                if head.len() != 0 {
                    // pattern column_body is:      hogehoge :param1 param_opt1 param_opt2
                    return Err(SyntaxError::UnknownOptionParam(head[0].clone()));
                }
                if body.len() == 0 {
                    return Err(SyntaxError::NoOption(column_name));
                }
                let body_ast = to_vec_of_ast(body);

                // set the checked column option with the params
                let column_opt = Box::new(
                    Ast::ColumnOption {
                        option_name: Box::new(column_name.to_ast()),
                        option_params: Box::new(Ast::Set(body_ast)),
                    }
                );
                options.push(column_opt.clone());

                // //////////////
                // update stream
                other_options
            } else if body_empty_ok {
                // possible, when option has no-params
                let opt = Ast::ColumnOption {
                    option_name: Box::new(column_name.to_ast()),
                    option_params: Box::new(Ast::Set(Vec::new())),
                };
                options.push(Box::new(opt));
                return parse_options_recursive(&stream.map(|t| t.clone()).collect(), options);
            } else {
                return Err(SyntaxError::UnknownOptionParam(token.clone()));
            }
        },
        _ if token.is_name_colon() => {
            let body_empty_ok = true;

            // the token-column-option's option-params
            let column_name = token.clone();
            if let Some(Token::LMidParen) = stream.peek() {
                stream.next();
                // split to (target column and the options, others)
                let mut separated: Vec<Vec<Token>> = vec!();
// split at last of first option from first left mid -paren
                let stream_dummy: Vec<Token> = stream.clone().map(|t| t.clone()).collect();
                for group in stream_dummy.splitn(2, |t| t.is_r_mid_paren()) {
                    separated.push(group.to_vec());
                }
                let mut column_body = separated[0].clone();
                let mut other_options = separated[1].clone();
                if column_body.len() == 0 {
                    return Err(SyntaxError::NoOption(column_name));
                }


                // to Ast from checked body
                let (head, body) = split_with_head_and_separator(&mut column_body, |t| t.is_name_colon());
                if head.len() != 0 {
                    // pattern column_body is:      hogehoge :param1 param_opt1 param_opt2
                    return Err(SyntaxError::UnknownOptionParam(head[0].clone()));
                }
                if body.len() == 0 {
                    return Err(SyntaxError::NoOption(column_name));
                }
                let body_ast = to_vec_of_ast(body);

                // set the checked column option with the params
                let column_opt = Box::new(
                    Ast::TableOption {
                        option_name: Box::new(column_name.to_ast()),
                        option_params: Box::new(Ast::Set(body_ast)),
                    }
                );
                options.push(column_opt.clone());

                // //////////////
                // update stream
                other_options
            } else if body_empty_ok {
                // possible, when option has no-params
                let opt = Ast::TableOption {
                    option_name: Box::new(column_name.to_ast()),
                    option_params: Box::new(Ast::Set(Vec::new())),
                };
                options.push(Box::new(opt));
                return parse_options_recursive(&stream.map(|t| t.clone()).collect(), options);
            } else {
                return Err(SyntaxError::UnknownOptionParam(token.clone()));
            }
        },
        _ => {
            return Err(SyntaxError::UnknownError);
        }
    };
    return parse_options_recursive(&other_options, options);
}

fn parse_options(tokens: &Vec<Token>) -> Result<Vec<Box<Ast>>, SyntaxError> {
    let mut options: Vec<Box<Ast>> = Vec::new();
    parse_options_recursive(tokens, &mut options);
    return Ok(options);
}
