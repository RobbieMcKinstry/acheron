use crate::core::{Clause, Formula};
use crate::solver::Solver;
use im::Vector;
use std::fs;

pub struct Parser {
    pub solver: Solver,
}

impl Parser {
    #[must_use]
    pub fn new(path: &str) -> Self {
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

        let formula = Formula::from(clauses);
        let solver = Solver::from(formula);

        Self { solver }
    }

    fn read_file(path: &str) -> String {
        let file_contents = fs::read_to_string(path);
        if file_contents.is_err() {
            eprintln!("Could not read file {}", path);
            std::process::exit(1);
        }

        file_contents.unwrap()
    }

    fn strip(file: &str) -> String {
        file.lines()
            .filter(|line| !line.starts_with('c'))
            .filter(|line| !line.starts_with('p'))
            .collect::<Vec<&str>>()
            .join(" ")
    }

    fn split_on_clauses(file: &str) -> Vec<String> {
        file.split(" 0")
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
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

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn strip_removes_comments_and_problem_line() {
        let input = "c This is a comment\nc Another comment\np cnf 3 2\n1 -2 0\n-3 0\n";
        let result = Parser::strip(input);
        assert_eq!(result, "1 -2 0 -3 0");
    }

    #[test]
    fn strip_preserves_spacing_between_lines() {
        let input = "4 -2 0\n2 -3 0\n";
        let result = Parser::strip(input);
        // Lines should be joined with a space, not concatenated directly.
        assert_eq!(result, "4 -2 0 2 -3 0");
    }

    #[test]
    fn split_on_clauses_no_trailing_empty_clause() {
        let input = "1 -2 0 3 0";
        let result = Parser::split_on_clauses(input);
        // Should not produce an empty trailing entry.
        assert_eq!(result, vec!["1 -2", "3"]);
    }

    #[test]
    fn split_on_clauses_single_clause() {
        let result = Parser::split_on_clauses("1 0");
        assert_eq!(result, vec!["1"]);
    }

    #[test]
    fn split_on_clauses_trailing_whitespace_after_terminator() {
        let result = Parser::split_on_clauses("-4 0 ");
        assert_eq!(result, vec!["-4"]);
    }

    #[test]
    fn parse_clause_single_literal() {
        let clause = Parser::parse_clause("1");
        assert_eq!(clause.iter().count(), 1);
    }

    #[test]
    fn parse_clause_multiple_literals() {
        let clause = Parser::parse_clause("4 -2 3");
        assert_eq!(clause.iter().count(), 3);
    }

    #[test]
    fn parse_clause_empty_string_produces_empty_clause() {
        let clause = Parser::parse_clause("");
        assert_eq!(clause.iter().count(), 0);
    }

    #[test]
    fn strip_trailing_epilogue_removes_content_after_percent() {
        let input = "1 0\n2 0\n%\n0\n".to_owned();
        let result = Parser::strip_trailing_epilogue(input);
        assert_eq!(result, "1 0\n2 0\n");
    }

    #[test]
    fn strip_trailing_epilogue_noop_without_percent() {
        let input = "1 0\n2 0\n".to_owned();
        let result = Parser::strip_trailing_epilogue(input);
        assert_eq!(result, "1 0\n2 0\n");
    }

    #[test]
    fn full_pipeline_unit_cnf() {
        // Simulates examples/robbie/satisfiable/unit.cnf: "1 0\n"
        let input = "1 0\n";
        let stripped = Parser::strip(input);
        let clauses = Parser::split_on_clauses(&stripped);
        assert_eq!(clauses, vec!["1"]);
        let parsed = Parser::parse_clauses(clauses);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].iter().count(), 1);
    }

    #[test]
    fn full_pipeline_contradiction_cnf() {
        // Simulates examples/robbie/unsatisfiable/contradiction.cnf: "1  0\n-1 0\n"
        let input = "1  0\n-1 0\n";
        let stripped = Parser::strip(input);
        let clauses = Parser::split_on_clauses(&stripped);
        assert_eq!(clauses, vec!["1", "-1"]);
        let parsed = Parser::parse_clauses(clauses);
        assert_eq!(parsed.len(), 2);
    }

    #[test]
    fn full_pipeline_contradiction2_cnf() {
        // Simulates examples/robbie/unsatisfiable/contradiction2.cnf
        let input = "4 -2 0\n2 -3 0\n3 4 0\n-4 0 \n";
        let stripped = Parser::strip(input);
        let clauses = Parser::split_on_clauses(&stripped);
        assert_eq!(clauses, vec!["4 -2", "2 -3", "3 4", "-4"]);
        let parsed = Parser::parse_clauses(clauses);
        assert_eq!(parsed.len(), 4);
    }

    #[test]
    fn full_pipeline_with_comments_and_problem_line() {
        let input = "c comment\np cnf 100 430\n26 -99 7 0\n-90 84 -94 0\n";
        let stripped = Parser::strip(input);
        let clauses = Parser::split_on_clauses(&stripped);
        assert_eq!(clauses, vec!["26 -99 7", "-90 84 -94"]);
    }
}
