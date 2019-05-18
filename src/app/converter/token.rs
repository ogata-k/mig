use std::cmp::min;

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

    pub fn analyze_syntax(&self) -> Result<Mig, SyntaxError> {
        //println!("\nparse data is:");
        let mut tokens = self.get_tokens();
        if tokens.len() < 5 { return Err(SyntaxError::TooShort); }
        let mut mig = Mig::new();
        // table name check
        if tokens[0].is_name_colon_with("create".to_string())
            & &tokens[1].is_name()
            & &tokens[2].is_l_mid_paren()
            & &tokens[tokens.len() - 1].is_r_mid_paren()
        {
            // set table params of Mig
            mig.set_method(tokens[0].clone());
            mig.set_table_name(tokens[1].clone());

            let l = tokens.len();
            let res = analyze_columns_or_table_options(&mut mig, &mut tokens[3..l - 1].to_vec());
            return res.and_then(|mig| Ok(mig.clone()));
        }
        println!("  {:?}", mig);
        return Err(SyntaxError::UnknownError);
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
                    let ast = Ast::new(method.clone(), table_name.clone());
                    let l = tokens.len();
                    // TODO not impl yet parse_options
                    let ast: Ast = Ast::Program {
                        start: Box::new(Ast::Method {
                            method: Box::new(Ast::String(method.to_string())),
                            table_name: Box::new(Ast::String(table_name.to_string())),
                            table_define: Box::new(Ast::Options {
                                table_define: Box::new(Ast::Set(
                                    parse_options(&tokens[3..l - 1].to_vec())?
                                )),
                            }),
                        })
                    };
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
    let mut stream = tokens.iter();
    let token_opt = stream.next();
    if token_opt.is_none() { return Ok(options); }
    let token = token_opt.unwrap();
    let other_options = match &token {
        _ if token.is_name() => {
            // the token-column-option's option-params
            let column_name = token.clone();
            if let Some(Token::LMidParen) = stream.next() {

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
            } else {
                return Err(SyntaxError::UnknownOptionParam(token.clone()));
            }
        }
        _ if token.is_name_colon() => {
            // TODO the token-table-option's option-params or no option-params
            Vec::new()
        }
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

    /*
        match &seq[0] {
    // columns is Name { many1 option }
            Token::Name(name) => {
                if seq[1].is_l_mid_paren() {
                    let mut separated: Vec<Vec<Token>> = vec!();
    // split at last of first option from first left mid -paren
                    for group in seq[2..].splitn(2, |t| t.is_r_mid_paren()) {
                        separated.push(group.to_vec());
                    }

                    // this column
                    let column_options = &(separated[0]);
                    // other column or table
                    let others = &(separated[1]);

                    if column_options.len() == 0 {
                        return Err(SyntaxError::NoOption(Token::Name(name.to_string())));
                    }

                    let mut col_opt =
                        parse_column_opt(name.to_string(), column_options.clone())?;
                    if others.len() != 0 {
                        col_opt.append(&mut parse_options(&others.clone())?)
                    }
                    return Ok(col_opt);
                }

                return Err(SyntaxError::UnknownError);
            }
    // table_option is NameColon { many1 option and option has Name } or NameColon
            Token::NameColon(name_c) => {
                if seq[1].is_l_mid_paren() {
                    let mut separated: Vec<Vec<Token>> = vec!();
                    for group in seq[2..].splitn(2, |t| t.is_r_mid_paren()) {
                        separated.push(group.to_vec());
                    }

                    // this table
                    let table_options = &(separated[0]);

                    // other tables or columns
                    let others = &(separated[1]);

                    if table_options.len() == 0 {
                        return Err(SyntaxError::NoOption(Token::NameColon(name_c.to_string())));
                    }

                    let mut table_opt =
                        parse_table_opt(name_c.to_string(), table_options.clone())?;
                    if others.len() != 0 {
                        table_opt.append(&mut parse_options(others)?);
                    }
                    return Ok(table_opt);
                }
                // TODO modify when table option has no options
                mig.add_table_options(t.clone(), &mut vec!());
                let mut seq_dummy =
                    seq[1..].to_vec().clone();
                return parse_options(&seq_dummy);
            }
            _ => { Err(SyntaxError::UnknownError) }
        }
    */
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
                    return analyze_column(mig, t.clone(), &mut column_options.clone());
                }
                return
                    analyze_column(mig, t.clone(), &mut column_options.clone())
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
                    return analyze_table(mig, t.clone(), &mut table_options.clone());
                }
                return
                    analyze_table(mig, t.clone(), &mut table_options.clone())
                        .and_then(|mig| analyze_columns_or_table_options(mig, &mut others.clone()));
            }
            println!("  {:?}:  No Options", t);
            mig.add_table_options(t.clone(), &mut vec!());
            let mut seq_dummy = seq[1..].to_vec().clone();
            return analyze_columns_or_table_options(mig, &mut seq_dummy);
        }
        _ => { Err(SyntaxError::UnknownError) }
    }
}

