use crate::core::Literal;
use crate::ops::{ConstructorContext, OpContext, OpMaker, Opcode, Operation};
use crate::work_queue::{Job, JobOutput, Summary, TerminationState};

pub struct UnitDetector;
impl OpMaker for UnitDetector {
    fn construct<'a>(&self, ctx: &ConstructorContext<'a>) -> Option<Box<dyn Operation>> {
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
    fn apply<'a>(&self, ctx: OpContext<'a>) -> JobOutput {
        // • Create a new History with the updates applied
        //   as an Opcode.
        let opcode = Opcode::Unit(self.units.clone());
        let mut summary = Summary::from(opcode);
        // • Apply the conditions to the unit literals.
        let mut formula = ctx.formula.clone();
        for lit in self.units.iter() {
            let condition = lit.satisfying_condition();
            formula = formula.assign(condition);
            summary.add_change(condition);
        }
        // TODO: Fix triple-accounting:
        // Formula is owned by the history, the next job, and
        // the JobOutput.
        let history = ctx.history.child(formula.clone(), summary);
        let state = TerminationState::Unfinished;

        JobOutput::new(formula, state, history)
    }
}
