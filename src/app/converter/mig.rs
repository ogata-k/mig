use std::collections::HashMap;
use std::iter::Map;

use crate::app::converter::generator::Generator;
use crate::app::converter::token::Token;
use crate::app::framework::Framework;

#[derive(Debug, Clone)]
pub struct Mig {
    method: String,
    table_name: String,
    column_options: HashMap<String, Vec<(String, Vec<Token>)>>,
    table_options: HashMap<String, Vec<(String, Vec<Token>)>>,
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

    pub fn set_method(&mut self, method_name: Token) -> &mut Self {
        match method_name {
            Token::NameColon(s) => self.method = s.clone(),
            _ => panic!("set method"),
        };
        return self;
    }

    pub fn set_table_name(&mut self, table_name: Token) -> &mut Self {
        match table_name {
            Token::Name(s) => self.table_name = s.clone(),
            _ => panic!("set table name"),
        }
        return self;
    }

    pub fn add_column_options(&mut self, column_name: Token, options: &mut Vec<(String, Vec<Token>)>) -> &mut Self {
        match column_name {
            Token::Name(name) => {
                if self.column_options.get(&name).is_none() {
                    self.column_options.insert(name, options.clone());
                } else {
                    let mut tokens = self.column_options.get(&name).unwrap().clone();
                    tokens.append(options);
                    self.column_options.insert(name, tokens.to_vec());
                }
            },
            _ => panic!("add column options"),
        }
        return self;
    }

    pub fn add_table_options(&mut self, option_name: Token, options: &mut Vec<(String, Vec<Token>)>) -> &mut Self {
        match option_name {
            Token::NameColon(name) => {
                if self.table_options.get(&name).is_none() {
                    self.table_options.insert(name, options.clone());
                } else {
                    let mut tokens = self.table_options.get(&name).unwrap().clone();
                    tokens.append(options);
                    self.table_options.insert(name, tokens.to_vec());
                }
            },
            _ => panic!("add table options"),
        }
        return self;
    }

    pub fn generate_string_for(&self, fw: Framework, name_space: String) -> String {
        match fw {
            Framework::Laravel => Laravel {}.generate(self, name_space),
        }
    }
}

struct Laravel {}

impl Generator for Laravel {
    fn gen_header(&self, mig: &Mig, name_space: String) -> String {
        unimplemented!()
    }

    fn gen_column_options(&self, mig: &Mig) -> String {
        unimplemented!()
    }

    fn gen_table_options(&self, mig: &Mig) -> String {
        unimplemented!()
    }

    fn gen_footer(&self, mig: &Mig) -> String {
        unimplemented!()
    }

    // TODO remove after impl other minimum function
    fn generate(&self, mig: &Mig, name_space: String) -> String {
        return format!("{:?}", mig);
    }
}