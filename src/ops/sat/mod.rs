use crate::ops::{ConstructorContext, OpContext, OpMaker, Operation};
use crate::work_queue::{JobOutput, TerminationState};

/// Builds an operator that handles
/// satisfied formulas.
pub struct SatOpMaker;

impl OpMaker for SatOpMaker {
    fn construct<'a>(&self, ctx: &ConstructorContext<'a>) -> Option<Box<dyn Operation>> {
        // Check if the formula is SAT.
        if ctx.formula().is_sat() {
            Some(Box::new(SatOperator))
        } else {
            None
        }
    }
}

pub struct SatOperator;

impl Operation for SatOperator {
    fn apply<'a>(&self, ctx: OpContext<'a>) -> JobOutput {
        // Since we know this formula is SAT, we can
        // reuse the formula provided to us in the context.
        // TODO: Right now, we don't record the satisfying
        // conditions. In the future, we need to walk
        // the history to collect all of the Conditions applied
        // to get this formula.
        let state = TerminationState::Sat(Vec::new());
        let history = ctx.history().clone();
        JobOutput::new(history, state)
    }
}
