use super::Variable;

pub use assignment::TruthAssignment;
pub use effect::ConditionEffect;

/// A `Condition` is a truth-assignment to a `Variable`.
/// It's logically distinct from a Literal, which is a
/// `Variable` found in a `Clause` with a given polarity.
#[derive(Clone, Copy)]
pub struct Condition {
    var: Variable,
    assignment: TruthAssignment,
}

impl Condition {
    #[must_use]
    pub fn new(variable: Variable, assignment: TruthAssignment) -> Self {
        Self {
            var: variable,
            assignment,
        }
    }

    pub fn var(self) -> Variable {
        self.var
    }

    pub fn assignment(self) -> TruthAssignment {
        self.assignment
    }
}

mod assignment;
mod effect;
