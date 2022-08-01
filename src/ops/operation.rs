use crate::core::Formula;
use crate::ops::op_context::OpContext;

pub trait Operation {
    // TODO: Its unclear how an operation should behave.
    // Should it return a new formula? What else should it
    // return? An `OpApplicationResult`, containing a new `Formula`
    // and a `History`? Who then processes the Result, applying
    // unit-propagation and pure literal elimination?
    fn apply<'a>(&self, ctx: OpContext<'a>);
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
