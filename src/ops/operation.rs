use crate::ops::op_context::OpContext;

pub trait Operation {
    // TODO: Its unclear how an operation should behave.
    // Should it return a new formula? What else should it
    // return? An `OpApplicationResult`, containing a new `Formula`
    // and a `History`? Who then processes the Result, applying
    // unit-propagation and pure literal elimination?
    fn apply<'a>(&self, ctx: OpContext<'a>);
}

// Apply the condition to the formula.
// Then, report a new history, generating
// the OpCode for `ConditionApplication`.
// The caller of `apply` is expected to perform
// unit propagation and pure literal elimination
// after calling `apply`.

#[cfg(test)]
mod tests {
    use super::Operation;
    use static_assertions::assert_obj_safe;

    #[test]
    fn operation_is_obj_safe() {
        assert_obj_safe!(Operation);
    }
}
