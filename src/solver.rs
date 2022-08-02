use crate::core::{Formula, Status};
use crate::History;
use std::collections::VecDeque;

pub struct Solver {
    queue: VecDeque<History>,
}

impl Solver {
    // TODO: Return a trace, not just a boolean.
    /// `solve` returns unsat if the formula
    /// this solver was seeded with is unsatisfiable.
    /// Otherwise, it turns a satisfying assignment.
    /// # Panics
    ///
    pub fn solve(&mut self) -> bool {
        while let Some(next) = self.queue.pop_front() {
            let new_histories = next.apply();
            for history in new_histories.iter() {
                match history.status() {
                    Status::Sat => return true,
                    Status::Unsat => continue,
                    Status::Unknown => self.queue.push_back(history.clone()),
                }
            }
        }
        false
    }
}

impl From<Formula> for Solver {
    fn from(formula: Formula) -> Self {
        let mut queue = VecDeque::new();
        let history = History::from(formula);
        queue.push_back(history);
        Self { queue }
    }
}
