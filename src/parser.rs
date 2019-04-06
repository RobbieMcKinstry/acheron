use crate::Clause;
use std::fs;

pub struct Parser {}

impl Parser {
    pub fn new(path: String) -> Self {
        // Read the file in from the path provided.
        // Then, convert it line by line into a list
        // of Clauses.
        let file = Self::read_file(path);
        let epilogue_stripped = Self::strip_trailing_epilogue(file);
        let stripped = Self::strip(epilogue_stripped);

        // Now, iterate over each of  the lines

        let clause_lines: Vec<String> = Self::split_on_clauses(stripped);
        // Next, we iterate over these strings
        // and split them on whitespace, capturing each
        // variable name and polarity.
        let clauses = Self::parse_clauses(clause_lines);

        for c in clauses.iter() {
            println!("{}", c);
        }

        Parser {}
    }

    fn read_file(path: String) -> String {
        let file_contents = fs::read_to_string(path.clone());
        if let Err(_) = file_contents {
            println!("Could not read file {}", path);
            std::process::exit(1);
        }

        file_contents.unwrap()
    }

    fn strip(file: String) -> String {
        file.lines()
            .filter(|line| !line.starts_with('c'))
            .filter(|line| !line.starts_with('p'))
            .map(|line| line.to_owned())
            .collect()
    }

    fn split_on_clauses(file: String) -> Vec<String> {
        file.lines()
            .fold("".to_owned(), |line1, line2| format!("{}{}", line1, line2))
            .split(" 0")
            .map(|a| a.to_owned())
            .collect()
    }

    fn parse_clause(line: String) -> Clause {
        let mut result = Clause::new();
        for var in line.split_whitespace() {
            result = result.add_variable(var.to_owned());
        }
        result
    }

    fn parse_clauses(clause_lines: Vec<String>) -> Vec<Clause> {
        let mut clauses = Vec::new();
        for line in clause_lines {
            let next = Self::parse_clause(line);
            if !next.is_empty() {
                clauses.push(next);
            }
        }
        clauses
    }

    /// Find the location of the
    /// % indicating the end of clauses,
    /// and remove any characters occuring after it.
    fn strip_trailing_epilogue(text: String) -> String {
        if let Some(position) = text.rfind("%") {
            return text.chars().take(position).collect();
        }
        text
    }
}
