use crate::core::Literal;
use crate::ops::{ConstructorContext, OpContext, OpMaker, Operation};

pub struct UnitDetector;
impl OpMaker for UnitDetector {
    fn construct<'a>(&self, ctx: ConstructorContext<'a>) -> Option<Box<dyn Operation>> {
        let units = ctx.formula.unit_literals();
        if units.is_empty() {
            None
        } else {
            Some(Box::new(UnitPropagator::new(units)))
        }
    }
}

struct UnitPropagator {
    units: Vec<Literal>,
}

impl UnitPropagator {
    pub fn new(units: Vec<Literal>) -> Self {
        Self { units }
    }
}

impl Operation for UnitPropagator {
    fn apply<'a>(&self, _ctx: OpContext<'a>) {
        todo!("Apply the conditions for the unit literals.");
    }
}
