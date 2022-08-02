use crate::core::Clause;

/// After a `Clause` is conditioned, the values
/// assigned during the conditioning process may
/// result in a new clause, but may or may not
/// have satisfied that clause.
pub enum ClauseAssignment {
    Satisfied,
    Other(Clause),
}

impl ClauseAssignment {
    pub fn is_sat(&self) -> bool {
        matches!(self, Self::Satisfied)
    }
}
