use crate::variable::Var;
use im::Vector;
use std::fmt;

pub struct Clause {
    variables: Vector<Var>,
}

impl Clause {
    pub fn new() -> Self {
        Clause {
            variables: Vector::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }

    pub fn add_variable(&self, var: String) -> Self {
        let variable = Var::from(var);
        let mut next = self.variables.clone();
        next.push_back(variable);
        Clause { variables: next }
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in self.variables.iter() {
            write!(f, "{},", v)?
        }
        Ok(())
    }
}
