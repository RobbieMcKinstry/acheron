use crate::ops::constructor_context::ConstructorContext;
use crate::ops::Operator;

/// An OpMaker is anything that can construct an Operation.
/// Returning `None` means this operation doesn't apply to the
/// current formula. Returning `Some` yields one or more operators
/// to enqueue (e.g. splitting produces two: one per branch).
pub trait OpMaker {
    fn construct<'a>(&self, ctx: &ConstructorContext<'a>) -> Option<Vec<Box<dyn Operator>>>;
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
