use crate::core::Condition;
use crate::ops::{OpContext, Opcode, Operator};
use crate::work_queue::{JobOutput, Summary, TerminationState};

pub fn new_condition_application(cond: Condition) -> Box<dyn Operator> {
    Box::new(ConditionApplication::new(cond))
}

/// A `ConditionApplication` represents a condition that
/// is expected to be applied to a Formula, but hasn't been
/// applied yet. We create a ConditionApplication when we
/// perform splitting: one ConditionApplication for the `true`
/// case and another for the `false` case. These objects are
/// then enqueued in the `WorkQueue` for later processing.
struct ConditionApplication {
    pending_condition: Condition,
}

impl ConditionApplication {
    pub fn new(cond: Condition) -> Self {
        Self {
            pending_condition: cond,
        }
    }
}

impl Operator for ConditionApplication {
    fn apply<'a>(&self, ctx: OpContext<'a>) -> JobOutput {
        let cond = self.pending_condition;
        let opcode = Opcode::Split(cond);
        let mut summary = Summary::from(opcode);
        summary.add_change(cond);
        let formula = ctx.formula().assign(cond);
        let history = ctx.history().child(formula, summary);
        JobOutput::new(history, TerminationState::Unfinished)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Clause, Formula, TruthAssignment, Variable};
    use crate::work_queue::History;
    use im::Vector;

    fn make_formula(clauses: Vec<Vec<&str>>) -> Formula {
        let mut vec = Vector::new();
        for lits in clauses {
            let mut clause = Clause::new();
            for lit in lits {
                clause = clause.add_literal(lit.to_owned());
            }
            vec.push_back(clause);
        }
        Formula::from(vec)
    }

    #[test]
    fn apply_satisfies_matching_clause() {
        // Formula: (1 ∨ 2). Assign variable 1 = true.
        // Clause contains literal +1, so the clause is satisfied and removed.
        let formula = make_formula(vec![vec!["1", "2"]]);
        let history = History::from(formula);
        let cond = Condition::new(Variable::from(1u64), TruthAssignment::True);
        let op = ConditionApplication::new(cond);
        let ctx = OpContext::new(&history);
        let output = op.apply(ctx);
        assert!(output.formula().is_sat());
    }

    #[test]
    fn apply_removes_falsified_literal() {
        // Formula: (1 ∨ 2). Assign variable 1 = false.
        // Literal +1 is falsified and removed, leaving clause (2).
        let formula = make_formula(vec![vec!["1", "2"]]);
        let history = History::from(formula);
        let cond = Condition::new(Variable::from(1u64), TruthAssignment::False);
        let op = ConditionApplication::new(cond);
        let ctx = OpContext::new(&history);
        let output = op.apply(ctx);
        // Formula should still have one clause with one literal.
        assert!(!output.formula().is_sat());
        assert!(!output.formula().is_unsat());
        let remaining: Vec<_> = output.formula().iter().collect();
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].iter().count(), 1);
    }

    #[test]
    fn apply_produces_empty_clause_on_contradiction() {
        // Formula: (-1). Assign variable 1 = true.
        // Literal -1 is falsified → clause becomes empty → UNSAT.
        let formula = make_formula(vec![vec!["-1"]]);
        let history = History::from(formula);
        let cond = Condition::new(Variable::from(1u64), TruthAssignment::True);
        let op = ConditionApplication::new(cond);
        let ctx = OpContext::new(&history);
        let output = op.apply(ctx);
        assert!(output.formula().is_unsat());
    }

    #[test]
    fn apply_returns_unfinished() {
        let formula = make_formula(vec![vec!["1", "2"]]);
        let history = History::from(formula);
        let cond = Condition::new(Variable::from(1u64), TruthAssignment::True);
        let op = ConditionApplication::new(cond);
        let ctx = OpContext::new(&history);
        let output = op.apply(ctx);
        assert!(matches!(output.state(), TerminationState::Unfinished));
    }

    #[test]
    fn apply_leaves_unrelated_clauses_intact() {
        // Formula: (1 ∨ 2) ∧ (3 ∨ 4). Assign variable 1 = true.
        // First clause is satisfied. Second clause is untouched.
        let formula = make_formula(vec![vec!["1", "2"], vec!["3", "4"]]);
        let history = History::from(formula);
        let cond = Condition::new(Variable::from(1u64), TruthAssignment::True);
        let op = ConditionApplication::new(cond);
        let ctx = OpContext::new(&history);
        let output = op.apply(ctx);
        let remaining: Vec<_> = output.formula().iter().collect();
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].iter().count(), 2);
    }
}
