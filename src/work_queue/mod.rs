pub use history::History;
pub use job::Job;
pub use job_output::JobOutput;
pub use summary::Summary;
pub use termination::TerminationState;

/// A `WorkQueue` holds search paths that are pending exploration.
/// When the "splitting" operation is performed on a
/// variable X, two `Jobs` are created as part of the case
/// analysis of X:
///   1. Δ X = true
///   2. Δ X = false
/// We use a `WorkQueue` instead of recursion to not blow the stack.
pub struct WorkQueue {
    // TODO: Include a VecDeque<Job> here to serve as a Stack.
}

// TODO: Going to need to add a "iterator" implementation
// for WorkQueue... hasNext() and Next() are important operations
// for it.

mod history;
mod job;
mod job_output;
mod summary;
mod termination;
