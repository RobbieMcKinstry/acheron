use crate::ops::prelude::*;
use crate::ops::ConstructorContext;
use crate::work_queue::History;

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

    pub fn select(&self, hist: &History) -> Vec<Box<dyn Operator>> {
        let ctx = ConstructorContext { history: hist };
        let op = self
            .decision_table
            .iter()
            .find_map(|elem| elem.construct(&ctx))
            .expect("One of the items in the list must return something");
        vec![op]
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
