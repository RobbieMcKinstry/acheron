use crate::core::Condition;
use crate::ops::constructor_context::ConstructorContext;
use crate::ops::op_context::OpContext;
use crate::ops::Operation;

/// An OpMaker is anything that can construct an Operation.
pub trait OpMaker {
    fn construct<'a>(&self, ctx: ConstructorContext<'a>) -> Option<Box<dyn Operation>>;
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
    fn apply<'a>(&self, _ctx: OpContext<'a>) {
        todo!("Not implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::OpMaker;
    use static_assertions::assert_obj_safe;

    #[test]
    fn operation_is_obj_safe() {
        assert_obj_safe!(OpMaker);
    }
}
