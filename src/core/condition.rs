use super::Variable;

#[derive(Clone, Copy)]
pub enum TruthAssignment {
    True,
    False,
}

/// A `Condition` is a truth-assignment to a `Variable`.
/// It's logically distinct from a Literal, which is a
/// `Variable` found in a `Clause` with a given polarity.
#[derive(Clone, Copy)]
pub struct Condition {
    id: Variable,
    assignment: TruthAssignment,
}

impl Condition {
    #[must_use]
    pub fn new(variable: Variable, assignment: TruthAssignment) -> Self {
        Self {
            id: variable,
            assignment,
        }
    }
}
