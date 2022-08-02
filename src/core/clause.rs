use crate::core::condition::{Condition, ConditionEffect};
use crate::core::{Literal, Variable};
use crate::Status;
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
            match literal.apply_condition(cond) {
                ConditionEffect::Sat => {
                    // Satisfied!
                    clause.status = Status::Sat;
                    clause.literals.push_back(*literal);
                }
                ConditionEffect::Unsat => {
                    // Matches variable but not sign!
                    // Remove this literal from the clause.
                    clause.literals.push_back(*literal)
                }
                ConditionEffect::NoImpact => {
                    // Literal's variable does not appear in condition.
                    // Keep literal in clause.
                    continue;
                }
            }
        }
        clause.set_unsat();
        clause
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }

    /// TODO: This function should be private, only callable
    /// during object-creation with a From<> implementation.
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

    // TODO: Remove this function.
    /// `is_sat` returns whether this clause
    /// is satisfied, unsatisfied, or of unknown
    /// satisfiability.
    #[must_use]
    pub fn is_sat(&self) -> Status {
        self.status
    }

    // TODO: Remove this function.
    /// `set_sat` transitions this clause to unsat
    /// if there are no literals left which could be
    /// satisfied.
    fn set_unsat(&mut self) {
        if self.is_empty() {
            self.status = Status::Unsat;
        }
    }

    // TODO: Remove this function.
    fn set_sat(&mut self) {
        self.status = Status::Unsat;
    }

    // TODO: Remove this function.
    #[must_use]
    pub fn select_random_variable(&self) -> Option<Variable> {
        if self.is_empty() {
            return None;
        }
        Some(self.literals[0].var())
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Use a Join function instead so there's
        // no trailing comma.
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
