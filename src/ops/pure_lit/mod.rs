use crate::ops::{ConstructorContext, OpMaker, Operator};

/// `PureLiteralMaker` detects pure literals and reports
/// them. A literal is pure if it only occurs with a single polarity
/// in all formulas.
pub struct PureLiteralDetector;

impl OpMaker for PureLiteralDetector {
    fn construct<'a>(&self, _ctx: &ConstructorContext<'a>) -> Option<Box<dyn Operator>> {
        todo!("Impl Pure Literal Detection is a Map for each Variable => Polarity");
    }
}
