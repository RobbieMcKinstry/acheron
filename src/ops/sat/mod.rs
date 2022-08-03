use crate::ops::{ConstructorContext, OpContext, OpMaker, Operation};

/// Builds an operator that handles
/// satisfied formulas.
pub struct SatOpMaker;

impl OpMaker for SatOpMaker {
    fn construct<'a>(&self, ctx: ConstructorContext<'a>) -> Option<Box<dyn Operation>> {
        // Check if the formula is SAT.
        if ctx.formula.is_sat() {
            Some(Box::new(SatOperator))
        } else {
            None
        }
    }
}

pub struct SatOperator;

impl Operation for SatOperator {
    fn apply<'a>(&self, _ctx: OpContext<'a>) {
        todo!("Return SAT");
    }
}
