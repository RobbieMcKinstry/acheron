use crate::ops::{ConstructorContext, OpContext, OpMaker, Operation};

/// Builds an operator that handles
/// unsatisfied formulas.
pub struct UnsatOpMaker();

impl<'a> OpMaker<'a> for UnsatOpMaker {
    type OpType = UnsatOperator;

    fn construct(ctx: ConstructorContext<'a>) -> Option<Box<Self::OpType>> {
        todo!();
        /* Blocked on adding Iteration to Formulas and Clauses
        for clause in ctx.formula.iter() {
            if clause.len() == 0 {
                return Some(Box::new(UnsatOperator));
            }
        }
        None
        */
    }
}

pub struct UnsatOperator;

impl Operation for UnsatOperator {
    fn apply<'a>(&self, ctx: OpContext<'a>) {
        todo!("Return UNSAT");
    }
}
