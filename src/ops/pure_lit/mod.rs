use crate::core::Literal;
use crate::ops::{ConstructorContext, OpContext, OpMaker, Opcode, Operator};
use crate::work_queue::{JobOutput, Summary, TerminationState};

/// `PureLiteralDetector` detects pure literals and reports
/// them. A literal is pure if it only occurs with a single polarity
/// across all clauses in the formula.
pub struct PureLiteralDetector;

impl OpMaker for PureLiteralDetector {
    fn construct<'a>(&self, ctx: &ConstructorContext<'a>) -> Option<Vec<Box<dyn Operator>>> {
        let pures = ctx.formula().pure_literals();
        if pures.is_empty() {
            None
        } else {
            Some(vec![Box::new(PureLiteralEliminator::new(pures))])
        }
    }
}

struct PureLiteralEliminator {
    literals: Vec<Literal>,
}

impl PureLiteralEliminator {
    pub fn new(literals: Vec<Literal>) -> Self {
        Self { literals }
    }
}

impl Operator for PureLiteralEliminator {
    fn apply<'a>(&self, ctx: OpContext<'a>) -> JobOutput {
        let opcode = Opcode::Pure(self.literals.clone());
        let mut summary = Summary::from(opcode);
        let mut formula = ctx.formula().clone();
        for lit in self.literals.iter() {
            let condition = lit.satisfying_condition();
            formula = formula.assign(condition);
            summary.add_change(condition);
        }
        let history = ctx.history().child(formula, summary);
        JobOutput::new(history, TerminationState::Unfinished)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Clause, Sign, Variable};
    use crate::work_queue::History;
    use im::Vector;
    use pretty_assertions::assert_eq;

    use crate::core::Formula;

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

    // --- PureLiteralDetector (OpMaker) tests ---

    #[test]
    fn detector_returns_none_when_no_pure_literals() {
        // Every variable appears with both polarities
        let formula = make_formula(vec![vec!["1", "-2"], vec!["-1", "2"]]);
        let history = History::from(formula);
        let ctx = ConstructorContext { history: &history };
        let result = PureLiteralDetector.construct(&ctx);
        assert!(result.is_none());
    }

    #[test]
    fn detector_returns_some_when_pure_literal_exists() {
        // x1 is pure (positive only), x2 appears with both polarities
        let formula = make_formula(vec![vec!["1", "2"], vec!["1", "-2"]]);
        let history = History::from(formula);
        let ctx = ConstructorContext { history: &history };
        let result = PureLiteralDetector.construct(&ctx);
        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn detector_returns_none_for_empty_formula() {
        let formula = Formula::from(Vector::new());
        let history = History::from(formula);
        let ctx = ConstructorContext { history: &history };
        let result = PureLiteralDetector.construct(&ctx);
        assert!(result.is_none());
    }

    // --- PureLiteralEliminator (Operator) tests ---

    #[test]
    fn eliminator_removes_clauses_with_pure_literal() {
        // Formula: (x1 ∨ x2) ∧ (x1 ∨ ¬x2)
        // x1 is pure positive. Satisfying x1=true removes both clauses.
        let formula = make_formula(vec![vec!["1", "2"], vec!["1", "-2"]]);
        let pure_lit = Literal::new(Variable::from(1), Sign::Positive);
        let eliminator = PureLiteralEliminator::new(vec![pure_lit]);

        let history = History::from(formula);
        let ctx = OpContext::new(&history);
        let output = eliminator.apply(ctx);
        assert!(output.formula().is_sat());
    }

    #[test]
    fn eliminator_preserves_unrelated_clauses() {
        // Formula: (x1 ∨ x2) ∧ (x3 ∨ ¬x3)
        // But x3 appears both ways, so only x1 and x2 could be pure.
        // Let's say x1 is pure positive.
        // After eliminating x1=true: clause (x1 ∨ x2) is satisfied,
        // but (x3 ∨ ¬x3) is untouched.
        let formula = make_formula(vec![vec!["1", "2"], vec!["3", "-3"]]);
        let pure_lit = Literal::new(Variable::from(1), Sign::Positive);
        let eliminator = PureLiteralEliminator::new(vec![pure_lit]);

        let history = History::from(formula);
        let ctx = OpContext::new(&history);
        let output = eliminator.apply(ctx);
        // (x1 ∨ x2) removed, (x3 ∨ ¬x3) remains
        let remaining: Vec<_> = output.formula().iter().collect();
        assert_eq!(remaining.len(), 1);
    }

    #[test]
    fn eliminator_returns_unfinished() {
        let formula = make_formula(vec![vec!["1", "2"], vec!["3", "-3"]]);
        let pure_lit = Literal::new(Variable::from(1), Sign::Positive);
        let eliminator = PureLiteralEliminator::new(vec![pure_lit]);

        let history = History::from(formula);
        let ctx = OpContext::new(&history);
        let output = eliminator.apply(ctx);
        assert!(matches!(output.state(), &TerminationState::Unfinished));
    }

    #[test]
    fn eliminator_handles_multiple_pure_literals() {
        // Formula: (x1 ∨ ¬x2) ∧ (x1 ∨ ¬x2 ∨ x3)
        // x1 is pure positive, x2 is pure negative, x3 is pure positive
        // Satisfying all of them should eliminate all clauses.
        let formula = make_formula(vec![vec!["1", "-2"], vec!["1", "-2", "3"]]);
        let pures = vec![
            Literal::new(Variable::from(1), Sign::Positive),
            Literal::new(Variable::from(2), Sign::Negative),
            Literal::new(Variable::from(3), Sign::Positive),
        ];
        let eliminator = PureLiteralEliminator::new(pures);

        let history = History::from(formula);
        let ctx = OpContext::new(&history);
        let output = eliminator.apply(ctx);
        assert!(output.formula().is_sat());
    }

    #[test]
    fn eliminator_with_negative_pure_literal() {
        // Formula: (¬x1 ∨ x2) ∧ (¬x1 ∨ ¬x2)
        // x1 is pure negative. Satisfying ¬x1 means x1=false,
        // which satisfies the ¬x1 literal in both clauses.
        let formula = make_formula(vec![vec!["-1", "2"], vec!["-1", "-2"]]);
        let pure_lit = Literal::new(Variable::from(1), Sign::Negative);
        let eliminator = PureLiteralEliminator::new(vec![pure_lit]);

        let history = History::from(formula);
        let ctx = OpContext::new(&history);
        let output = eliminator.apply(ctx);
        assert!(output.formula().is_sat());
    }
}
