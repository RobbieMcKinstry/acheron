use crate::ops::prelude::*;

/// The DecisionEngine determines which operation
/// to apply to a Formula next.
pub struct DecisionEngine {
    // An ordered list of deciders.
    // The first item in the list that
    // can be applied to a given job, will be.
    decision_table: Vec<Box<dyn OpMaker>>,
}

impl DecisionEngine {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for DecisionEngine {
    fn default() -> Self {
        let decision_table = vec![Box::new(SatMaker) as Box<dyn OpMaker>, Box::new(UnsatMaker)];
        Self { decision_table }
    }
}

// 1. Check if SAT.
// 2. Check if UNSAT.
// 3. Check if unit literals exist.
// 4. Check if pure literals exist.
// 5. Perform Splitting.

/*
/// sat is true if formula.is_empty()
// Return the list of
fn is_sat() -> Vec<Literal> {
    todo!()
}

/// Unsat is true if there exists clause | clause.is_empty()
/// Return the Conflict Set.
fn is_unsat() -> Vec<Literal> {
    todo!()
}

/// Empty => no unit literal exists
/// Non-empty => literals are all unit.
fn unit_literal() -> Vec<Literal> {
    todo!()
}

/// Empty => no pure literal exists
/// Non-empty => literals are all pure
fn pure_literal() -> Vec<Literal> {
    todo!()
}
*/
