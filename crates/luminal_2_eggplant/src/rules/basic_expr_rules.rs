use crate::datatypes::*;
use eggplant::prelude::*;
use eggplant::wrap::{G, RuleCtx, RuleSetId};
macro_rules! fold {
    ($ty:ident,$f:expr,$pat_name:ident,$ruleset:ident) => {
        T::add_rule(
            stringify!($pat_name),
            $ruleset,
            || {
                let l = MNum::query();
                let r = MNum::query();
                let p = $ty::query(&l, &r);
                #[eggplant::pat_vars_catch]
                struct $pat_name {
                    l: MNum,
                    r: MNum,
                    p: $ty,
                }
            },
            |ctx, pat| {
                let cal = $f(ctx.devalue(pat.l.num), ctx.devalue(pat.r.num));
                let op_value = ctx.insert_m_num(cal);
                ctx.union(pat.p, op_value);
            },
        );
    };
}

macro_rules! commu {
    ($ty:ident,$f:expr,$pat_name:ident,$ruleset:ident) => {
        T::add_rule(
            stringify!($pat_name),
            $ruleset,
            || {
                let l = Expr::query_leaf();
                let r = Expr::query_leaf();
                let p = $ty::query(&l, &r);
                #[eggplant::pat_vars_catch]
                struct $pat_name {
                    l: Expr,
                    r: Expr,
                    p: $ty,
                }
            },
            |ctx, pat| {
                let op = $f(ctx, pat.l, pat.r);
                ctx.union(pat.p, op);
            },
        );
    };
}

macro_rules! assoc {
    ($ty:ident,$f:expr,$pat_name:ident,$ruleset:ident) => {
        T::add_rule(
            stringify!($pat_name),
            $ruleset,
            || {
                let ll = Expr::query_leaf();
                let lr = Expr::query_leaf();
                let l = $ty::query(&ll, &lr);
                let r = Expr::query_leaf();
                let p = $ty::query(&l, &r);
                #[eggplant::pat_vars_catch]
                struct $pat_name {
                    ll: Expr,
                    lr: Expr,
                    r: Expr,
                    p: $ty,
                }
            },
            |ctx, pat| {
                let r = $f(ctx, pat.lr, pat.r);
                let p = $f(ctx, pat.ll, r);
                ctx.union(pat.p, p);
            },
        );
    };
}
pub fn assoc<T: G>(ruleset: RuleSetId) {
    assoc!(MAdd, RuleCtx::insert_m_add, AddAssocPat, ruleset);
    assoc!(MMul, RuleCtx::insert_m_mul, MulAssocPat, ruleset);
}

pub fn commu<T: G>(ruleset: RuleSetId) {
    commu!(MAdd, RuleCtx::insert_m_add, AddCommuPat, ruleset);
    commu!(MMul, RuleCtx::insert_m_mul, MulCommuPat, ruleset);
}

pub fn const_fold<T: G>(ruleset: RuleSetId) {
    use std::cmp::*;
    use std::ops::*;

    fold!(MAdd, Add::add, AddPat, ruleset);
    fold!(MSub, Sub::sub, SubPat, ruleset);
    fold!(MMul, Mul::mul, MulPat, ruleset);
    fold!(MMax, max, MaxPat, ruleset);
    fold!(MMin, min, MinPat, ruleset);
    fold!(MAnd, BitAnd::bitand, BitAndPat, ruleset);
}
