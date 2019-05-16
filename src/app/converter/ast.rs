use std::collections::{HashMap, HashSet};

use crate::app::converter::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Program { start: Box<Ast> },
    Method { method: Box<Ast>, table_name: Box<Ast>, table_define: Box<Ast> },
    // HashMap<option_name, options_set>
    Options { table_define: HashMap<String, Box<Ast>> },
    // option_param: Set
    ColumnOption { option_name: Box<Ast>, option_param: Box<Ast> },
    TableOption { option_name: Box<Ast>, option_param: Box<Ast> },
    Set(Vec<Box<Ast>>),  // use Vec as Set, so unique necessary!!!!!!!!!
    Ymd(Box<Ast>, Box<Ast>, Box<Ast>),
    Time(Box<Ast>, Box<Ast>, Box<Ast>),
    DateTime(Box<Ast>, Box<Ast>, Box<Ast>, Box<Ast>, Box<Ast>, Box<Ast>),
    UnsignedInteger(u16),
    Integer(i16),
    Double(f32),
    String(String),
}
// TODO SequenceからAstへの変換とAstの最適化(結果はMig構造体という名前に）した木への変換を実装する。
// TODO 最終的に木を走査しながら文字列へ変換する

