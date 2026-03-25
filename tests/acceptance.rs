extern crate acheron;
use acheron::Parser;

#[test]
fn test_sat1() {
    let filepath = "examples/robbie/satisfiable/unit.cnf";
    let parser = Parser::new(filepath);
    let solver = parser.solver;
    let output = solver.solve();
    assert_eq!(output, true);
}

#[test]
fn test_unsat1() {
    let filepath = "examples/robbie/unsatisfiable/contradiction.cnf";
    let parser = Parser::new(filepath);
    let solver = parser.solver;
    let output = solver.solve();
    assert_eq!(output, false);
}

#[test]
fn test_unsat2() {
    let filepath = "examples/robbie/unsatisfiable/contradiction2.cnf";
    let parser = Parser::new(filepath);
    let solver = parser.solver;
    let output = solver.solve();
    assert_eq!(output, false);
}

#[test]
fn test_sat_requires_split() {
    // (1 ∨ 2) ∧ (¬1 ∨ 2) — no unit clauses, requires splitting.
    let filepath = "examples/robbie/satisfiable/split.cnf";
    let parser = Parser::new(filepath);
    let solver = parser.solver;
    assert!(solver.solve());
}

#[test]
fn test_sat_requires_multiple_splits() {
    // (1 ∨ 2) ∧ (¬1 ∨ 3) ∧ (¬2 ∨ 3) — requires multiple splits.
    let filepath = "examples/robbie/satisfiable/split2.cnf";
    let parser = Parser::new(filepath);
    let solver = parser.solver;
    assert!(solver.solve());
}

#[test]
fn test_unsat_pigeonhole() {
    // 3 pigeons, 2 holes — unsatisfiable, requires splitting.
    let filepath = "examples/robbie/unsatisfiable/pigeonhole2.cnf";
    let parser = Parser::new(filepath);
    let solver = parser.solver;
    assert!(!solver.solve());
}
