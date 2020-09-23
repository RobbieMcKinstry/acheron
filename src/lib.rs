#![allow(dead_code)]

pub use clause::Clause;
pub use formula::Formula;
pub use frame::Frame;
pub use literal::Literal;
pub use opcode::Opcode;
pub use parser::Parser;
pub use sign::Sign;
pub use solver::Solver;
pub use status::Status;
pub use variable::Variable;

mod autoincrementer;
mod clause;
mod formula;
mod frame;
mod literal;
mod opcode;
mod parser;
mod sign;
mod solver;
mod status;
mod variable;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
