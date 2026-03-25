use crate::core::{Condition, TruthAssignment};
use crate::ops::cond_app::new_condition_application;
use crate::ops::{ConstructorContext, OpMaker, Operator};

/// `SplitMaker` is the fallback operation in the decision table.
/// When no other operation applies (SAT, UNSAT, unit propagation),
/// it picks a variable from the formula and branches into two
/// sub-problems: one assigning the variable true, another false.
pub struct SplitMaker;

impl OpMaker for SplitMaker {
    fn construct<'a>(&self, ctx: &ConstructorContext<'a>) -> Option<Vec<Box<dyn Operator>>> {
        let variable = ctx.formula().select_random_variable()?;
        let pos = Condition::new(variable, TruthAssignment::True);
        let neg = Condition::new(variable, TruthAssignment::False);
        Some(vec![
            new_condition_application(pos),
            new_condition_application(neg),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Clause, Formula};
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
    fn split_maker_returns_two_operators() {
        // Formula: (1 ∨ 2) — has variables, so splitting should apply.
        let formula = make_formula(vec![vec!["1", "2"]]);
        let history = History::from(formula);
        let ctx = ConstructorContext { history: &history };
        let result = SplitMaker.construct(&ctx);
        assert!(result.is_some());
        let ops = result.unwrap();
        assert_eq!(ops.len(), 2);
    }

    #[test]
    fn split_maker_returns_none_for_empty_formula() {
        // Empty formula (SAT) — no variables to split on.
        let formula = Formula::from(Vector::new());
        let history = History::from(formula);
        let ctx = ConstructorContext { history: &history };
        let result = SplitMaker.construct(&ctx);
        assert!(result.is_none());
    }

    #[test]
    fn split_maker_works_on_multi_clause_formula() {
        // Formula: (1 ∨ -2) ∧ (2 ∨ 3)
        let formula = make_formula(vec![vec!["1", "-2"], vec!["2", "3"]]);
        let history = History::from(formula);
        let ctx = ConstructorContext { history: &history };
        let result = SplitMaker.construct(&ctx);
        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 2);
    }
}
