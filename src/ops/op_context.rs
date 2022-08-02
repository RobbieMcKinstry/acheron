use crate::core::Formula;
use crate::work_queue::WorkQueue;
use crate::History;

/// `OpContext` is the input into an `Operator`.
/// An `Operator` is allowed to inspect the `Formula`,
/// the `History` up to this point, and the `WorkQueue`.
pub struct OpContext<'a> {
    pub formula: &'a Formula,
    pub history: &'a History,
    pub queue: &'a mut WorkQueue,
}
