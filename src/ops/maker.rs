use crate::core::{Condition, Formula, Literal};
use crate::ops::constructor_context::ConstructorContext;
use crate::ops::op_context::OpContext;
use crate::ops::Operation;
use crate::work_queue::WorkQueue;
use crate::History;

/// An OpMaker is anything that can construct an Operation.
trait OpMaker<'a> {
    type OpType: Operation;

    fn construct(ctx: ConstructorContext<'a>) -> Option<Self::OpType>;
}

struct UnitPropagationMaker();
impl<'a> OpMaker<'a> for UnitPropagationMaker {
    type OpType = UnitPropagation;

    fn construct(ctx: ConstructorContext<'a>) -> Option<Self::OpType> {
        todo!("Determine if there are any unit literals.");
    }
}

struct UnitPropagation {
    // TODO: Store any Unit Literals we observed during construction.
    units: Vec<Literal>,
}

impl Operation for UnitPropagation {
    fn apply<'a>(&self, ctx: OpContext<'a>) {
        todo!("Apply the conditions for the unit literals.");
    }
}

// TODO: Switch the solver over to using the WorkQueue instead
// of the History type directly.

/// A `ConditionApplication` represents a condition that
/// is expected to be applied to a Formula, but hasn't been
/// applied yet. We create a ConditionApplication when we
/// perform splitting: one ConditionApplication for the `true`
/// case and another for the `false` case. These objects are
/// then enqueued in the `WorkQueue` for later processing.
struct ConditionApplication {
    pending_condition: Condition,
}

impl Operation for ConditionApplication {
    fn apply<'a>(&self, ctx: OpContext<'a>) {
        // Apply the condition to the formula.
        // Then, report a new history, generating
        // the OpCode for `ConditionApplication`.
        // The caller of `apply` is expected to perform
        // unit propagation and pure literal elimination
        // after calling `apply`.
        todo!("Not implemented");
    }
}
