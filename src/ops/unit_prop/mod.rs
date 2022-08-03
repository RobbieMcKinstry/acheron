use crate::ops::{ConstructorContext, OpContext, OpMaker, Operation};

struct UnitMaker;
impl OpMaker for UnitMaker {
    fn construct<'a>(&self, ctx: ConstructorContext<'a>) -> Option<Box<dyn Operation>> {
        let units = ctx.formula.unit_literals();
        if units.is_empty() {
            None
        } else {
            Some(Box::new(UnitPropgation::new(units)))
        }
    }
}

struct UnitPropagation {
    units: Vec<Literal>,
}

impl UnitPropagation {
    pub fn new(units: Vec<Literal>) -> Self {
        Self { units }
    }
}

impl Operation for UnitPropagation {
    fn apply<'a>(&self, ctx: OpContext<'a>) {
        todo!("Apply the conditions for the unit literals.");
    }
}
