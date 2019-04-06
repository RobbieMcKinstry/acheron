use crate::Clause;
use std::fs;

pub struct Parser {}

impl Parser {
    pub fn new(path: String) -> Self {
        // Read the file in from the path provided.
        // Then, convert it line by line into a list
        // of Clauses.
        let file_contents_opt = fs::read_to_string(path.clone());

        if let Err(_) = file_contents_opt {
            println!("Could not read file {}", path);
            std::process::exit(1);
        }
        let file_contents = file_contents_opt.unwrap();

        // Now, iterate over each of  the lines
        let stripped = file_contents
            .lines()
            .filter(|line| !line.starts_with('c'))
            .filter(|line| !line.starts_with('p'))
            .filter(|line| !line.starts_with('%'))
            .map(|line| line.to_owned());

        let clauses: Vec<String> = stripped
            .fold("".to_owned(), |line1, line2| format!("{}{}", line1, line2))
            .split("0")
            .map(|a| a.to_owned())
            .collect();

        // Next, we iterate over these strings
        // and split them on whitespace, capturing each
        // variable name and polarity.
        let mut domain_clauses = Vec::new();
        for clause in clauses {
            let mut next_clause = Clause::new();

            for var in clause.split_whitespace() {
                // if it starts with a - minus sign,
                // its negative. Else, its posibie
                next_clause = next_clause.add_variable(var.to_owned());
            }
            domain_clauses.push(next_clause);
        }

        for c in domain_clauses {
            println!("{}", c);
        }

        Parser {}
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
