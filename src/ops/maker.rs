use crate::ops::constructor_context::ConstructorContext;
use crate::ops::Operation;

/// An OpMaker is anything that can construct an Operation.
pub trait OpMaker {
    fn construct<'a>(&self, ctx: &ConstructorContext<'a>) -> Option<Box<dyn Operation>>;
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
