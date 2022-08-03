use crate::core::Condition;
use crate::ops::{OpContext, Operation};
use crate::work_queue::JobOutput;

pub fn new_condition_application(cond: Condition) -> Box<dyn Operation> {
    Box::new(ConditionApplication::new(cond))
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

impl ConditionApplication {
    pub fn new(cond: Condition) -> Self {
        Self {
            pending_condition: cond,
        }
    }
}

impl Operation for ConditionApplication {
    fn apply<'a>(&self, _ctx: OpContext<'a>) -> JobOutput {
        todo!("Not implemented");
    }
}
