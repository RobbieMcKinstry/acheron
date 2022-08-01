use crate::Literal;

/// An `Opcode` represents an operation on a formula.
/// Each operation will yield one or more equivalent formulas.
/// As more solving techniques are added to this solver,
/// the number of opcodes will grow.
#[derive(Clone)]
pub enum Opcode {
    /// `Nothing` implies no operation is performed.
    /// Used by the input formula.
    Nothing,
    Resolution(Literal),
    /// Unit propagation (unit resolution) occurs when a clause has only
    /// one literal. That literal must be satisfied
    /// for the formula to be satisfied.
    Unit(Literal),
    /// A pure literal is one that always occurs with
    /// the same polarity in a given formula.
    /// You can freely satisfied all clauses that contain a pure literal by
    /// conditioning the pure literal.
    // TODO Keep a table of all variables and their polarities.
    // Once a clause is satisfied, decrement the counts for each
    // variable.
    // If a variable ever reaches zero, then it is pure.
    // Note: This is effectively a Scope table from Compilers.
    Pure(Literal),
}
