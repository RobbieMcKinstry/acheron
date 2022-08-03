use crate::ops::prelude::*;

// 1. Check if SAT.
// 2. Check if UNSAT.
// 3. Check if unit literals exist.
// 4. Check if pure literals exist.
// 5. Perform Splitting.

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
        let decision_table = vec![
            Box::new(SatMaker) as Box<dyn OpMaker>,
            Box::new(UnsatMaker),
            Box::new(UnitDetector),
        ];
        Self { decision_table }
    }
}
