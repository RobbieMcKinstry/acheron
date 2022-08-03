use crate::core::Formula;
use crate::engine::DecisionEngine;
use crate::ops::OpContext;
use crate::work_queue::{History, Job, TerminationState, WorkQueue};

pub struct Solver {
    queue: WorkQueue,
    engine: DecisionEngine,
}

impl Solver {
    pub fn new(start: History) -> Self {
        // Given a starting history, we build a DecisionEngine,
        // queue up the first job, and get ready to run.
        let engine = DecisionEngine::new();
        let queue = WorkQueue::new();
        let mut solver = Self { engine, queue };
        let next_job = solver.select_next_job(&start);
        solver.enqueue_job(next_job);
        solver
    }

    fn enqueue_job(&mut self, job: Job) {
        self.queue.push(job);
    }

    fn select_next_job(&self, hist: &History) -> Job {
        let op = self.engine.select(hist);
        Job::new(hist, op)
    }

    // TODO: Return a trace, not just a boolean.
    /// `solve` returns unsat if the formula
    /// this solver was seeded with is unsatisfiable.
    /// Otherwise, it turns a satisfying assignment.
    /// # Panics
    ///
    pub fn solve(&mut self) -> bool {
        // Continue to take items from the stack
        // until the stack is exhausted, or until
        // we've reached a termination case.
        while let Some(job) = self.queue.pop() {
            // Capture the history from the job.
            // Apply the pending operator.
            let (history, pending) = job.take();
            let ctx = OpContext::new(&history);
            let output = pending.apply(ctx);
            match output.state() {
                TerminationState::Sat(_) => {
                    // Satisfied! We can exit with the result!
                    return true;
                }
                TerminationState::Unfinished => {
                    // The operation succeeded, but there's
                    // more work to be done.
                    let j = self.select_next_job(output.history());
                    self.enqueue_job(j);
                }
                TerminationState::Unsat(_) => continue,
            }

            /*
            let new_histories = next.apply();
            for history in new_histories.iter() {
                match history.status() {
                    Status::Sat => return true,
                    Status::Unsat => continue,
                    Status::Unknown => self.queue.push_back(history.clone()),
                }
            }
            */
        }

        false
    }
}

impl From<Formula> for Solver {
    fn from(formula: Formula) -> Self {
        let history = History::from(formula);
        Self::new(history)
    }
}
