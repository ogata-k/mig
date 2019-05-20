use crate::app::converter::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Program { start: Box<Ast> },
    Method { method: Box<Ast>, table_name: Box<Ast>, table_define: Box<Ast> },
    Set(Vec<Box<Ast>>),
    // option_param: Set
    // option_name: String, option_params: Set(Param)
    ColumnOption { option_name: Box<Ast>, option_params: Box<Ast> },
    TableOption { option_name: Box<Ast>, option_params: Box<Ast> },
    Param { param_name: Box<Ast>, param_options: Box<Ast> },
    Ymd(Box<Ast>, Box<Ast>, Box<Ast>),
    Time(Box<Ast>, Box<Ast>, Box<Ast>),
    DateTime(Box<Ast>, Box<Ast>, Box<Ast>, Box<Ast>, Box<Ast>, Box<Ast>),
    UnsignedInteger(u16),
    Integer(i16),
    Double(f32),
    String(String),
    ColumnString(String),
    None,
}

pub trait ToAst {
    fn to_ast(&self) -> Ast;
}

impl ToAst for u16 {
    fn to_ast(&self) -> Ast {
        return Ast::UnsignedInteger(self.clone());
    }
}

impl ToAst for u8 {
    fn to_ast(&self) -> Ast {
        return Ast::UnsignedInteger(self.clone() as u16);
    }
}

impl ToAst for i16 {
    fn to_ast(&self) -> Ast {
        return Ast::Integer(self.clone());
    }
}

impl ToAst for f32 {
    fn to_ast(&self) -> Ast {
        return Ast::Double(self.clone());
    }
}

impl ToAst for String {
    fn to_ast(&self) -> Ast {
        return Ast::String(self.to_string());
    }
}

// TODO SequenceからAstへの変換とAstの最適化(結果はMig構造体という名前に）した木への変換を実装する。
// TODO 最終的に木を走査しながら文字列へ変換する
impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match &self {
            Ast::Program { start: start } => { write!(f, "AST:{{\n{}\n}}", start) }
            Ast::Method { method: method, table_name: name, table_define: body } => {
                write!(f, "{} {} {{{}\n}}", method, name, body)
            }
            Ast::Set(set) => {
                for ast in set.iter() {
                    write!(f, "\n{}", ast);
                }
                Ok(())
            }
            Ast::ColumnOption { option_name: name, option_params: params } => {
                write!(f, "column: {} {{", name);
                write!(f, "{}\n}}", params)
            }
            Ast::TableOption { option_name: name, option_params: params } => {
                write!(f, "table: {} {{", name);
                write!(f, "{}\n}}", params)
            }
            Ast::Param { param_name: name, param_options: options } => {
                write!(f, "param: {}", name);
                write!(f, "{}", options)
            }
            Ast::Ymd(y, m, d) => {
                write!(f, "{}-{}-{}", y, m, d)
            }
            Ast::Time(h, m, d) => {
                write!(f, "{}:{}:{}", h, m, d)
            }
            Ast::DateTime(y, mon, d, h, min, s) => {
                write!(f, "{}-{}-{}_{}:{}:{}", y, mon, d, h, min, s)
            }
            Ast::UnsignedInteger(i) => { write!(f, "{}", i) }
            Ast::Integer(i) => { write!(f, "{}", i) }
            Ast::Double(d) => { write!(f, "{}", d) }
            Ast::String(s) => { write!(f, "\"{}\"", s) }
            Ast::ColumnString(s) => { write!(f, "_{}_", s) }
            Ast::None => { write!(f, "None") }
        }
    }
}

impl Ast {
    pub fn new_ast(method: String, table_name: String, table_body: Vec<Box<Ast>>) -> Self {
        Ast::Program {
            start: Box::new(Ast::Method {
                method: Box::new(Ast::String(method)),
                table_name: Box::new(Ast::String(table_name)),
                table_define: Box::new(Ast::Set(table_body)),
            })
        }
    }

    pub fn new_column_option(column_name: Token, body: Vec<(Token, Vec<Token>)>) -> Self {
        let body_ast = to_vec_of_ast(body);
        return Ast::ColumnOption {
            option_name: Box::new(column_name.to_ast()),
            option_params: Box::new(Ast::Set(body_ast)),
        };
    }

    pub fn new_table_option(column_name: Token, body: Vec<(Token, Vec<Token>)>) -> Self {
        let body_ast = to_vec_of_ast(body);
        return Ast::TableOption {
            option_name: Box::new(column_name.to_ast()),
            option_params: Box::new(Ast::Set(body_ast)),
        };
    }

