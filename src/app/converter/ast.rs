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
    None,
}

pub trait ToAst{
    fn to_ast(&self)->Ast;
}

impl ToAst for u16 {
    fn to_ast(&self) -> Ast{
        return Ast::UnsignedInteger(self.clone());
    }
}

impl ToAst for u8 {
    fn to_ast(&self) -> Ast{
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

