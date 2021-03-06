use crate::{Formula, Frame, Status};
use std::collections::VecDeque;

pub struct Solver {
    queue: VecDeque<Frame>,
}

impl Solver {
    // TODO: Return a trace, not just a boolean.
    /// `solve` returns unsat if the formula
    /// this solver was seeded with is unsatisfiable.
    /// Otherwise, it turns a satisfying assignment.
    pub fn solve(&mut self) -> bool {
        while !self.queue.is_empty() {
            let next = self.queue.pop_front().unwrap();
            let new_frames = next.apply();
            for frame in new_frames.iter() {
                match frame.is_sat() {
                    Status::Sat => return true,
                    Status::Unsat => continue,
                    Status::Unknown => self.queue.push_back(frame.clone()),
                }
            }
        }
        false
    }
}

impl From<Formula> for Solver {
    fn from(formula: Formula) -> Self {
        let mut queue = VecDeque::new();
        let frame = Frame::from(formula);
        queue.push_back(frame);
        Self { queue }
    }
}
