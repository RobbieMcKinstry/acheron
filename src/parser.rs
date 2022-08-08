use crate::core::{Clause, Formula};
use crate::solver::Solver;
use im::Vector;
use std::fs;
use std::path::Path;

pub struct Parser {
    solver: Solver,
}

impl Parser {
    #[must_use]
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        // Read the file in from the path provided.
        // Then, convert it line by line into a list
        // of Clauses.
        let file = Self::read_file(path);
        let epilogue_stripped = Self::strip_trailing_epilogue(file);
        let stripped = Self::strip(&epilogue_stripped);

        // Now, iterate over each of  the lines

        let clause_lines: Vec<String> = Self::split_on_clauses(&stripped);
        // Next, we iterate over these strings
        // and split them on whitespace, capturing each
        // variable name and polarity.
        let clauses = Self::parse_clauses(clause_lines);

        for c in clauses.iter() {
            println!("{}", c);
        }
        let formula = Formula::from(clauses);
        let solver = Solver::from(formula);

        Self { solver }
    }

    pub fn solver(self) -> Solver {
        self.solver
    }

    fn read_file<T: AsRef<Path>>(path: T) -> String {
        let file_contents = fs::read_to_string(path);
        if let std::io::Result::Err(err) = file_contents {
            println!("Could not read file: {:?}", err);
            std::process::exit(1);
        }

        file_contents.unwrap()
    }

    fn strip(file: &str) -> String {
        file.lines()
            .filter(|line| !line.starts_with('c'))
            .filter(|line| !line.starts_with('p'))
            .map(ToOwned::to_owned)
            .collect()
    }

    fn split_on_clauses(file: &str) -> Vec<String> {
        file.lines()
            .fold("".to_owned(), |line1, line2| format!("{}{}", line1, line2))
            .split(" 0")
            .map(ToOwned::to_owned)
            .collect()
    }

    fn parse_clause(line: &str) -> Clause {
        let mut result = Clause::new();
        for var in line.split_whitespace() {
            result = result.add_literal(var.to_owned());
        }
        result
    }

    fn parse_clauses(clause_lines: Vec<String>) -> Vector<Clause> {
        let mut clauses = Vector::new();
        for line in clause_lines {
            let next = Self::parse_clause(&line);
            clauses.push_back(next);
        }
        clauses
    }

    /// Find the location of the
    /// % indicating the end of clauses,
    /// and remove any characters occuring after it.
    fn strip_trailing_epilogue(text: String) -> String {
        if let Some(position) = text.rfind('%') {
            return text.chars().take(position).collect();
        }
        text
    }
}
