use super::History;
use crate::core::Formula;
use crate::ops::Operation;

/// A Job contains the present formula, a history of prior
/// operations applied to the formula, and the pending operation
/// about to be applied.
/// In the case where these `Jobs` were created by splitting,
/// the pending operation is a condition.
pub struct Job {
    state: Formula,
    history: History,
    pending: Box<dyn Operation>,
}

impl Job {
    pub fn new(f: Formula, history: History, pending: Box<dyn Operation>) -> Self {
        Self {
            state: f,
            history,
            pending,
        }
    }
}
