use std::sync::atomic::{AtomicBool, Ordering};

use crate::core::Formula;
use crate::engine::DecisionEngine;
use crate::ops::OpContext;
use crate::work_queue::{History, Job, TerminationState, WorkQueue};

pub struct Solver {
    start: History,
    engine: DecisionEngine,
}

impl Solver {
    pub fn new(start: History) -> Self {
        let engine = DecisionEngine::new();
        Self { start, engine }
    }

    // TODO: Return a trace, not just a boolean.
    /// `solve` returns unsat if the formula
    /// this solver was seeded with is unsatisfiable.
    /// Otherwise, it turns a satisfying assignment.
    /// # Panics
    ///
    pub fn solve(&self) -> bool {
        let stop = AtomicBool::new(false);
        let sat_found = AtomicBool::new(false);
        self.solve_from_history(&self.start, &stop, &sat_found)
    }

    /// Like `solve`, but can be interrupted by setting `stop` to `true`.
    /// Returns `None` if interrupted, `Some(true)` if SAT, `Some(false)` if UNSAT.
    pub fn solve_interruptible(&self, stop: &AtomicBool) -> Option<bool> {
        let sat_found = AtomicBool::new(false);
        let result = self.solve_from_history(&self.start, stop, &sat_found);
        if result {
            Some(true)
        } else if stop.load(Ordering::Relaxed) {
            None
        } else {
            Some(false)
        }
    }

    /// Core solving function. Returns `true` if SAT was found, `false` otherwise.
    ///
    /// - `stop`: external interruption flag (never written by the solver).
    /// - `sat_found`: set to `true` when any branch finds SAT, causing all
    ///   sibling branches to short-circuit.
    fn solve_from_history(
        &self,
        start: &History,
        stop: &AtomicBool,
        sat_found: &AtomicBool,
    ) -> bool {
        if stop.load(Ordering::Relaxed) || sat_found.load(Ordering::Relaxed) {
            return false;
        }

        let mut queue = WorkQueue::new();
        for job in self.select_next_job(start) {
            queue.push(job);
        }

        while let Some(job) = queue.pop() {
            if stop.load(Ordering::Relaxed) || sat_found.load(Ordering::Relaxed) {
                return false;
            }

            let (history, pending) = job.take();
            let ctx = OpContext::new(&history);
            let output = pending.apply(ctx);

            match output.state() {
                TerminationState::Sat(_) => {
                    sat_found.store(true, Ordering::Relaxed);
                    return true;
                }
                TerminationState::Unfinished => {
                    let mut jobs = self.select_next_job(output.history());
                    if jobs.len() == 2 {
                        // Split: eagerly apply both conditions, then decide
                        // whether to spawn parallel work.
                        let job_b = jobs.pop().unwrap();
                        let job_a = jobs.pop().unwrap();

                        let (hist_a, op_a) = job_a.take();
                        let (hist_b, op_b) = job_b.take();

                        let output_a = op_a.apply(OpContext::new(&hist_a));
                        let output_b = op_b.apply(OpContext::new(&hist_b));

                        match (output_a.state(), output_b.state()) {
                            (TerminationState::Sat(_), _) | (_, TerminationState::Sat(_)) => {
                                sat_found.store(true, Ordering::Relaxed);
                                return true;
                            }
                            (TerminationState::Unsat(_), TerminationState::Unsat(_)) => {
                                continue;
                            }
                            (TerminationState::Unfinished, TerminationState::Unsat(_)) => {
                                for j in self.select_next_job(output_a.history()) {
                                    queue.push(j);
                                }
                                continue;
                            }
                            (TerminationState::Unsat(_), TerminationState::Unfinished) => {
                                for j in self.select_next_job(output_b.history()) {
                                    queue.push(j);
                                }
                                continue;
                            }
                            (TerminationState::Unfinished, TerminationState::Unfinished) => {
                                let (found_a, found_b) = rayon::join(
                                    || {
                                        self.solve_from_history(
                                            output_a.history(),
                                            stop,
                                            sat_found,
                                        )
                                    },
                                    || {
                                        self.solve_from_history(
                                            output_b.history(),
                                            stop,
                                            sat_found,
                                        )
                                    },
                                );
                                if found_a || found_b {
                                    return true;
                                }
                                continue;
                            }
                        }
                    } else {
                        for job in jobs {
                            queue.push(job);
                        }
                    }
                }
                TerminationState::Unsat(_) => continue,
            }
        }
        false
    }

    fn select_next_job(&self, hist: &History) -> Vec<Job> {
        self.engine
            .select(hist)
            .into_iter()
            .map(|op| Job::new(hist, op))
            .collect()
    }
}

impl From<Formula> for Solver {
    fn from(formula: Formula) -> Self {
        let history = History::from(formula);
        Self::new(history)
    }
}
