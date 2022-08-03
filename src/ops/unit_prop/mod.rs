struct UnitPropagationMaker;
impl OpMaker for UnitPropagationMaker {
    type OpType = UnitPropagation;

    fn construct<'a>(ctx: ConstructorContext<'a>) -> Option<Box<Self::OpType>> {
        todo!("Determine if there are any unit literals.");
    }
}

struct UnitPropagation {
    // TODO: Store any Unit Literals we observed during construction.
    units: Vec<Literal>,
}

impl Operation for UnitPropagation {
    fn apply<'a>(&self, ctx: OpContext<'a>) {
        todo!("Apply the conditions for the unit literals.");
    }
}
