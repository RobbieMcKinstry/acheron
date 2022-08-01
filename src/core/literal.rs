use crate::core::condition::{Condition, ConditionEffect, TruthAssignment};
use crate::{Sign, Variable};
use std::fmt;

#[derive(Clone, Copy)]
pub struct Literal {
    var: Variable,
    sign: Sign,
}

impl Literal {
    #[must_use]
    pub fn new(variable: Variable, sign: Sign) -> Self {
        Self {
            var: variable,
            sign,
        }
    }

    #[must_use]
    pub fn apply_condition(self, cond: Condition) -> ConditionEffect {
        if self.var() != cond.var() {
            return ConditionEffect::NoImpact;
        }
        match (self.sign(), cond.assignment()) {
            (Sign::Positive, TruthAssignment::True) | (Sign::Negative, TruthAssignment::False) => {
                ConditionEffect::Sat
            }
            _ => ConditionEffect::Unsat,
        }
    }

    /// satisfying_condition returns the condition
    /// that would satisfy this literal.
    pub fn satisfying_condition(self) -> Condition {
        let var = self.var();
        let assign = TruthAssignment::from(self.sign);
        Condition::new(var, assign)
    }

    #[must_use]
    pub fn var(self) -> Variable {
        self.var
    }

    #[must_use]
    pub fn sign(self) -> Sign {
        self.sign
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
        let var = Variable::from(int);

        Literal { var, sign }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Sign::Negative = self.sign {
            write!(f, "¬")?;
        }
        write!(f, "{}", self.var)
    }
}
