use crate::ops::{ConstructorContext, OpContext, OpMaker, Operation};

/// Builds an operator that handles
/// satisfied formulas.
pub struct SatOpMaker();

impl<'a> OpMaker<'a> for SatOpMaker {
    type OpType = SatOperator;

    fn construct(ctx: ConstructorContext<'a>) -> Option<Box<Self::OpType>> {
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
    fn apply<'a>(&self, ctx: OpContext<'a>) {
        todo!("Return SAT");
    }
}
