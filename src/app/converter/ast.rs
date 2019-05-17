use std::collections::{HashMap, HashSet};

use crate::app::converter::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Program { start: Box<Ast> },
    Method { method: Box<Ast>, table_name: Box<Ast>, table_define: Box<Ast> },
    Options { table_define: Box<Ast> },
    Set(Vec<Box<Ast>>),
    // option_param: Set
    ColumnOption { option_name: Box<Ast>, option_param: Box<Ast> },
    TableOption { option_name: Box<Ast>, option_param: Box<Ast> },
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

