use super::TerminationState;
use crate::core::Formula;
use crate::history::History;

pub struct JobOutput {
    /// The formula is
    formula: Formula,
    state: TerminationState,
    history: History,
}

impl JobOutput {
    pub fn new(f: Formula, s: TerminationState, hist: History) -> Self {
        Self {
            formula: f,
            state: s,
            history: hist,
        }
    }
}
