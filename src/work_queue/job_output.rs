use super::TerminationState;
use crate::core::Formula;
use crate::work_queue::History;

pub struct JobOutput {
    /// The formula is the newly transformed formula afer the
    /// job has been run.
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
