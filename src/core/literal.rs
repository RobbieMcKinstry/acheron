use crate::{Sign, Variable};
use std::fmt;

#[derive(Clone, Copy)]
pub struct Literal {
    id: Variable,
    sign: Sign,
}

impl Literal {
    #[must_use]
    pub fn new(variable: Variable, sign: Sign) -> Self {
        Self { id: variable, sign }
    }

    #[must_use]
    pub fn variable(self) -> Variable {
        self.id
    }

    #[must_use]
    pub fn sign(self) -> Sign {
        self.sign
    }

    #[must_use]
    pub fn matching_variable(self, other: Self) -> bool {
        self.variable() == other.variable()
    }

    #[must_use]
    pub fn matching_sign(self, other: Self) -> bool {
        self.sign() == other.sign()
    }
}

impl From<String> for Literal {
    #[allow(clippy::cast_sign_loss)]
    fn from(string: String) -> Self {
        println!("{}", string);
        let integer = string.parse::<i64>().expect("string should be an integer");
        let (int, sign) = if integer < 0 {
            (-integer as u64, Sign::Negative)
        } else {
            (integer as u64, Sign::Positive)
        };
        let id = Variable::from(int);

        Literal { id, sign }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.sign {
            Sign::Positive => write!(f, "+")?,
            Sign::Negative => write!(f, "-")?,
        }
        write!(f, "{}", self.id)?;
        Ok(())
    }
}
