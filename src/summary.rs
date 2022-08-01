use super::Literal;
use super::Opcode;

/// A `Summary` records an action that was taken to transform
/// a `Formula`. It records the decision that was applied to transform
/// the formula (i.e. unit propagation, pure literal elimination, etc)
/// as well as any new conditions applied to the formula.
#[derive(Clone)]
pub struct Summary {
    recent_conditions: Vec<Literal>,
    code: Opcode,
}

impl Summary {
    pub fn add_condition(mut self, lit: Literal) -> Self {
        self.recent_conditions.push(lit);
        self
    }
}

impl From<Opcode> for Summary {
    fn from(code: Opcode) -> Self {
        Summary {
            recent_conditions: Vec::new(),
            code,
        }
    }
}
