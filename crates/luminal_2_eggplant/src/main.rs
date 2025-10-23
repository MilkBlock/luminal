mod datatypes;
mod rules;
mod shortcut;
use datatypes::*;
use eggplant::{prelude::*, tx_rx_vt_pr};

tx_rx_vt_pr!(MyTx, MyPatRec);
fn main() {
    // let expr: Expr<MyTx, ()> = (MNum::new(4) * 3) + 2;
    let expr: Expr<MyTx, _> = MNum::new(4);
    let ruleset = MyTx::new_ruleset("expr");
    rules::add_rules::<MyTx>(ruleset);
    expr.commit();
    MyTx::run_ruleset(ruleset, RunConfig::Sat);
}

#[test]
fn test_const_fold() {
    let expr: Expr<MyTx, _> = MNum::new(3) * MNum::new(4) + MNum::new(2);
    expr.commit();
    let ruleset = MyTx::new_ruleset("expr");
    rules::add_rules::<MyTx>(ruleset);
    MyTx::run_ruleset(ruleset, RunConfig::Sat);

    let ans: Expr<MyTx, _> = MNum::new(12);
    ans.commit();
    assert!(MyTx::canonical_raw(&expr) == MyTx::canonical_raw(&ans))
}
