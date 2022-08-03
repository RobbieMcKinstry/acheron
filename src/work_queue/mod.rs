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
    pending_jobs: Vec<Job>,
}

impl WorkQueue {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, j: Job) {
        self.pending_jobs.push(j);
    }

    pub fn pop(&mut self) -> Option<Job> {
        self.pending_jobs.pop()
    }
}

impl Default for WorkQueue {
    fn default() -> Self {
        Self {
            pending_jobs: Vec::new(),
        }
    }
}

// TODO: Going to need to add a "iterator" implementation
// for WorkQueue... hasNext() and Next() are important operations
// for it.

mod history;
mod job;
mod job_output;
mod summary;
mod termination;
