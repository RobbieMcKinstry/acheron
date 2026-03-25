use crate::core::clause::ClauseAssignment;
use crate::core::{Clause, Condition, Literal, Sign, Status, Variable};
use im::Vector;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct Formula {
    clauses: Vector<Clause>,
}

impl Formula {
    /// `assign` will construct a new formula with the given
    /// truth-assignment.
    #[must_use]
    pub fn assign(&self, cond: Condition) -> Self {
        // Iterate through the clauses, applying this assignment.
        // If a clause is satisfied, then do not include it
        // in the vector.
        let clauses = self
            .clauses
            .iter()
            // Remove clauses that have been satisfied.
            .filter_map(|clause| match clause.assign(cond) {
                ClauseAssignment::Satisfied => None,
                ClauseAssignment::Other(c) => Some(c),
            })
            .collect();
        Self { clauses }
    }

    /// A `Formula` is satisfied if there are no unsatisfied clauses.
    pub fn is_sat(&self) -> bool {
        self.clauses.is_empty()
    }

    /// A `Formula` is unsatisfied if it has an empty clause.
    /// This represents the case where one clause has all of its
    /// literals assigned, and none of them have been satisfied
    /// by assignment. Thus, there is no opportunity for the clause
    /// to be satisfied.
    pub fn is_unsat(&self) -> bool {
        self.clauses.iter().any(Clause::is_unsat)
    }

    pub fn status(&self) -> Status {
        if self.is_sat() {
            Status::Sat
        } else if self.is_unsat() {
            Status::Unsat
        } else {
            Status::Unknown
        }
    }

    /// `unit_literals` returns the list of unit literals
    /// found within this formula. If there are no literals
    /// in the forumua, an empty vector is returned.
    pub fn unit_literals(&self) -> Vec<Literal> {
        self.clauses.iter().filter_map(Clause::unit).collect()
    }

    /// `pure_literals` returns the list of pure literals in this formula.
    /// A literal is pure if the variable only appears with a single polarity
    /// across all clauses. If there are no pure literals, an empty vector
    /// is returned.
    pub fn pure_literals(&self) -> Vec<Literal> {
        let mut polarities: HashMap<Variable, HashSet<Sign>> = HashMap::new();
        for clause in self.clauses.iter() {
            for lit in clause.iter() {
                polarities.entry(lit.var()).or_default().insert(lit.sign());
            }
        }
        polarities
            .into_iter()
            .filter_map(|(var, signs)| {
                if signs.len() == 1 {
                    let sign = signs.into_iter().next().unwrap();
                    Some(Literal::new(var, sign))
                } else {
                    None
                }
            })
            .collect()
    }

    // TODO: Remove this function.
    #[must_use]
    pub fn select_random_variable(&self) -> Option<Variable> {
        // Scan through the list of clauses and find
        // one that is not yet sat. Then, select a random
        // literal in it, and condition that literal.
        self.clauses
            .front()
            .map(|clause| clause.select_random_variable())
            .flatten()
    }

    pub fn iter<'a>(&'a self) -> im::vector::Iter<'a, Clause> {
        self.clauses.iter()
    }
}

/// Build a Formula from a list of clauses
impl From<Vector<Clause>> for Formula {
    fn from(clauses: Vector<Clause>) -> Self {
        Self { clauses }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
    fn pure_literals_empty_formula() {
        let formula = Formula::from(Vector::new());
        assert_eq!(formula.pure_literals().len(), 0);
    }

    #[test]
    fn pure_literals_finds_positive_only() {
        // x1 appears only positive across all clauses
        // x2 appears both positive and negative
        let formula = make_formula(vec![vec!["1", "2"], vec!["1", "-2"]]);
        let pures = formula.pure_literals();
        assert_eq!(pures.len(), 1);
        let lit = pures[0];
        assert_eq!(lit.var(), Variable::from(1));
        assert_eq!(lit.sign(), Sign::Positive);
    }

    #[test]
    fn pure_literals_finds_negative_only() {
        // x1 appears only negative
        // x2 appears both polarities
        let formula = make_formula(vec![vec!["-1", "2"], vec!["-1", "-2"]]);
        let pures = formula.pure_literals();
        assert_eq!(pures.len(), 1);
        let lit = pures[0];
        assert_eq!(lit.var(), Variable::from(1));
        assert_eq!(lit.sign(), Sign::Negative);
    }

    #[test]
    fn pure_literals_excludes_mixed() {
        // x1 appears both positive and negative → not pure
        let formula = make_formula(vec![vec!["1", "2"], vec!["-1", "2"]]);
        let pures = formula.pure_literals();
        // x2 is pure (positive only), x1 is not
        assert_eq!(pures.len(), 1);
        assert_eq!(pures[0].var(), Variable::from(2));
    }

    #[test]
    fn pure_literals_multiple() {
        // x1 only positive, x2 only negative, x3 both polarities
        let formula = make_formula(vec![vec!["1", "-2", "3"], vec!["1", "-2", "-3"]]);
        let mut pures = formula.pure_literals();
        pures.sort_by_key(|l| format!("{}", l.var()));
        assert_eq!(pures.len(), 2);
        // Verify both x1 (positive) and x2 (negative) are found
        let vars: HashSet<_> = pures
            .iter()
            .map(|l| (format!("{}", l.var()), l.sign()))
            .collect();
        assert!(vars.contains(&("1".to_string(), Sign::Positive)));
        assert!(vars.contains(&("2".to_string(), Sign::Negative)));
    }

    #[test]
    fn pure_literals_all_pure() {
        // Every variable appears with only one polarity
        let formula = make_formula(vec![vec!["1", "-2"], vec!["1", "-2"]]);
        let pures = formula.pure_literals();
        assert_eq!(pures.len(), 2);
    }

    #[test]
    fn pure_literals_none_pure() {
        // Every variable appears with both polarities
        let formula = make_formula(vec![vec!["1", "-2"], vec!["-1", "2"]]);
        let pures = formula.pure_literals();
        assert_eq!(pures.len(), 0);
    }
}
