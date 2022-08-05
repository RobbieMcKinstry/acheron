use super::History;
use crate::ops::Operation;

/// A Job contains the present formula, a history of prior
/// operations applied to the formula, and the pending operation
/// about to be applied.
/// In the case where these `Jobs` were created by splitting,
/// the pending operation is a condition.
pub struct Job {
    history: History,
    pending: Box<dyn Operation>,
}

impl Job {
    pub fn new(hist: &History, pending: Box<dyn Operation>) -> Self {
        let history = hist.clone();
        Self { history, pending }
    }

    /// take ownership of the values in the job.
    pub fn take(self) -> (History, Box<dyn Operation>) {
        (self.history, self.pending)
    }
}
