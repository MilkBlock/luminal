use crate::datatypes::*;
use eggplant::wrap::G;

macro_rules! cartesian_ops {
    (($t1:ty, $t2:ty),$out:tt,$op:tt,$method:ident) => {
        impl<T: G> std::ops::$op<$t2> for $t1 {
            type Output = $out<T>;

            fn $method(self, rhs: $t2) -> Self::Output {
                $out::new(&self, &rhs)
            }
        }
    };
    (($t1:ty, $t2:ty, $($rest:ty),+),$out:tt,$op:tt,$method:ident) => {
        cartesian_ops!(($t1, $t2),$out,$op,$method);
        cartesian_ops!(($t2, $t1),$out,$op,$method);
        cartesian_ops!(($t1, $t1),$out,$op,$method);
        $( cartesian_ops!(($t1, $rest),$out,$op,$method);)*
        $( cartesian_ops!(($rest, $t1),$out,$op,$method);)*
        cartesian_ops!(($t2, $($rest),*),$out,$op,$method);
    };
}
cartesian_ops!(
    (
        MNum<T>,
        MVar<T>,
        MAdd<T>,
        MSub<T>,
        MMul<T>,
        MDiv<T>,
        MMod<T>,
        MMin<T>,
        MMax<T>,
        MAnd<T>,
        MOr<T>,
        MGte<T>,
        MLt<T>,
        MFloorTo<T>,
        MReplace<T>,
        MAccum<T>
    ),
    MAdd,
    Add,
    add
);
cartesian_ops!(
    (
        MNum<T>,
        MVar<T>,
        MAdd<T>,
        MSub<T>,
        MMul<T>,
        MDiv<T>,
        MMod<T>,
        MMin<T>,
        MMax<T>,
        MAnd<T>,
        MOr<T>,
        MGte<T>,
        MLt<T>,
        MFloorTo<T>,
        MReplace<T>,
        MAccum<T>
    ),
    MMul,
    Mul,
    mul
);

cartesian_ops!(
    (
        MNum<T>,
        MVar<T>,
        MAdd<T>,
        MSub<T>,
        MMul<T>,
        MDiv<T>,
        MMod<T>,
        MMin<T>,
        MMax<T>,
        MAnd<T>,
        MOr<T>,
        MGte<T>,
        MLt<T>,
        MFloorTo<T>,
        MReplace<T>,
        MAccum<T>
    ),
    MDiv,
    Div,
    div
);

cartesian_ops!(
    (
        MNum<T>,
        MVar<T>,
        MAdd<T>,
        MSub<T>,
        MMul<T>,
        MDiv<T>,
        MMod<T>,
        MMin<T>,
        MMax<T>,
        MAnd<T>,
        MOr<T>,
        MGte<T>,
        MLt<T>,
        MFloorTo<T>,
        MReplace<T>,
        MAccum<T>
    ),
    MSub,
    Sub,
    sub
);

cartesian_ops!(
    (
        MNum<T>,
        MVar<T>,
        MAdd<T>,
        MSub<T>,
        MMul<T>,
        MDiv<T>,
        MMod<T>,
        MMin<T>,
        MMax<T>,
        MAnd<T>,
        MOr<T>,
        MGte<T>,
        MLt<T>,
        MFloorTo<T>,
        MReplace<T>,
        MAccum<T>
    ),
    MMod,
    Rem,
    rem
);