fn analyze_column<'a>(mig: &'a mut Mig, token: Token, column_options: &mut Vec<Token>) -> Result<&'a mut Mig, SyntaxError> {
    let r = split_with_head_and_separator(&column_options, |t| t.is_name_colon());
//println!("{:?}", r);
    let mut options = r.1.clone();
    let head = r.0;
    if head.is_empty() {
        return analyze_column_options(mig, token, &mut options);
    }
    return Err(SyntaxError::UnknownOptionName(head[0].clone()));
}

fn analyze_table<'a>(mig: &'a mut Mig, token: Token, table_options: &mut Vec<Token>) -> Result<&'a mut Mig, SyntaxError> {
    let r = split_with_head_and_separator(&table_options, |t| t.is_name_colon());
//println!("{:?}", r);
    let mut options = r.1.clone();
    let head = r.0;

    if head.is_empty() {
        return analyze_table_options(mig, token, &mut options);
    }
    return Err(SyntaxError::UnknownOptionName(head[0].clone()));
}

fn analyze_table_options<'a>(mig: &'a mut Mig, token: Token, options: &mut Vec<(Token, Vec<Token>)>) -> Result<&'a mut Mig, SyntaxError> {
    let mut token_s: Vec<(String, Vec<Token>)> = vec!();
    for (name, option_params) in options.iter() {
//println!("{:?}", (name, option_params));
        if name.is_name_colon() {
            for param in option_params.iter() {
                if param.is_table_option() {
                    continue;
                }
//println!("{:?}", param);
                return Err(SyntaxError::UnknownOptionParam(param.clone()));
            }

            match name {
                Token::NameColon(s) => token_s.push((s.to_string(), option_params.clone())),
                _ => unreachable!(),
            }
            continue;
        }
        return Err(SyntaxError::UnknownOptionName(name.clone()));
    }

    mig.add_table_options(token, &mut token_s);
    return Ok(mig);
}

fn analyze_column_options<'a>(mig: &'a mut Mig, token: Token, options: &mut Vec<(Token, Vec<Token>)>) -> Result<&'a mut Mig, SyntaxError> {
    let mut token_s: Vec<(String, Vec<Token>)> = vec!();
    for (name, option_params) in options.iter() {
//println!("{:?}", (name, option_params));
        if name.is_name_colon() {
            for param in option_params.iter() {
                if param.is_column_option() {
                    continue;
                }
//println!("{:?}", param);
                return Err(SyntaxError::UnknownOptionParam(param.clone()));
            }

            match name {
                Token::NameColon(s) => token_s.push((s.to_string(), option_params.clone())),
                _ => unreachable!(),
            }
            continue;
        }
        return Err(SyntaxError::UnknownOptionName(name.clone()));
    }
    mig.add_column_options(token, &mut token_s);
    return Ok(mig);
}
