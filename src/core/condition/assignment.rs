use std::fmt;

#[derive(Clone, Copy)]
pub enum TruthAssignment {
    True,
    False,
}

impl fmt::Display for TruthAssignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::True => write!(f, "T"),
            Self::False => write!(f, "F"),
        }
    }
}
