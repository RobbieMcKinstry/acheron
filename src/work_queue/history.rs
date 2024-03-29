use crate::core::{Condition, Formula, Status, TruthAssignment, Variable};
use crate::ops::Opcode;
use crate::work_queue::Summary;
use im::{vector, Vector};
use std::sync::Arc;

/// A (Solver) `History` contains a set of partial-solved
/// CNF formla. Each history contains a CNF equivalence in satisfiability
/// to the input CNF.
#[derive(Clone)]
pub struct History {
    previous: Option<Arc<History>>,
    summary: Summary,
    formula: Formula,
}

impl History {
    pub fn formula(&self) -> &Formula {
        &self.formula
    }

    pub fn child(&self, f: Formula, summary: Summary) -> Self {
        Self {
            previous: Some(Arc::new(self.clone())),
            formula: f,
            summary,
        }
    }

    #[must_use]
    pub fn is_sat(&self) -> bool {
        self.formula.is_sat()
    }

    pub fn status(&self) -> Status {
        self.formula.status()
    }

    /// `split_random` will randomly select a literal
    /// to split. This should eliminate that literal
    /// from the formula, unless the clause is already satisfied.
    fn split_random(&self) -> Vector<Self> {
        let random_var = self.formula.select_random_variable().unwrap();
        let (f1, f2) = self.split(random_var);
        vector![f1, f2]
    }

    // TODO:
    // Extract this into a Box<Operation> rather than a method!
    /// `split` will assign a truth value the variable
    /// identified by this literal.
    /// This will generate two new formulas, one with positive polarity
    /// and one with negative.
    fn split(&self, name: Variable) -> (Self, Self) {
        // First, construct a formula where each clause
        // has been modified by the given assignment.
        let pos = Condition::new(name, TruthAssignment::True);
        let neg = Condition::new(name, TruthAssignment::False);
        let formula_pos = self.formula.assign(pos);
        let formula_neg = self.formula.assign(neg);
        let op_pos = Opcode::Split(pos);
        let mut summary_pos = Summary::from(op_pos);
        summary_pos.add_change(pos);
        // TODO: No tracking info for now.
        // Must modify self to be an Arc.
        let resolvant_pos = Self {
            previous: None,
            summary: summary_pos,
            formula: formula_pos,
        };
        let op_neg = Opcode::Split(neg);
        let mut summary_neg = Summary::from(op_neg);
        summary_neg.add_change(neg);
        let resolvant_neg = Self {
            previous: None,
            summary: summary_neg,
            formula: formula_neg,
        };
        (resolvant_pos, resolvant_neg)
    }
}

/// Build a History from a formula, where opcode and previous are
/// zeroed out. Opcode is set to Nothing, and previous is set to None.
impl From<Formula> for History {
    fn from(formula: Formula) -> Self {
        Self {
            previous: None,
            summary: Summary::from(Opcode::Nothing),
            formula,
        }
    }
}
