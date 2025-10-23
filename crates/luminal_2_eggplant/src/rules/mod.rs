use eggplant::wrap::{G, RuleSetId};
mod basic_expr_rules;
pub fn add_rules<T: G>(ruleset: RuleSetId) {
    basic_expr_rules::commu::<T>(ruleset);
    basic_expr_rules::const_fold::<T>(ruleset);
    basic_expr_rules::assoc::<T>(ruleset);
}
