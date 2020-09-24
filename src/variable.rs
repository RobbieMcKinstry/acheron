use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Variable {
    id: u64,
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl From<u64> for Variable {
    fn from(id: u64) -> Self {
        Self { id }
    }
}
