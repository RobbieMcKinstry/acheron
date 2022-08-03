pub use constructor_context::ConstructorContext;
pub use maker::OpMaker;
pub use op_context::OpContext;
pub use opcode::Opcode;
pub use operation::Operation;
pub use sat::{SatOpMaker as SatMaker, SatOperator};
pub use unsat::{UnsatOpMaker as UnsatMaker, UnsatOperator};

pub mod prelude {
    pub use super::{OpMaker, Operation};
    pub use super::{SatMaker, SatOperator, UnsatMaker, UnsatOperator};
}

mod constructor_context;
mod maker;
mod op_context;
mod opcode;
mod operation;
mod pure_lit;
mod sat;
// mod unit_prop;
mod unsat;
