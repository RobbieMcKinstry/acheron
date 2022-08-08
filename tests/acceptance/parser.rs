extern crate acheron;

use std::path::{Path, PathBuf};

use acheron::Parser;

const BASE_PATH: &str = "examples/satisfiable/uniform-random-3sat";

#[test]
fn parses_ex1() {
    let path = join("uf20-01.cnf");
    let parser = Parser::new(path);
    _ = parser;
}

fn join(path: &str) -> PathBuf {
    Path::new(BASE_PATH).join(path)
}
