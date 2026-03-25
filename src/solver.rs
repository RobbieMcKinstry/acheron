use std::sync::atomic::{AtomicBool, Ordering};

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
        let next_jobs = solver.select_next_job(&start);
        solver.enqueue(next_jobs);
        solver
    }

    fn enqueue(&mut self, jobs: Vec<Job>) {
        for job in jobs {
            self.enqueue_job(job);
        }
    }

    fn enqueue_job(&mut self, job: Job) {
        self.queue.push(job);
    }

    fn select_next_job(&self, hist: &History) -> Vec<Job> {
        self.engine
            .select(hist)
            .into_iter()
            .map(|op| Job::new(hist, op))
            .collect()
    }

    // TODO: Return a trace, not just a boolean.
    /// `solve` returns unsat if the formula
    /// this solver was seeded with is unsatisfiable.
    /// Otherwise, it turns a satisfying assignment.
    /// # Panics
    ///
    pub fn solve(&mut self) -> bool {
        self.run_loop(&AtomicBool::new(false)).unwrap_or(false)
    }

    /// Like `solve`, but can be interrupted by setting `stop` to `true`.
    /// Returns `None` if interrupted, `Some(true)` if SAT, `Some(false)` if UNSAT.
    pub fn solve_interruptible(&mut self, stop: &AtomicBool) -> Option<bool> {
        self.run_loop(stop)
    }

    fn run_loop(&mut self, stop: &AtomicBool) -> Option<bool> {
        while let Some(job) = self.queue.pop() {
            if stop.load(Ordering::Relaxed) {
                return None;
            }
            let (history, pending) = job.take();
            let ctx = OpContext::new(&history);
            let output = pending.apply(ctx);
            match output.state() {
                TerminationState::Sat(_) => return Some(true),
                TerminationState::Unfinished => {
                    let jobs = self.select_next_job(output.history());
                    self.enqueue(jobs);
                }
                TerminationState::Unsat(_) => continue,
            }
        }
        Some(false)
    }
}

impl From<Formula> for Solver {
    fn from(formula: Formula) -> Self {
        let history = History::from(formula);
        Self::new(history)
    }
}
