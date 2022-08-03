use super::Job;
use crate::core::Condition;

/// After an operator is applied
/// to a Job, the job can terminal, or
/// it can require additional processing.
/// This enum tracks those states.
/// A job can terminate as Sat, in which case
/// the variant contains a list of satisfying
/// literals (or a unsigned Literal...).
/// If it is unsat, the variant contains the
/// "conflict set". If it doesn't terminate,
/// then a list of Pending Jobs.
pub enum TerminationState {
    Sat(Vec<Condition>),
    Unsat(Vec<Condition>),
    Unknown(Vec<Job>),
}
