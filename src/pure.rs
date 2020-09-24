use crate::{Formula, Variable};
use im::Hashmap;

/// A `PureTracker` counts how many times each variable
/// appears in a clause with either a positive or negative
/// polarity. If a clause 0 counts in one polarity and
/// non-zero counts in another polarity, then it's "pure"
/// and can be resolved.
pub struct PureTracker {
    counts: Hashmap<Literal, usize>,
}

impl PureTracker {
    fn new() -> Self {
        Self {
            counts: Default::default(),
        }
    }

    /// `seed` will create a PureTracker seeded with the literals
    /// from the given formula.
    pub fn seed(formula: &Formula) -> Self {
        let mut tracker = Default::default();
        for lit in formula.flatten().iter() {
            tracker.increment(lit);
        }
        tracker
    }

    fn increment(&mut self, lit: &Literal) {
        self.counts.alter(
            |val| match val {
                Some(integer) => Some(integer + 1),
                None => Some(1),
            },
            lit,
        );
    }
}

impl Default for PureTracker {
    fn default() -> Self {
        PureTracker::new()
    }
}
