/// `ConditionEffect` is the result of applying a condition
/// to a literal. Suppose you have the literal ¬A.
/// 1. If the condition is Δ|A=True, then ConditionEffect = Unsat.
/// 2. If the condition is Δ|A=False, then ConditionEffect = Sat.
/// 3. If the literal is any other literal B,
///    then ConditionEffect = No Impact.
pub enum ConditionEffect {
    Sat,
    Unsat,
    NoImpact,
}
