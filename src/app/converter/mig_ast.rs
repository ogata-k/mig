use std::collections::{HashMap, HashSet};

use crate::app::converter::token::Token;

/// AST
#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Method { method: String, table_name: String, table_define: Box<Ast> },
    // HashMap<option_name, options>
    Options { table_define: HashMap<String, Vec<Box<Ast>>> },
    // column and table options
    Option { option_type: OptionType, option_name: String, option_param: Vec<Ast> },
    Token(Token),
}
// TODO SequenceからAstへの変換とAstの最適化(結果はMig構造体という名前に）した木への変換を実装する。
// TODO 最終的に木を走査しながら文字列へ変換する

#[derive(Debug, Clone, PartialEq)]
pub enum OptionType {
    Column,
    Table,
}