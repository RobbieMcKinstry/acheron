use crate::{Formula, Literal, Opcode, Sign, Status, Variable};
use im::{vector, Vector};
use std::sync::Arc;

/// A (Solver) `Frame` contains a set of partial-solved
/// CNF formla. Each frame contains a CNF equivalence in satisfiability
/// to the input CNF.
#[derive(Clone)]
pub struct Frame {
    previous: Option<Arc<Frame>>,
    op: Opcode,
    formula: Formula,
}

impl Frame {
    /// `apply` will apply an operation to this
    /// formula, resulting in one or more new frames.
    pub fn apply(&self) -> Vector<Self> {
        // If we have a unit clause, let's propagate it.
        if let Some(literal) = self.formula.get_unit() {
            let resolvant = self.resolve_unit(literal);
            vector![resolvant]
        } else {
            self.resolve_random()
        }
    }

    pub fn is_sat(&self) -> Status {
        self.formula.is_sat()
    }

    fn resolve_unit(&self, literal: Literal) -> Self {
        let resolved_formula = self.formula.assign(&literal);
        Self {
            previous: None,
            op: Opcode::Unit(literal),
            formula: resolved_formula,
        }
    }

    /// `resolve_random` will randomly select a literal
    /// to resolve. This should eliminate that literal
    /// from the formula, unless the clause is already satisfied.
    fn resolve_random(&self) -> Vector<Self> {
        let random_var = self.formula.select_random_variable().unwrap();
        let (f1, f2) = self.resolve(random_var);
        vector![f1, f2]
    }

    /// `resolve` will assign a truth value the variable
    /// identified by this literal.
    /// This will generate two new formulas, o
    fn resolve(&self, name: Variable) -> (Self, Self) {
        // First, construct a formula where each clause
        // has been modified by the given assignment.
        let lit_pos = Literal::new(name, Sign::Positive);
        let lit_neg = Literal::new(name, Sign::Negative);
        let formula_pos = self.formula.assign(&lit_pos);
        let formula_neg = self.formula.assign(&lit_neg);
        // TODO: No tracking info for now.
        // Must modify self to be an Arc.
        let resolvant_pos = Self {
            previous: None,
            op: Opcode::Resolution(lit_pos),
            formula: formula_pos,
        };
        let resolvant_neg = Self {
            previous: None,
            op: Opcode::Resolution(lit_neg),
            formula: formula_neg,
        };
        (resolvant_pos, resolvant_neg)
    }
}

/// Build a Frame from a formula, where opcode and previous are
/// zeroed out. Opcode is set to Nothing, and previous is set to None.
impl From<Formula> for Frame {
    fn from(formula: Formula) -> Self {
        Self {
            previous: None,
            op: Opcode::Nothing,
            formula,
        }
    }
}
