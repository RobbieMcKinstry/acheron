use crate::core::{Clause, Condition, Literal, Variable};
use crate::Status;
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
            .map(|clause| clause.assign(cond))
            .filter(|clause| clause.is_sat() != Status::Sat)
            .collect();
        Self { clauses }
    }

    pub fn len(&self) -> usize {
        self.clauses.len()
    }

    // TODO: Remove this function.
    #[must_use]
    pub fn is_sat(&self) -> Status {
        // A formula is satisfiable if all of its clauses are
        // satisfiable. If any clause is unsatisfiable, then
        // the entire formula is unsatisfiable.
        // Otherwise, the formula has unknown satisfiability.
        let mut satisfiability = Status::Unknown;
        let mut found_unknown = false;
        for clause in self.clauses.iter() {
            match clause.is_sat() {
                Status::Unsat => {
                    satisfiability = Status::Unsat;
                    return satisfiability;
                }
                Status::Unknown => found_unknown = true,

                Status::Sat => {
                    continue; /* Do nothing */
                }
            }
        }
        if !found_unknown {
            satisfiability = Status::Sat;
        }
        satisfiability
    }

    // TODO: Remove this function.
    #[must_use]
    pub fn select_random_variable(&self) -> Option<Variable> {
        // Scan through the list of clauses and find
        // one that is not yet sat. Then, select a random
        // literal in it, and condition that literal.
        for clause in self.clauses.iter() {
            if clause.is_sat() == Status::Sat {
                continue;
            }
            return clause.select_random_variable();
        }
        None
    }

    // TODO: Remove this function.
    /// If this `Formula` contains a unit clause, then `get_unit`
    /// will return the unit literal. Otherwise, it will return
    /// none.
    #[must_use]
    pub fn get_unit(&self) -> Option<Literal> {
        // Search the clauses for a unit.
        for clause in self.clauses.iter() {
            if clause.unit().is_some() {
                return clause.unit();
            }
        }
        None
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
