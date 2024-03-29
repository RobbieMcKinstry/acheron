use crate::ops::{ConstructorContext, OpContext, OpMaker, Operator};
use crate::work_queue::{JobOutput, TerminationState};

/// Builds an operator that handles
/// unsatisfied formulas.
pub struct UnsatOpMaker;

impl OpMaker for UnsatOpMaker {
    fn construct<'a>(&self, ctx: &ConstructorContext<'a>) -> Option<Box<dyn Operator>> {
        if ctx.formula().is_unsat() {
            Some(Box::new(UnsatOperator))
        } else {
            None
        }
    }
}

pub struct UnsatOperator;

impl Operator for UnsatOperator {
    fn apply<'a>(&self, ctx: OpContext<'a>) -> JobOutput {
        // Since we know this formula is UNSAT, we can
        // reuse the formula in the context's history.
        // TODO: Right now, we don't record the conflict set.
        // In the future, we need to walk
        // the history to collect all of the Conditions applied
        // to get this formula.
        let state = TerminationState::Unsat(Vec::new());
        let history = ctx.history().clone();
        JobOutput::new(history, state)
    }
}
