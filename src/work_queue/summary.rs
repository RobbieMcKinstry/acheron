use crate::core::Condition;
use crate::ops::Opcode;

/// A `Summary` records an action that was taken to transform
/// a `Formula`. It records the decision that was applied to transform
/// the formula (i.e. unit propagation, pure literal elimination, etc)
/// as well as any new conditions applied to the formula.
#[derive(Clone)]
pub struct Summary {
    recent_changes: Vec<Condition>,
    code: Opcode,
}

impl Summary {
    pub fn add_change(&mut self, cond: Condition) {
        self.recent_changes.push(cond);
    }
}

impl From<Opcode> for Summary {
    fn from(code: Opcode) -> Self {
        Summary {
            recent_changes: Vec::new(),
            code,
        }
    }
}
