// flatten function
// This function takes a tuple like this:
// ((1, 2), 3) and flattens it to (1, 2, 3)
// not every tuple is supported, only tuple where every tuple is 2 elements long and the last element is not a tuple
// So this is supported:
// ((((1, 2), 3), 4), 5)
// But this is not:
// (((1, 2), 3), 4, 5) or (1, 2, 3, (4, 5))

// as rust doesn't support variadic generics yet, we have to use a macro to generate the impl for every tuple size

use crate::{fst::{Space, Expression, Statement, Fst}, parser::lexer::LocatedToken};

pub fn flatten<T: Flatable>(tuple: T) -> T::Flattened {
    tuple.flatten()
}

pub trait BaseElement {}
trait FlatTuple {}

pub trait Flatable {
    type Flattened: FlatTuple;
    fn flatten(self) -> Self::Flattened;
}

// Nest is used to turn (A, B, C, D) into (((A, B), C), D)
macro_rules! generate_nest {
    ($type:ident, $name:ident) => {
        macro_rules! $name {
            ($a:$type, $b:$type) => {
                ($a, $b)
            };
            ($a:$type, $b:$type, $$($rest:$type),+ $$(,)?) => {
                $name!(($a, $b), $$($rest),+)
            };
            //handle single element
            ($a:$type) => {
                ($a,)
            };
            //handle empty tuple
            () => {
                ()
            };
        }
    }
}
generate_nest!(ty, nest_type);
generate_nest!(pat, nest_pat);

// tuple is used to ensure that things such as (A) are not turned to A
macro_rules! generate_tuple {
    ($type:ident, $name:ident) => {
        macro_rules! $name {
            ($a:$type, $$($rest:$type),+ $$(,)?) => {
                ($a, $$($rest),+)
            };
            //handle single element
            ($a:$type) => {
                ($a,)
            };
            //handle empty tuple
            () => {
                ()
            };
        }
    }
}
generate_tuple!(ty, type_tuple);
generate_tuple!(expr, expr_tuple);

// max tuple size is 12

// example of generated code:

// impl<A, B, C> Flatable for ((A, B), C)
// where
//     A: Debug + Clone,
//     B: Debug + Clone,
//     C: Debug + Clone,
// {
//     type Flattened = (A, B, C);
//     fn flatten(self) -> Self::Flattened {
//         let ((a, b), c) = self;
//         (a, b, c)
//     }
// }

macro_rules! impl_flatten {
    ($($type:ident $var:ident),+) => {
        impl<$($type),+> Flatable for nest_type!($($type),+)
        where
            $($type: BaseElement),+
        {
            type Flattened = type_tuple!($($type),+);
            fn flatten(self) -> Self::Flattened {
                let nest_pat!($($var),+) = self;
                expr_tuple!($($var),+)
            }
        }
        impl<$($type),+> FlatTuple for type_tuple!($($type),+) where $($type: BaseElement),+ {}
    };
}

macro_rules! generate_flatten {
    ($type:ident $var:ident) => {
        impl_flatten!($type $var);
    };
    ($type:ident $var:ident, $($rest:ident $rest_var:ident),+) => {
        impl_flatten!($type $var, $($rest $rest_var),+);
        generate_flatten!($($rest $rest_var),+);
    };
}

generate_flatten!(A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k, L l, M m, N n, O o, P p, Q q, R r, S s, T t, U u, V v, W w, X x, Y y, Z z);

impl Flatable for () {
    type Flattened = ();
    fn flatten(self) -> Self::Flattened {
        ()
    }
}
impl FlatTuple for () {}

macro_rules! impl_base_element {
    ($($type:ty),+) => {
        $(
            impl BaseElement for $type {}
        )+
    };
}

impl<'a> BaseElement for LocatedToken<'a> {}
impl<T: BaseElement> BaseElement for Option<T> {}
impl<T: BaseElement> BaseElement for Vec<T> {}
impl_base_element!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, bool, char, String,
    str, Space, Expression, Statement, Fst
);

#[cfg(test)]
mod tests {
    generate_nest!(expr, nest_expr);

    #[test]
    fn test_nest() {
        let tuple = nest_expr!(1);
        assert_eq!(tuple, (1,));
        let tuple = nest_expr!(1, 2);
        assert_eq!(tuple, (1, 2));
        let tuple = nest_expr!(1, 2, 3);
        assert_eq!(tuple, ((1, 2), 3));
        let tuple = nest_expr!(1, 2, 3, 4);
        assert_eq!(tuple, (((1, 2), 3), 4));
    }
    #[test]
    fn test_tuple() {
        let tuple = expr_tuple!(1);
        assert_eq!(tuple, (1,));
        let tuple = expr_tuple!(1, 2);
        assert_eq!(tuple, (1, 2));
        let tuple = expr_tuple!(1, 2, 3);
        assert_eq!(tuple, (1, 2, 3));
    }
    #[test]
    fn test_flatten() {
        let tuple = nest_expr!();
        let flattened = super::flatten(tuple);
        assert_eq!(flattened, ());
        let tuple = nest_expr!(1);
        let flattened = super::flatten(tuple);
        assert_eq!(flattened, (1,));
        let tuple = nest_expr!(1, 2);
        let flattened = super::flatten(tuple);
        assert_eq!(flattened, (1, 2));
        let tuple = nest_expr!(1, 2, 3);
        let flattened = super::flatten(tuple);
        assert_eq!(flattened, (1, 2, 3));
        let tuple = nest_expr!(1, 2, 3, 4);
        let flattened = super::flatten(tuple);
        assert_eq!(flattened, (1, 2, 3, 4));
    }
}
