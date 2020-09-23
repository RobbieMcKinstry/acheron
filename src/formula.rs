use crate::{Clause, Literal, Status, Variable};
use im::Vector;

#[derive(Clone)]
pub struct Formula {
    clauses: Vector<Clause>,
}

impl Formula {
    /// `assign` will construct a new formula with the given
    /// truth-assignment.
    pub fn assign(&self, lit: &Literal) -> Self {
        // Iterate through the clauses, applying this assignment.
        // If a clause is satisfied, then do not include it
        // in the vector.
        let clauses = self
            .clauses
            .iter()
            .map(|clause| clause.assign(lit))
            .filter(|clause| clause.is_sat() != Status::Sat)
            .collect();
        Self { clauses }
    }

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
    pub fn select_random_variable(&self) -> Option<Variable> {
        // Scan through the list of clauses and find
        // one that is not yet sat. Then, select a random
        // literal in it, and resolve that literal.
        for clause in self.clauses.iter() {
            if clause.is_sat() == Status::Sat {
                continue;
            }
            return clause.select_random_variable();
        }
        None
    }
}

/// Build a Frame from a formula, where opcode and previous are
/// zeroed out. Opcode is set to Nothing, and previous is set to None.
impl From<Vector<Clause>> for Formula {
    fn from(clauses: Vector<Clause>) -> Self {
        Self { clauses }
    }
}
