use crate::ops::op_context::OpContext;
use crate::work_queue::JobOutput;

pub trait Operator {
    fn apply(&self, ctx: OpContext<'_>) -> JobOutput;
}

// Apply the condition to the formula.
// Then, report a new history, generating
// the OpCode for `ConditionApplication`.
// The caller of `apply` is expected to perform
// unit propagation and pure literal elimination
// after calling `apply`.

#[cfg(test)]
mod tests {
    use super::Operator;
    use static_assertions::assert_obj_safe;

    #[test]
    fn operation_is_obj_safe() {
        assert_obj_safe!(Operator);
    }
}
