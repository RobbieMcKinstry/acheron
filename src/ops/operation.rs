use crate::core::{Condition, Formula};

pub trait Operation {
    // TODO: Its unclear how an operation should behave.
    // Should it return a new formula? What else should it
    // return? An `OpApplicationResult`, containing a new `Formula`
    // and a `History`? Who then processes the Result, applying
    // unit-propagation and pure literal elimination?
    fn apply(&self, formula: &Formula);
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
    fn apply(&self, formula: &Formula) {
        // Apply the condition to the formula.
        // Then, report a new history, generating
        // the OpCode for `ConditionApplication`.
        // The caller of `apply` is expected to perform
        // unit propagation and pure literal elimination
        // after calling `apply`.
        todo!("Not implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::Operation;
    use static_assertions::assert_obj_safe;

    #[test]
    fn operation_is_obj_safe() {
        assert_obj_safe!(Operation);
    }
}
