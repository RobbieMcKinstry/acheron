extern crate acheron;
use acheron::Parser;

#[test]
#[ignore = "Currently failing! This one parses an empty clause where one shouldn't exist."]
fn test_sat1() {
    let filepath = "examples/robbie/satisfiable/unit.cnf";
    let parser = Parser::new(filepath);
    let mut solver = parser.solver;
    let output = solver.solve();
    assert_eq!(output, true);
}

#[test]
#[ignore = "Currently failing!"]
fn test_unsat1() {
    let filepath = "examples/robbie/unsatisfiable/contradiction.cnf";
    let parser = Parser::new(filepath);
    let mut solver = parser.solver;
    let output = solver.solve();
    assert_eq!(output, false);
}

#[test]
#[ignore = "Failing. History::Apply is deprecated."]
fn test_unsat2() {
    let filepath = "examples/robbie/unsatisfiable/contradiction2.cnf";
    let parser = Parser::new(filepath);
    let mut solver = parser.solver;
    let output = solver.solve();
    assert_eq!(output, false);
}
