use super::TerminationState;
use crate::core::Formula;
use crate::work_queue::History;

pub struct JobOutput {
    state: TerminationState,
    history: History,
}

impl JobOutput {
    pub fn new(hist: History, s: TerminationState) -> Self {
        Self {
            history: hist,
            state: s,
        }
    }

    pub fn history(&self) -> &History {
        &self.history
    }

    pub fn formula(&self) -> &Formula {
        self.history.formula()
    }

    pub fn state(&self) -> &TerminationState {
        &self.state
    }
}
