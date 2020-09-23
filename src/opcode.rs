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
}
