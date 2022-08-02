use crate::core::Formula;
use crate::work_queue::WorkQueue;
use crate::History;

/// `ConstructorContext` is the input into the OpMaker's constructor
/// call. This is what an `Operation` can see to determine if
/// its allowed to be constructed for the given `Formula`.
/// It is the same as the `OpContext`, except the `OpMaker` is not
/// allowed to mutate the `WorkQueue` yet, not until the `Operation`
/// is being applied.
pub struct ConstructorContext<'a> {
    pub formula: &'a Formula,
    pub history: &'a History,
    pub queue: &'a WorkQueue,
}
