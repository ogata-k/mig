#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Program { start: Box<Ast> },
    Method { method: Box<Ast>, table_name: Box<Ast>, table_define: Box<Ast> },
    Options { table_define: Box<Ast> },
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
            Ast::Program { start: start } => { write!(f, "AST:{{\n{}\n}}", start) },
            Ast::Method { method: method, table_name: name, table_define: body } => {
                write!(f, "{} {} {{{}\n}}", method, name, body)
            },
            Ast::Options { table_define: options } => { write!(f, "{}", options) },
            Ast::Set(set) => {
                for ast in set.iter() {
                    write!(f, "\n{}", ast);
                }
                Ok(())
            },
            Ast::ColumnOption { option_name: name, option_params: params } => {
                write!(f, "column: {} {{", name);
                write!(f, "{}\n}}", params)
            },
            Ast::TableOption { option_name: name, option_params: params } => {
                write!(f, "table: {} {{", name);
                write!(f, "{}\n}}", params)
            },
            Ast::Param { param_name: name, param_options: options } => {
                write!(f, "param: {}", name);
                write!(f, "{}", options)
            },
            Ast::Ymd(y, m, d) => {
                write!(f, "{}-{}-{}", y, m, d)
            },
            Ast::Time(h, m, d) => {
                write!(f, "{}:{}:{}", h, m, d)
            },
            Ast::DateTime(y, mon, d, h, min, s) => {
                write!(f, "{}-{}-{}_{}:{}:{}", y, mon, d, h, min, s)
            },
            Ast::UnsignedInteger(i) => { write!(f, "{}", i) },
            Ast::Integer(i) => { write!(f, "{}", i) },
            Ast::Double(d) => { write!(f, "{}", d) },
            Ast::String(s) => { write!(f, "\"{}\"", s) },
            Ast::ColumnString(s) => { write!(f, "_{}_", s) }
            Ast::None => { write!(f, "None") },
        }
    }
}

impl Ast {
    pub fn new(method: String, table_name: String) -> Self {
        Ast::Program {
            start: Box::new(Ast::Method {
                method: Box::new(Ast::String(method)),
                table_name: Box::new(Ast::String(table_name)),
                table_define: Box::new(Ast::Set(Vec::new())),
            })
        }
    }
}

