use std::sync::LazyLock;

use eggplant::prelude::*;
use eggplant::tx_rx_vt_pr_pf;
use eggplant::wrap::G;
use eggplant::wrap::Rx;

const Z: LazyLock<MVar<MyTx>> = LazyLock::new(|| MVar::<MyTx>::new("z".to_string()));

#[eggplant::ty]
pub enum Math {
    MNum { num: i64 },
    MVar { s: String },
    MAdd { l: Math, r: Math },
    MSub { l: Math, r: Math },
    MMul { l: Math, r: Math },
    MDiv { l: Math, r: Math },
    MMod { l: Math, r: Math },
    MMin { l: Math, r: Math },
    MMax { l: Math, r: Math },
    MAnd { l: Math, r: Math },
    MOr { l: Math, r: Math },
    MGte { l: Math, r: Math },
    MLt { l: Math, r: Math },
    MFloorTo { l: Math, r: Math },
    MReplace { l: Math, r: Math, s: Math },
    MAccum {},
}
#[eggplant::ty]
pub enum LoopType {
    Loop { s: String, range: Math },
}
#[eggplant::ty]
pub enum Expr {
    GMem {
        label: String,
    },
    LIn {
        expr: Expr,
        arr_and_range: LoopType,
        stride: Math,
    },
    LOut {
        expr: Expr,
        arr_and_range: LoopType,
        stride: Math,
    },
    SMem {},
    SMemLoad {
        expr1: Expr,
        expr2: Expr,
    },
    SMemRead {
        expr1: Expr,
        expr2: Expr,
    },

    /// unary ops
    Exp2 {
        base: Expr,
    },
    Log2 {
        expr: Expr,
    },
    Sqrt {
        expr: Expr,
    },
    Sin {
        expr: Expr,
    },
    Recip {
        expr: Expr,
    },
    Neg {
        expr: Expr,
    },
    /// binary ops
    Add {
        l: Expr,
        r: Expr,
    },
    Mul {
        l: Expr,
        r: Expr,
    },
    Max {
        l: Expr,
        r: Expr,
    },
    /// search helpers
    Unary {
        op: String,
        expr: Expr,
    },
    Binary {
        op: String,
        l: Expr,
        r: Expr,
    },
    SwapLoops {
        expr: Expr,
        a: String,
        b: String,
    },
    TileLoop {
        expr: Expr,
        name: String,
    },
    UnpadLoop {
        expr: Expr,
        name: String,
    },
}
tx_rx_vt_pr_pf!(MyTx, MyPatRec);
macro_rules! prop {
    ($ty:ident,$op:tt,$pat_name:ident,$ruleset:ident) => {
        #[eggplant::pat_vars]
        struct $pat_name {
            l: MNum,
            r: MNum,
            p: $ty,
        }
        MyTx::add_rule(
            stringify!($pat_name),
            $ruleset,
            || {
                let l = MNum::query();
                let r = MNum::query();
                let p = $ty::query(&l, &r);
                $pat_name::new(l, r, p)
            },
            |ctx, values| {
                let cal = ctx.devalue(values.l.num) $op ctx.devalue(values.r.num);
                let op_value = ctx.insert_m_num(cal);
                ctx.union(values.p, op_value);
            },
        );
    };
}
fn main() {
    // env_logger::init();
    // let expr: Math<MyTx, _> = MAdd::new(&MMul::new(&MNum::new(3), &MNum::new(2)), &MNum::new(4));
    // expr.commit();

    let ruleset = MyTx::new_ruleset("constant_prop");
    prop!(MAdd,+,AddPat,ruleset);
    prop!(MSub,-,SubPat,ruleset);
    prop!(MMul,*,MulPat,ruleset);
    // prop!(MDiv,/,DivPat,ruleset);

    // compute max in arr
    let tensor_arr: Expr<MyTx, _> = GMem::new("Arr [1,5,2,9]".to_string());
    let acc_init: Expr<MyTx, _> = GMem::new("acc".to_string());
    let arr_loop = LoopType::new("i".to_string(), &MNum::new(4));
    let col1 = MVar::new("z".to_string());
    let arr_loop_in = LIn::new(&tensor_arr, &arr_loop, &col1);
    let acc_loop_in = LIn::new(&acc_init, &arr_loop, &MAccum::new());
    let max = Max::new(&arr_loop_in, &acc_loop_in);
    let max_arr: Expr<MyTx, LOutTy> = LOut::new(
        &max,
        &Loop::new("fold".to_string(), &MNum::new(4)),
        &MAccum::new(),
    );
    max_arr.commit();

    let report = MyTx::run_ruleset(ruleset, RunConfig::Sat);
    println!("{:#?}", report);
    MyTx::egraph_to_dot("egraph.dot".into());

    let val = MyTx::value(&max);
    MyTx::sgl().on_pull_value(val);
    MyTx::wag_to_dot("wag.dot".into());
}

#[test]
fn fusion_test() {
    let tensor_a: Expr<MyTx, GMemTy> = GMem::new("A".to_string());
    let tensor_a = lout_unary_lin(&tensor_a, 10, &Z, "add", "a", "a");
    let tensor_a = lout_unary_lin(&tensor_a, 10, &Z, "add", "a", "a");
}
/// create things like
/// lout(unary(lin(tensor, range, stride))
fn lout_unary_lin<T: G>(
    tensor: &Expr<T>,
    range: i64,
    stride: &Math<T>,
    unary_op: &str,
    lin_label: &str,
    lout_label: &str,
) -> LOut<T> {
    let lin = lin(tensor, lin_label, range, &(var("z") * 1));
    let unary = unary(unary_op, &lin);
    let lout = lout(tensor, lout_label, range, stride);
    lout
}
fn lin<T: G>(tensor: &Expr<T>, lin_label: &str, range: i64, stride: &Math<T>) -> LIn<T> {
    let lin = LIn::new(
        tensor,
        &Loop::new(lin_label.to_string(), &MNum::new(range)),
        stride,
    );
    lin
}
fn lout<T: G>(tensor: &Expr<T>, lout_label: &str, range: i64, stride: &Math<T>) -> LOut<T> {
    let lout = LOut::new(
        tensor,
        &Loop::new(lout_label.to_string(), &MNum::new(range)),
        stride,
    );
    lout
}
fn unary<T: G>(op: &str, lin: &Expr<T>) -> Unary<T> {
    Unary::new(op.to_string(), lin)
}
fn var<T: G>(name: &str) -> MVar<T> {
    MVar::new(name.to_string())
}
impl<T: G> std::ops::Mul<i64> for MVar<T> {
    type Output = MMul<T>;
    fn mul(self, rhs: i64) -> Self::Output {
        MMul::new(&self, &MNum::new(rhs))
    }
}
