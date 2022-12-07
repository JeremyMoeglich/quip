// flatten function
// This function takes a tuple like this:
// ((1, 2), 3) and flattens it to (1, 2, 3)
// not every tuple is supported, only tuple where every tuple is 2 elements long and the last element is not a tuple
// So this is supported:
// ((((1, 2), 3), 4), 5)
// But this is not:
// (((1, 2), 3), 4, 5) or (1, 2, 3, (4, 5))

// as rust doesn't support variadic generics yet, we have to use a macro to generate the function for every tuple size

use std::fmt::Debug;

pub fn flatten<T: Flatable>(tuple: T) -> T::Flattened {
    tuple.flatten()
}

pub trait Flatable {
    type Flattened: Debug + Clone;
    fn flatten(self) -> Self::Flattened;
}

// The first macro is used to turn (A, B, C, D) into (((A, B), C), D)

macro_rules! nest {
    ($a:expr, $b:expr) => {
        ($a, $b)
    };
    ($a:expr, $b:expr, $($rest:expr),+) => {
        nest!(($a, $b), $($rest),+)
    };
    //handle trailing comma
    ($a:expr, $b:expr, $($rest:expr),+,) => {
        nest!(($a, $b), $($rest),+)
    };
}

// makes a type to a variable name A -> a
macro_rules! to_var {
    ($ident:ident) => {
        stringify!($ident).to_lowercase()
    };
}

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
    ($($ident:ident),+) => {
        impl<$($ident),+> Flatable for nest!($($ident),+)
        where
            $($ident: Debug + Clone),+
        {
            type Flattened = ($($ident),+);
            fn flatten(self) -> Self::Flattened {
                let nest!($($ident),+) = self;
                nest!($($ident),+)
            }
        }
    };
}

// impl_flatten!(A);
// impl_flatten!(A, B);
// impl_flatten!(A, B, C);
// impl_flatten!(A, B, C, D);
impl_flatten!(A, B, C, D, E);
// impl_flatten!(A, B, C, D, E, F);
// impl_flatten!(A, B, C, D, E, F, G);
// impl_flatten!(A, B, C, D, E, F, G, H);
// impl_flatten!(A, B, C, D, E, F, G, H, I);
// impl_flatten!(A, B, C, D, E, F, G, H, I, J);
// impl_flatten!(A, B, C, D, E, F, G, H, I, J, K);
// impl_flatten!(A, B, C, D, E, F, G, H, I, J, K, L);

const test = nest!(A, B, C, D, E);

#[cfg(test)]
mod tests {
    #[test]
    fn test_nest() {
        let tuple = nest!(1, 2, 3, 4, 5, );
        assert_eq!(tuple, ((((1, 2), 3), 4), 5));
    }
    #[test]
    fn test_flatten() {
        let tuple = nest!(1, 2, 3, 4, 5, );
        let flattened = super::flatten(tuple);
        assert_eq!(flattened, (1, 2, 3, 4, 5));
    }
}
