use crate::Sign;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Var {
    id: u64,
    sign: Sign,
}

impl From<String> for Var {
    fn from(string: String) -> Self {
        println!("{}", string.clone());
        let integer = string.parse::<i64>().expect("string should be an integer");
        if integer < 0 {
            Var {
                id: -integer as u64,
                sign: Sign::Negative,
            }
        } else {
            Var {
                id: integer as u64,
                sign: Sign::Positive,
            }
        }
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.sign {
            Sign::Positive => write!(f, "+")?,
            Sign::Negative => write!(f, "-")?,
        }
        write!(f, "{}", self.id)?;
        Ok(())
    }
}
