#![allow(dead_code)]

pub use clause::Clause;
pub use parser::Parser;
pub use sign::Sign;

mod autoincrementer;
mod clause;
mod parser;
mod sign;
mod variable;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
