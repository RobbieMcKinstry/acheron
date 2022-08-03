use crate::core::Formula;
use crate::engine::DecisionEngine;
use crate::work_queue::{History, Job, WorkQueue};

pub struct Solver {
    queue: WorkQueue,
    engine: DecisionEngine,
}

impl Solver {
    pub fn new(start: History) -> Self {
        // Given a starting history, we build a DecisionEngine,
        // queue up the first job, and get ready to run.
        let engine = DecisionEngine::new();
        let mut queue = WorkQueue::new();
        let mut solver = Self { engine, queue };
        let next_job = solver.select_next_job(start);
        solver.enqueue_job(next_job);
        solver
    }

    fn enqueue_job(&mut self, job: Job) {
        self.queue.push(job);
    }

    fn select_next_job(&self, hist: History) -> Job {
        let op = self.engine.select(&hist);
        Job::new(hist, op)
    }

    // TODO: Return a trace, not just a boolean.
    /// `solve` returns unsat if the formula
    /// this solver was seeded with is unsatisfiable.
    /// Otherwise, it turns a satisfying assignment.
    /// # Panics
    ///
    pub fn solve(&mut self) -> bool {
        // TODO: Do I need to prime the queue with the first job,
        // or is that done during resource creation?
        while let Some(job) = self.queue.pop() {
            // Apply the pending operator.
            // let operator = self.decider.select(f, hist)

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
