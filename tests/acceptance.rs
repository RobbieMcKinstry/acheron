extern crate acheron;
use acheron::Parser;

#[test]
fn test_sat1() {
    let filepath = "examples/robbie/satisfiable/unit.cnf";
    let parser = Parser::new(filepath);
    let mut solver = parser.solver;
    let output = solver.solve();
    assert_eq!(output, true);
}

#[test]
fn test_unsat1() {
    let filepath = "examples/robbie/unsatisfiable/contradiction.cnf";
    let parser = Parser::new(filepath);
    let mut solver = parser.solver;
    let output = solver.solve();
    assert_eq!(output, false);
}

#[test]
fn test_unsat2() {
    let filepath = "examples/robbie/unsatisfiable/contradiction2.cnf";
    let parser = Parser::new(filepath);
    let mut solver = parser.solver;
    let output = solver.solve();
    assert_eq!(output, false);
}
