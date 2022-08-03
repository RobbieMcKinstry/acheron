use crate::core::Formula;
use crate::work_queue::History;

/// `OpContext` is the input into an `Operator`.
/// An `Operator` is allowed to inspect the `Formula`,
/// the `History` up to this point, and the `WorkQueue`.
pub struct OpContext<'a> {
    history: &'a History,
}

impl<'a> OpContext<'a> {
    pub fn new(history: &'a History) -> Self {
        Self { history }
    }

    pub fn formula(&self) -> &Formula {
        self.history.formula()
    }

    pub fn history(&self) -> &History {
        &self.history
    }
}
