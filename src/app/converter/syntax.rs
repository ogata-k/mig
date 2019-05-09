use std::fmt::{Display, Error, Formatter};

#[derive(debug)]
pub enum SyntaxError {}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            // enum item to string with write!(f, "", item);
        }
    }
}