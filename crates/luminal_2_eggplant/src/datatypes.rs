use eggplant::egglog;
use eggplant::prelude::*;
use serde::Deserialize;
use serde::Serialize;

// datatype((datatype Expression(MNum i64:args_name "num")(MVar String:args_name "name")(MAdd Expression Expression:args_name "l,r")(MSub Expression Expression:args_name "l,r")(MMul Expression Expression:args_name "l,r")(MDiv Expression Expression:args_name "l,r")(MMod Expression Expression:args_name "l,r")(MMin Expression Expression:args_name "l,r")(MMax Expression Expression:args_name "l,r")(MAnd Expression Expression:args_name "l,r")(MOr Expression Expression:args_name "l,r")(MGte Expression Expression:args_name "l,r")(MLt Expression Expression:args_name "l,r")(MFloorTo Expression Expression:args_name "l,r")(MReplace Expression Expression Expression:args_name "l,r,rpl")(MAccum String:args_name "name")))#[doc = "DSl Generated"]
#[eggplant::dsl]
pub enum Expr {
    MNum { num: i64 },
    MVar { name: String },
    MAdd { l: Expr, r: Expr },
    MSub { l: Expr, r: Expr },
    MMul { l: Expr, r: Expr },
    MDiv { l: Expr, r: Expr },
    MMod { l: Expr, r: Expr },
    MMin { l: Expr, r: Expr },
    MMax { l: Expr, r: Expr },
    MAnd { l: Expr, r: Expr },
    MOr { l: Expr, r: Expr },
    MGte { l: Expr, r: Expr },
    MLt { l: Expr, r: Expr },
    MFloorTo { l: Expr, r: Expr },
    MReplace { l: Expr, r: Expr, rpl: Expr },
    MAccum { name: String },
}

#[eggplant::base_ty]
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum UnOp {
    Exp2,
    Log2,
    Sqrt,
    Sin,
    Recip,
    Neg,
    #[default]
    Unknown,
}
#[eggplant::base_ty]
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    #[default]
    Unknown,
}

#[eggplant::dsl(base=BinOp,base=UnOp)]
enum IR {
    GMEM {
        name: String,
    },
    LoopIn {
        ir: IR,
        l: Expr,
        r: Expr,
    },
    LoopOut {
        ir: IR,
        l: Expr,
        r: Expr,
    },
    Unary {
        op: UnOp,
        ir: IR,
    },
    Binary {
        op: BinOp,
        l: IR,
        r: IR,
    },
    SwapLoops {
        ir: IR,
        level: i64,
    },
    TileLoop {
        ir: IR,
        level: i64,
    },
    MergeLoops {
        ir: IR,
        level: i64,
    },
    TCMatmul {
        inp_a: IR,
        inp_b: IR,
        a_k_stride: Expr,
        b_k_stride: Expr,
        a_inner_stride: Expr,
        b_inner_stride: Expr,
        c_inner_stride: Expr,
        num_k_loops: Expr,
    },
    TiledMatmulInputA {
        ir: IR,
        num: i64,
        expr: Expr,
    },
    TiledMatmulInputB {
        ir: IR,
        num: i64,
        expr: Expr,
    },
}
