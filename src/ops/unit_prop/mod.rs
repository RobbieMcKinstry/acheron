use crate::core::Literal;
use crate::ops::{ConstructorContext, OpContext, OpMaker, Opcode, Operator};
use crate::work_queue::{JobOutput, Summary, TerminationState};

pub struct UnitDetector;
impl OpMaker for UnitDetector {
    fn construct<'a>(&self, ctx: &ConstructorContext<'a>) -> Option<Box<dyn Operator>> {
        let units = ctx.formula().unit_literals();
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

impl Operator for UnitPropagator {
    fn apply<'a>(&self, ctx: OpContext<'a>) -> JobOutput {
        // • Create a new History with the updates applied
        //   as an Opcode.
        let opcode = Opcode::Unit(self.units.clone());
        let mut summary = Summary::from(opcode);
        // • Apply the conditions to the unit literals.
        let mut formula = ctx.formula().clone();
        for lit in self.units.iter() {
            let condition = lit.satisfying_condition();
            formula = formula.assign(condition);
            summary.add_change(condition);
        }
        let history = ctx.history().child(formula, summary);
        let state = TerminationState::Unfinished;
        JobOutput::new(history, state)
    }
}