    pub fn check_syntax(&self) -> bool {
        return match self {
            Ast::Program { start: method } => {
                method.check_method()
            }
            _ => false
        };
    }

    fn check_method(&self) -> bool {
        return match self {
            Ast::Method { method, table_name, table_define } => {
                method.check_string()
                    && table_name.check_string()
                    && table_define.check_options()
            }
            _ => false,
        };
    }

    fn check_options(&self) -> bool {
        return match self {
            Ast::Set(options) => {
                options.iter().all(|option|
                    option.check_column_option() || option.check_table_option()
                )
            }
            _ => false,
        };
    }

    fn check_column_option(&self) -> bool {
        return match self {
            Ast::ColumnOption { option_name, option_params } => {
                option_name.check_column_name() && option_params.check_column_params()
            }
            _ => false,
        };
    }

    fn check_column_name(&self) -> bool {
        return self.check_column_string();
    }

    fn check_column_params(&self) -> bool {
        return match self {
            Ast::Set(params) => {
                if params.is_empty() {
                    false
                } else {
                    params.iter().all(|param|
                        param.check_column_param()
                    )
                }
            }
            _ => false,
        };
    }

    fn check_column_param(&self) -> bool {
        return match self {
            Ast::Param { param_name, param_options } => {
                param_name.check_string()
                    && param_options.check_column_option_param_options()
            }
            _ => false,
        };
    }

    fn check_column_option_param_options(&self) -> bool {
        return match self {
            Ast::Set(options) => {
                options.iter().all(|option|
                    option.check_column_option_param_option()
                )
            }
            _ => false,
        };
    }

    fn check_column_option_param_option(&self) -> bool {
        return
            self.check_date_time()
                || self.check_double()
                || self.check_integer()
                || self.check_string()
                || self.check_time()
                || self.check_ymd();
    }

    fn check_table_option(&self) -> bool {
        return match self {
            Ast::TableOption { option_name, option_params } => {
                option_name.check_table_name() && option_params.check_table_params()
            }
            _ => false,
        };
    }

    fn check_table_name(&self) -> bool {
        return self.check_string();
    }

    fn check_table_params(&self) -> bool {
        return match self {
            Ast::Set(params) => {
                params.iter().all(|param|
                    param.check_table_param()
                )
            }
            _ => false,
        };
    }

    fn check_table_param(&self) -> bool {
        return match self {
            Ast::Param { param_name, param_options } => {
                param_name.check_string()
                    && param_options.check_table_option_param_options()
            }
            _ => false,
        };
    }

    fn check_table_option_param_options(&self) -> bool {
        return match self {
            Ast::Set(options) => {
                options.iter().all(|option|
                    option.check_table_option_param_option()
                )
            }
            _ => false,
        };
    }

    fn check_table_option_param_option(&self) -> bool {
        return
            self.check_date_time()
                || self.check_double()
                || self.check_integer()
                || self.check_string()
                || self.check_time()
                || self.check_ymd()
                || self.check_column_string();
    }

    fn check_ymd(&self) -> bool {
        return match self {
            Ast::Ymd(y, m, d) => {
                y.check_unsigned_integer()
                    && m.check_unsigned_integer()
                    && d.check_unsigned_integer()
            }
            _ => false,
        };
    }
    fn check_time(&self) -> bool {
        return match self {
            Ast::Time(h, m, s) => {
                h.check_unsigned_integer()
                    && m.check_unsigned_integer()
                    && s.check_unsigned_integer()
            }
            _ => false,
        };
    }
    fn check_date_time(&self) -> bool {
        return match self {
            Ast::DateTime(y, mon, d, h, min, s) => {
                y.check_unsigned_integer()
                    && mon.check_unsigned_integer()
                    && d.check_unsigned_integer()
                    && h.check_unsigned_integer()
                    && min.check_unsigned_integer()
                    && s.check_unsigned_integer()
            }
            _ => false,
        };
    }
    fn check_unsigned_integer(&self) -> bool {
        return match self {
            Ast::UnsignedInteger(_) => true,
            _ => false,
        };
    }
    fn check_integer(&self) -> bool {
        return match self {
            Ast::Integer(_) => true,
            _ => false,
        };
    }
    fn check_double(&self) -> bool {
        return match self {
            Ast::Double(_) => true,
            _ => false
        };
    }
    fn check_column_string(&self) -> bool {
        return match self {
            Ast::ColumnString(_) => true,
            _ => false,
        };
    }
    fn check_string(&self) -> bool {
        return match self {
            Ast::String(_) => true,
            _ => false,
        };
    }
    fn check_none(&self) -> bool {
        return false;
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
