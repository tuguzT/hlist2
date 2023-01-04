//! Compatibility of heterogenous lists for tuples.
//!
//! This module implements conversion between heterogenous lists and their tuple forms
//! when tuple has length of 12 and less, and vise versa.
//!
//! This is done by implementing a [`From`] trait from the Rust [`core`] library.
//! This allows to implement [`Into`] trait automatically.
//!
//! For example, we can easily convert a tuple into heterogenous list
//! where arrangement of its elements is the same:
//!
//! ```
//! use hlist2::hlist;
//!
//! let list = hlist!(1, 2.0, true, "hello world");
//! let tuple: (_, _, _, _) = list.into();
//! assert_eq!(tuple, (1, 2.0, true, "hello world"));
//! ```
//!
//! Also this module defines 2 new traits, [`IntoTuple`] and [`IntoHList`]
//! to convert heterogenous list into tuple and vise versa using associated types instead of generics.
//!
//! For example, it is easily to convert heterogenous list into tuple
//! with [`into_tuple`][IntoTuple::into_tuple()] method instead of the previous example,
//! as there is no need to explicitly provide tuple type:
//!
//! ```
//! use hlist2::{hlist, tuple::IntoTuple};
//!
//! let list = hlist!(1, 2.0, true, "hello world");
//! let tuple = list.into_tuple();
//! assert_eq!(tuple, (1, 2.0, true, "hello world"));
//! ```

use crate::HList;

/// Convert the heterogenous list into tuple.
pub trait IntoTuple {
    /// Type of tuple the heterogenous list will be converted to.
    type Output;

    /// Converts the heterogenous list into tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, tuple::IntoTuple};
    ///
    /// let list = hlist!(1, 2.0, true, "hello world");
    /// assert_eq!(list.into_tuple(), (1, 2.0, true, "hello world"));
    /// ```
    fn into_tuple(self) -> Self::Output;
}

/// Convert the tuple into heterogenous list.
pub trait IntoHList {
    /// Type of heterogenous list the tuple will be converted to.
    type Output: HList;

    /// Converts the tuple into heterogenous list.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, tuple::IntoHList};
    ///
    /// let tuple = (1, 2.0, true, "hello world");
    /// assert_eq!(tuple.into_hlist(), hlist!(1, 2.0, true, "hello world"));
    /// ```
    fn into_hlist(self) -> Self::Output;
}

impl<L> IntoHList for L
where
    L: HList,
{
    type Output = Self;

    fn into_hlist(self) -> Self::Output {
        self
    }
}

macro_rules! hlist_from_tuple {
    ($($types:ident),*) => {
        impl<$($types),*> From<($($types,)*)> for $crate::HList!($($types,)*) {
            #[allow(non_snake_case)]
            fn from(value: ($($types,)*)) -> Self {
                let ($($types,)*) = value;
                $crate::hlist!($($types,)*)
            }
        }

        impl<$($types),*> IntoTuple for $crate::HList!($($types,)*) {
            type Output = ($($types,)*);

            fn into_tuple(self) -> Self::Output {
                self.into()
            }
        }

        impl<$($types),*> IntoTuple for ($($types,)*) {
            type Output = Self;

            fn into_tuple(self) -> Self::Output {
                self
            }
        }
    };
}

// Conversion from tuple to heterogenous list is implemented for tuples of size 12 and less
hlist_from_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
hlist_from_tuple!(A, B, C, D, E, F, G, H, I, J, K);
hlist_from_tuple!(A, B, C, D, E, F, G, H, I, J);
hlist_from_tuple!(A, B, C, D, E, F, G, H, I);
hlist_from_tuple!(A, B, C, D, E, F, G, H);
hlist_from_tuple!(A, B, C, D, E, F, G);
hlist_from_tuple!(A, B, C, D, E, F);
hlist_from_tuple!(A, B, C, D, E);
hlist_from_tuple!(A, B, C, D);
hlist_from_tuple!(A, B, C);
hlist_from_tuple!(A, B);
hlist_from_tuple!(A);
hlist_from_tuple!();

macro_rules! tuple_from_hlist {
    ($($types:ident),*) => {
        impl<$($types),*> From<$crate::HList!($($types,)*)> for ($($types,)*) {
            #[allow(non_snake_case, clippy::unused_unit)]
            fn from(value: $crate::HList!($($types,)*)) -> Self {
                let $crate::hlist!($($types,)*) = value;
                ($($types,)*)
            }
        }

        impl<$($types),*> IntoHList for ($($types,)*) {
            type Output = $crate::HList!($($types,)*);

            fn into_hlist(self) -> Self::Output {
                self.into()
            }
        }
    };
}

// Conversion from heterogenous list to tuple is implemented for tuples of size 12 and less
tuple_from_hlist!(A, B, C, D, E, F, G, H, I, J, K, L);
tuple_from_hlist!(A, B, C, D, E, F, G, H, I, J, K);
tuple_from_hlist!(A, B, C, D, E, F, G, H, I, J);
tuple_from_hlist!(A, B, C, D, E, F, G, H, I);
tuple_from_hlist!(A, B, C, D, E, F, G, H);
tuple_from_hlist!(A, B, C, D, E, F, G);
tuple_from_hlist!(A, B, C, D, E, F);
tuple_from_hlist!(A, B, C, D, E);
tuple_from_hlist!(A, B, C, D);
tuple_from_hlist!(A, B, C);
tuple_from_hlist!(A, B);
tuple_from_hlist!(A);
tuple_from_hlist!();
