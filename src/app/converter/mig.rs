use std::collections::HashMap;
use std::iter::Map;

use crate::app::converter::token::Token;

#[derive(Debug, Clone)]
pub struct Mig {
    method: String,
    table_name: String,
    column_options: HashMap<String, Vec<Token>>,
    table_options: HashMap<String, Vec<Token>>,
}

impl Mig {
    pub fn new() -> Self {
        Mig {
            method: "".to_string(),
            table_name: "".to_string(),
            column_options: HashMap::new(),
            table_options: HashMap::new(),
        }
    }

    pub fn set_method(&mut self, method_name: String) -> &mut Self {
        self.method = method_name;
        return self;
    }

    pub fn set_table_name(&mut self, table_name: String) -> &mut Self {
        self.table_name = table_name;
        return self;
    }

    pub fn add_column_options(&mut self, column_name: String, options: &mut Vec<Token>) -> &mut Self {
        if self.column_options.get(&column_name).is_none() {
            self.column_options.insert(column_name, options.clone());
        } else {
            let mut tokens = self.column_options.get(&column_name).unwrap().clone();
            tokens.append(options);
            self.column_options.insert(column_name, tokens);
        }
        return self;
    }

    pub fn add_table_options(&mut self, table_name: String, options: &mut Vec<Token>) -> &mut Self {
        if self.table_options.get(&table_name).is_none() {
            self.table_options.insert(table_name, options.clone());
        } else {
            let mut tokens = self.table_options.get(&table_name).unwrap().clone();
            tokens.append(options);
            self.table_options.insert(table_name, tokens);
        }
        return self;
    }
}
