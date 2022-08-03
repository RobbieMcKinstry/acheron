pub use clause_assignment::ClauseAssignment;

use crate::core::condition::{Condition, ConditionEffect};
use crate::core::{Literal, Status, Variable};
use im::Vector;
use std::fmt;

mod clause_assignment;

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
    pub fn assign(&self, cond: Condition) -> ClauseAssignment {
        let mut clause = Self::new();
        for literal in self.literals.iter() {
            match literal.apply_condition(cond) {
                ConditionEffect::Sat => {
                    // Satisfied!
                    return ClauseAssignment::Satisfied;
                }
                ConditionEffect::NoImpact => {
                    // Literal's variable does not appear in condition.
                    // Keep literal in clause.
                    clause.literals.push_back(*literal)
                }
                ConditionEffect::Unsat => {
                    // Matches variable but not sign!
                    // Remove this literal from the clause.
                }
            }
        }
        ClauseAssignment::Other(clause)
    }

    pub fn is_unsat(&self) -> bool {
        self.is_empty()
    }

    fn is_empty(&self) -> bool {
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
    pub fn is_unit(&self) -> bool {
        self.unit().is_some()
    }

    /// `unit` returns the unit literal in this clause if this
    /// clause contains the a unit literal. Otherwise it returns
    // None.
    #[must_use]
    pub fn unit(&self) -> Option<Literal> {
        if self.literals.len() == 1 {
            Some(self.literals[0])
        } else {
            None
        }
    }

    // TODO: Remove this function.
    #[must_use]
    pub fn select_random_variable(&self) -> Option<Variable> {
        if self.is_empty() {
            return None;
        }
        Some(self.literals[0].var())
    }

    pub fn iter<'a>(&'a self) -> im::vector::Iter<'a, Literal> {
        self.literals.iter()
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
