use crate::core::clause::ClauseAssignment;
use crate::core::{Clause, Condition, Literal, Status, Variable};
use im::Vector;

#[derive(Clone)]
pub struct Formula {
    clauses: Vector<Clause>,
}

impl Formula {
    /// `assign` will construct a new formula with the given
    /// truth-assignment.
    #[must_use]
    pub fn assign(&self, cond: Condition) -> Self {
        // Iterate through the clauses, applying this assignment.
        // If a clause is satisfied, then do not include it
        // in the vector.
        let clauses = self
            .clauses
            .iter()
            // Remove clauses that have been satisfied.
            .filter_map(|clause| match clause.assign(cond) {
                ClauseAssignment::Satisfied => None,
                ClauseAssignment::Other(c) => Some(c),
            })
            .collect();
        Self { clauses }
    }

    /// A `Formula` is satisfied if there are no unsatisfied clauses.
    pub fn is_sat(&self) -> bool {
        self.clauses.is_empty()
    }

    /// A `Formula` is unsatisfied if it has an empty clause.
    /// This represents the case where one clause has all of its
    /// literals assigned, and none of them have been satisfied
    /// by assignment. Thus, there is no opportunity for the clause
    /// to be satisfied.
    pub fn is_unsat(&self) -> bool {
        self.clauses.iter().any(Clause::is_unsat)
    }

    pub fn status(&self) -> Status {
        if self.is_sat() {
            Status::Sat
        } else if self.is_unsat() {
            Status::Unsat
        } else {
            Status::Unknown
        }
    }

    /// `unit_literals` returns the list of unit literals
    /// found within this formula. If there are no literals
    /// in the forumua, an empty vector is returned.
    pub fn unit_literals(&self) -> Vec<Literal> {
        self.clauses.iter().filter_map(Clause::unit).collect()
    }

    // TODO: Remove this function.
    #[must_use]
    pub fn select_random_variable(&self) -> Option<Variable> {
        // Scan through the list of clauses and find
        // one that is not yet sat. Then, select a random
        // literal in it, and condition that literal.
        self.clauses
            .front()
            .map(|clause| clause.select_random_variable())
            .flatten()
    }

    pub fn iter<'a>(&'a self) -> im::vector::Iter<'a, Clause> {
        self.clauses.iter()
    }
}

/// Build a Formula from a list of clauses
impl From<Vector<Clause>> for Formula {
    fn from(clauses: Vector<Clause>) -> Self {
        Self { clauses }
    }
}
