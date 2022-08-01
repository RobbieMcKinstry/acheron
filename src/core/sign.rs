use crate::core::TruthAssignment;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}

impl From<Sign> for TruthAssignment {
    fn from(sign: Sign) -> Self {
        match sign {
            Sign::Positive => Self::True,
            Sign::Negative => Self::False,
        }
    }
}
