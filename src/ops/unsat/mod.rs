use crate::ops::{ConstructorContext, OpContext, OpMaker, Operation};

/// Builds an operator that handles
/// unsatisfied formulas.
pub struct UnsatOpMaker;

impl OpMaker for UnsatOpMaker {
    fn construct<'a>(&self, ctx: ConstructorContext<'a>) -> Option<Box<dyn Operation>> {
        if ctx.formula.is_unsat() {
            Some(Box::new(UnsatOperator))
        } else {
            None
        }
    }
}

pub struct UnsatOperator;

impl Operation for UnsatOperator {
    fn apply<'a>(&self, _ctx: OpContext<'a>) {
        todo!("Return UNSAT");
    }
}
