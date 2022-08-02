/// The `SatStatus` tells if a clause or formula is satisfiable,
/// unsatisfiable, or unknown. If a formula or clause is satisfiable, then
/// it will come with a satisfying assignment.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Sat,
    Unsat,
    Unknown,
}
