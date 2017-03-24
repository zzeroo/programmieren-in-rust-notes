// http://jadpole.github.io/rust/typechecked-matrix
//

pub trait Num {
    fn val() -> usize;
}

macro_rules! nums {
    ( $( $name:ident => $num:expr ),* ) => {
        $(
            #[derive(Clone, Copy, Eq, PartialEq)]
            pub struct $name;
            impl Num for $name {
                fn val() -> usize { $num }
            }
        )*
    }
}

nums! {
    N1 => 1,
    N2 => 2,
    N3 => 3,
    N4 => 4,
    N5 => 5,
    N6 => 6
}
