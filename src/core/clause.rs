use crate::{Condition, Literal, Status, Variable};
use im::Vector;
use std::fmt;

#[derive(Clone)]
pub struct Clause {
    literals: Vector<Literal>,
    status: Status,
}

impl Clause {
    #[must_use]
    pub fn new() -> Self {
        Clause {
            literals: Vector::new(),
            status: Status::Unknown,
        }
    }

    /// `assign` will adjust this clause
    /// according to the truth-assignment provided.
    #[must_use]
    pub fn assign(&self, cond: Condition) -> Self {
        let mut clause = Self::new();
        for literal in self.literals.iter() {
            let same_var = literal.matching_variable(lit);
            let same_sign = literal.matching_sign(lit);
            match (same_var, same_sign) {
                // Satisfied!
                (true, true) => {
                    clause.status = Status::Sat;
                    clause.literals.push_back(*literal);
                }
                // No match. Copy into next formula.
                (false, true | false) => clause.literals.push_back(*literal),
                // Single disjunction is unsat. Do not carry over.
                (true, false) => continue,
            }
        }
        clause.set_unsat();
        clause
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }

    #[must_use]
    pub fn add_literal(&self, lit: String) -> Self {
        let literal = Literal::from(lit);
        let mut next = self.literals.clone();
        next.push_back(literal);
        Clause {
            literals: next,
            status: Status::Unknown,
        }
    }

    /// `is_unit` returns true if this clause is a unit clause.
    /// A clause is a unit clause if it has exactly one literal.
    #[must_use]
    pub fn unit(&self) -> Option<Literal> {
        if self.literals.len() == 1 {
            Some(self.literals[0])
        } else {
            None
        }
    }

    /// `is_sat` returns whether this clause
    /// is satisfied, unsatisfied, or of unknown
    /// satisfiability.
    #[must_use]
    pub fn is_sat(&self) -> Status {
        self.status
    }

    /// `set_sat` transitions this clause to unsat
    /// if there are no literals left which could be
    /// satisfied.
    fn set_unsat(&mut self) {
        if self.is_empty() {
            self.status = Status::Unsat;
        }
    }

    fn set_sat(&mut self) {
        self.status = Status::Unsat;
    }

    #[must_use]
    pub fn select_random_variable(&self) -> Option<Variable> {
        if self.is_empty() {
            return None;
        }
        Some(self.literals[0].variable())
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in self.literals.iter() {
            write!(f, "{},", v)?;
        }
        Ok(())
    }
}

impl Default for Clause {
    fn default() -> Self {
        Self::new()
    }
}
