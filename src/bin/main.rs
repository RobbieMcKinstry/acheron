use clap::{App, Arg};

use acheron::Parser;

fn main() {
    let matches = App::new("Acheron")
        .version("0.1")
        .author("Robbie McKinstry <thesnowmancometh@gmail.com>")
        .about("SAT Solver")
        .arg(
            Arg::with_name("INPUT")
                .help("path to the Dimacs file to solve")
                .required(true)
                .index(1),
        )
        .get_matches();
    let filepath = matches.value_of("INPUT").unwrap();
    Parser::new(filepath.to_owned());
}
